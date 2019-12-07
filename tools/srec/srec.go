package main

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"io"
	"log"
	"os"
)

func srec(r io.Reader) ([]byte, error) {
	var b = bytes.NewBufferString("/* http://srecord.sourceforge.net/ */\n")
	var off uint32
	for {
		var dat [7]uint32
		err := binary.Read(os.Stdin, binary.LittleEndian, dat[:])
		if err == io.EOF {
			break
		}
		if err != nil && err != io.ErrUnexpectedEOF {
			return nil, err
		}

		if n, err := fmt.Fprintf(b, "@%08X %08X %08X %08X %08X %08X %08X %08X\n", off, dat[0], dat[1], dat[2], dat[3], dat[4], dat[5], dat[6]); err != nil || n != 73 {
			return nil, fmt.Errorf("Short write %d not 73 or %v", n, err)

		}
		off += 7

	}
	return b.Bytes(), nil
}

func main() {
	b, err := srec(os.Stdin)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%s", string(b))
}
