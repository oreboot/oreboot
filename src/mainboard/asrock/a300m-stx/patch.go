package main

import (
	"flag"
	"io/ioutil"
	"log"
	"strconv"
	"strings"
)

var (
	in  = flag.String("in", "in.bin", "in file")
	out = flag.String("out", "out.bin", "out file")
)

func main() {
	b, err := ioutil.ReadFile(*in)
	if err != nil {
		log.Fatal(err)
	}
	flag.Parse()
	a := flag.Args()
	if len(a) == 0 {
		a = append(a, "serial.bin@FFFFF0")
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
