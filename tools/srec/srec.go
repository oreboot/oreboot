package main

import (
	"bytes"
	"encoding/binary"
	"flag"
	"fmt"
	"io"
	"log"
	"os"
)

var offset = flag.Uint("o", 0, "default offset")

func srec(r io.Reader) ([]byte, error) {
	var b = bytes.NewBufferString("/* http://srecord.sourceforge.net/ */\n")
	for {
		var dat [1]uint32
		err := binary.Read(os.Stdin, binary.LittleEndian, dat[:])
		if err == io.EOF {
			break
		}
		if err != nil && err != io.ErrUnexpectedEOF {
			return nil, err
		}

		if _, err := fmt.Fprintf(b, "@%08X %08X\n", *offset, dat[0]); err != nil {
			return nil, fmt.Errorf("Write: %v", err)
		}
		*offset += 1

	}
	return b.Bytes(), nil
}

func main() {
	flag.Parse()
	b, err := srec(os.Stdin)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%s", string(b))
}
