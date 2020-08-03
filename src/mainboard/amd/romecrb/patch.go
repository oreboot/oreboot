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
		// works with simply outb jmp 1b. a = append(a, "loop.bin@0xFFEFBF")
		// fails if stack is used.a = append(a, "loop.bin@0xFFEFBF")
		// works and shows log 8 bits a = append(a, "loop.bin@0xFF_fa0f")
		// works and shows fa a = append(a, "looppostEIP15:8.bin@0xFF_fa0f")
		// shows ff a = append(a, "looppostEIP23:16.bin@0xFFfa0f")
		// works, shows 76!!! a = append(a, "looppostEIP31:24.bin@0xFFfa0f")
		a = append(a, "jmporeboot.bin@FFfa0f")
		 // GPF a = append(a, "jmpcoreboot.bin@FFefbf")
		a = append(a, "target/x86_64-unknown-none/release/bootblob.bin@FF0000@EF00")
		//a = append(a, "build/coreboot.rom@C00000@3f0000")
		//a = append(a, "target/x86_64-unknown-none/release/bootblob.bin@D00000@2f0000")
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
		log.Printf("Patch %v at %#x for %#x bytes", f, off, plen)
		copy(b[off:], patch[:plen])
	}
	if err := ioutil.WriteFile(*out, b, 0644); err != nil {
		log.Fatal(err)
	}
}
