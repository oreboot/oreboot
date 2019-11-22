package main

import (
	"encoding/json"
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"os"
	"strconv"
	"strings"

	"github.com/hjson/hjson-go"
)

type pin struct {
	Name string `json:"name"`
	Desc string `json:"desc"`
}

type field struct {
	Bits string `json:"bits"`
	Name string `json:"name"`
	Desc string `json:"desc"`
}

type register struct {
	Name     string  `json:"name'`
	Desc     string  `json:"desc"`
	SWAccess string  `json:"swaccess"`
	HWAccess string  `json:"hwaccess"`
	Fields   []field `json:"fields"`
}

type otdev struct {
	Name             string     `json:"name"`
	Clock            string     `json:"clock_primary"`
	BusDevice        string     `json:"bus_device"`
	BusHost          string     `json:"bus_host"`
	AvailableInputs  []pin      `json:"available_input_list"`
	AvailableOutputs []pin      `json:"available_output_list"`
	Interrupts       []pin      `json:"interrupt_list"`
	Width            string     `json:"regwidth"`
	Registers        []register `json:"registers"`
}

func convert(r io.Reader) (*otdev, error) {
	b, err := ioutil.ReadAll(r)
	if err != nil {
		return nil, err
	}
	// We need to provide a variable where Hjson
	// can put the decoded data.
	var dat map[string]interface{}
	// Decode and a check for errors.
	if err := hjson.Unmarshal(b, &dat); err != nil {
		return nil, err
	}

	// convert to JSON
	if b, err = json.Marshal(dat); err != nil {
		return nil, err
	}

	// In order to use the values in the decoded map,
	// we'll need to cast them to their appropriate type.

	var o = &otdev{}
	if err := json.Unmarshal(b, &o); err != nil {
		return nil, err
	}
	return o, nil
}

func block(o *otdev) {
	regfield := "u32"
	fmt.Println("pub struct RegisterBlock {")
	for i, r := range o.Registers {
		mode := "ReadWrite"
		if r.SWAccess == "ro" {
			mode = "ReadOnly"
		}
		comma := ""
		if i < len(o.Registers)-1 {
			comma = ","
		}
		fmt.Printf("\t%v: %s<%s, %s::Register>%s /* %s */\n", strings.ToLower(r.Name), mode, regfield, r.Name, comma, r.Desc)
	}
	fmt.Println("}")
}

func fields(o *otdev) {
	fmt.Println("register_bitfields! {\n\tu32,\n")
	for i, r := range o.Registers {
		fmt.Printf("\t%s [\n", r.Name)
		for i, f := range r.Fields {
			bits := strings.Split(f.Bits, ":")
			b1, err := strconv.Atoi(bits[0])
			if err != nil {
				log.Printf("f %v bits %s err %v", f, f.Bits, err)
				continue
			}
			bl := 1
			if len(bits) == 2 {
				b2, err := strconv.Atoi(bits[0])
				if err != nil {
					log.Printf("f %v bits %s err %v", f, f.Bits, err)
					continue
				}
				bl = b1 - b2 + 1
			}

			comma := ""
			if i < len(r.Fields)-1 {
				comma = ","
			}
			if f.Name == "" {
				f.Name = "DATA"
			}
			fmt.Printf("\t\t%s OFFSET(%d) NUMBITS(%d) []%s/* %s */\n", f.Name, b1, bl, comma, f.Desc)
		}
		comma := ""
		if i < len(o.Registers)-1 {
			comma = ","
		}
		fmt.Printf("\t]%s\n", comma)
	}
	fmt.Println("}")
}

func main() {
	o, err := convert(os.Stdin)
	if err != nil {
		log.Fatal(err)
	}
	block(o)
	fields(o)
}
