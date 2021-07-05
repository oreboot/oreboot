package main

import (
	"flag"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

var (
	in  = flag.String("in", "trembyle.bin", "in file")
	out = flag.String("out", "out.bin", "out file")
)

func main() {
	flag.Parse()
	b, err := ioutil.ReadFile(*in)
	if err != nil {
		log.Fatal(err)
	}
	a := flag.Args()
	if len(a) == 0 {
		// for oreboot
		a = append(a, "target/x86_64-unknown-none/release/image.bin@C00000@3f0000")
		a = append(a, "target/x86_64-unknown-none/release/bootblob.bin@Fe0000@1EF00")
		a = append(a, "start.bin@FF0000")
	}
	for _, v := range a {
		parms := strings.Split(v, "@")
		if len(parms) < 2 || len(parms) > 3 {
			log.Fatalf("malformed arg: must be file@off[@size]: %v", v)
		}
		f := parms[0]
		o, err := strconv.ParseUint(parms[1], 16, 24)
		if err != nil {
			log.Fatal(err)
		}
		off := int(o)
		patch, err := ioutil.ReadFile(f)
		if err != nil {
			log.Fatal(err)
		}
		if len(patch) > 0x800000 {
			log.Fatalf("Patch is more than 8m -- please fix")
		}
		if off > len(b) {
			log.Fatalf("Off %d is > len of file %d", off, len(b))
		}
		plen := len(patch)
		if len(parms) == 3 {
			l, err := strconv.ParseUint(parms[2], 16, 24)
			if err != nil {
				log.Fatal(err)
			}
			plen = int(l)
		}
		if (off + plen) > len(b) {
			log.Fatalf("Off %#08x  + len %#08x is %#08x > len of file %#08x", off, plen, off+plen, len(b))
		}
		if plen > len(patch) {
			log.Printf("(warning)Patch will only be %#x bytes, not %#x bytes\n", len(patch), plen)
			plen = len(patch)
		}
		log.Printf("Patch %v at %#x for %#x bytes", f, off, plen)
		copy(b[off:], patch[:plen])
	}
	/*
	 * We need to patch the PSP directory entries. This is what we have:
	 * At 0x1680b8 in the original ROM starts a PSP directory entry for the BIOS.
	 * The other 8 bytes here before are the remainder of the previous entry.
	 * Note that `type` here actually contains more. See below for more details.
	 *
	 * 001680b0: 0000  200a  0000  0000   6200  0100  0000  3000  .. .....b.....0.
	 * 001680c0: 0000  d0ff  0000  0000   0000  d009  0000  0000  ................
	 *           |---- remainder -----|   |- type -|  |- size -|
	 *           |- location in ROM --|   |- RAM destination --|
	 *
	 * Above is an excerpt from one of the PSP directories. The entries are 24
	 * bytes each. The first byte indicates the type, 0x62 here, which means that
	 * it represents a "BIOS" entry. The third byte contains flags. We need to set
	 * the lowest two to tell the PSP that the entry is a reset image and should
	 * be copied to RAM. In other words: then we can execute from RAM directly. :)
	 * Bytes 4-7 describe the size in bytes. Ours is 4MB fixed (for now).
	 * Bytes 8-15 describe the position in the SPI flash. We use the last 4MB.
	 * The two highest bytes need to be set to `f` because of some mapping.
	 * Bytes 16-23 finally set the address in RAM to copy to. Note that setting
	 * this without the flag in the third byte does not suffice.
	 *
	 * So our goal is to have this entry (all in a single row):
	 * 6200  0300  0000  4000   0000  c0ff  0000  0000   0000  c076  0000  0000
	 *
	 * The full struct is documented in the coreboot docs:
	 * https://doc.coreboot.org/soc/amd/psp_integration.html#bios-directory-table-entries
	 */
	// or do we? This is a coreboot box and coreboot starts in rom, and jumps
	// to ffff0000
	if false {
		entry := []byte{
			0x62, 0x00, // type
			0x03, 0x00, // flags
			0x00, 0x00, 0x40, 0x00, // size
			0x00, 0x00, 0xc0, 0xff, 0x00, 0x00, 0x00, 0x00, // ROM position
			0x00, 0x00, 0xc0, 0x76, 0x00, 0x00, 0x00, 0x00, // RAM destination
		}

		// We just patch the individual entries here.
		copy(b[0x1680b8:], entry)

		// There are three more entries to patch, and they are all the same. This is
		// for different versions of the PSP firmware to pick them up.

		// 0063f080: 0000 200a 0000 0000 6200 0100 0000 3000  .. .....b.....0.
		// 0063f090: 0000 d0ff 0000 0000 0000 d009 0000 0000  ................
		copy(b[0x63f088:], entry)

		// 00268080: 0000 200a 0000 0000 6200 0100 0000 3000  .. .....b.....0.
		// 00268090: 0000 d0ff 0000 0000 0000 d009 0000 0000  ................
		copy(b[0x268088:], entry)

		// 004d00b0: 0000 200a 0000 0000 6200 0100 0000 3000  .. .....b.....0.
		// 004d00c0: 0000 d0ff 0000 0000 0000 d009 0000 0000  ................
		copy(b[0x4d00b8:], entry)
	}
	if err := ioutil.WriteFile(*out, b, 0644); err != nil {
		log.Fatal(err)
	}
}
