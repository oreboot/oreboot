package main

import (
	"flag"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

var (
	in  = flag.String("in", "Ethanol-X_BIOS.16.bin", "in file")
	out = flag.String("out", "bios16.bin", "out file")
)

func main() {
	flag.Parse()
	b, err := ioutil.ReadFile(*in)
	if err != nil {
		log.Fatal(err)
	}
	a := flag.Args()
	if len(a) == 0 {
		// a = append(a, "jmporeboot.bin@FFefbf")
		a = append(a, "start.bin@FF0000")
		//a = append(a, "x.bin@FFEF72@108e")
		// for oreboot
		a = append(a, "target/x86_64-unknown-none/release/bootblob.bin@Fe0000@1EF00")
		a = append(a, "target/x86_64-unknown-none/release/image.bin@C00000@3f0000")
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
			log.Fatalf("Off %#08x  + len %#08x is %#08x > len of file %#08x", off, plen, off + plen, len(b))
		}
		if plen > len(patch) {
			log.Printf("(warning)Patch will only be %#x bytes, not %#x bytes\n", len(patch), plen)
			plen = len(patch)
		}
		log.Printf("Patch %v at %#x for %#x bytes", f, off, plen)
		copy(b[off:], patch[:plen])
	}
	// just patch the size here.
	// 0025601C   00 00 00 00  FF FF FF FF  FF FF FF FF  61 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 04  00 00 00 00  62 00 03 00  00 00 30 00  ............a.......................b.....0.
	// 00256048   00 00 D0 00  00 00 00 00  00 00 D0 76  00 00 00 00  64 00 10 01  90 49 00 00  00 90 25 00  00 00 00 00  FF FF FF FF  FF FF FF FF  65 00 10 01  ...........v....d....I....%.............e...
	b[0x256046] = 0x40
	b[0x25604a] = 0xc0
	b[0x256052] = 0xc0
	if err := ioutil.WriteFile(*out, b, 0644); err != nil {
		log.Fatal(err)
	}
}
