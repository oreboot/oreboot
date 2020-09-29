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
	b, err := ioutil.ReadFile(*in)
	if err != nil {
		log.Fatal(err)
	}
	a := flag.Args()
	if len(a) == 0 {
		a = append(a, "jmporeboot.bin@FFefbf")
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
		if len(patch) > 0x400000 {
			patch = patch[len(patch)-0x400000:]
			log.Printf("Adjusted length to %#x", len(patch))
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
			log.Fatalf("Off %d  + len %d is > len of file %d", off, plen, len(b))
		}
		if plen > len(patch) {
			log.Printf("(warning)Patch will only be %#x bytes, not %#x bytes\n", len(patch), plen)
			plen = len(patch)
		}
		log.Printf("Patch %v at %#x for %#x bytes", f, off, plen)
		copy(b[off:], patch[:plen])
	}
	if err := ioutil.WriteFile(*out, b, 0644); err != nil {
		log.Fatal(err)
	}
}
