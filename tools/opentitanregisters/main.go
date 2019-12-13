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

type param struct {
	Name string `json:"name"`
	Desc string `json:"desc"`
	Type string `json:"type"`
	Default string `json:"default"`
	Local string `json:"local"`
}

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
	Count    string  `json:"count"`
	SWAccess string  `json:"swaccess"`
	HWAccess string  `json:"hwaccess"`
	Fields   []field `json:"fields"`

	// When set, the fields in the register are repeated.
	MultiReg *register `json:"multireg"`
}

type otdev struct {
	Name             string     `json:"name"`
	Clock            string     `json:"clock_primary"`
	ParamList        []param    `json:"param_list"`
	BusDevice        string     `json:"bus_device"`
	BusHost          string     `json:"bus_host"`
	AvailableInputs  []pin      `json:"available_input_list"`
	AvailableOutputs []pin      `json:"available_output_list"`
	Interrupts       []pin      `json:"interrupt_list"`
	Width            string     `json:"regwidth"`
	Registers        []register `json:"registers"`
}

type bitSet struct {
	Offset, NumBits int
}

func ParseBitSet(s string) (*bitSet, error) {
	bits := strings.Split(s, ":")
	b1, err := strconv.Atoi(bits[0])
	if err != nil {
		return nil, fmt.Errorf("bits %s err %v", s, err)
	}
	bl := 1
	if len(bits) == 2 {
		b2, err := strconv.Atoi(bits[1])
		if err != nil {
			return nil, fmt.Errorf("bits %s err %v", s, err)
		}
		bl = b1 - b2 + 1
		b1 = b2
	}

	return &bitSet{
		Offset: b1,
		NumBits: bl,
	}, nil
}

func (b *bitSet) String() string {
	return fmt.Sprintf("%d:%d", b.Offset + b.NumBits - 1, b.Offset)
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

func paramLookup(o *otdev, param string) string {
	for _, p := range o.ParamList {
		if p.Name == param {
			return p.Default
		}
	}
	log.Fatalf("Could not find parameter %q", param)
	return ""
}

// Convert multireg registers into multiple regular registers.
func multiregister(o *otdev) {
	newRegs := []register{}
	for _, r := range o.Registers {
		if r.MultiReg == nil {
			newRegs = append(newRegs, r)
		} else {
			count, err := strconv.Atoi(paramLookup(o, r.MultiReg.Count))
			if err != nil {
				log.Fatalf("Count not convert param %q to int", r.MultiReg.Count)
			}

			newReg := *r.MultiReg
			newReg.Name = fmt.Sprintf("%s0", newReg.Name)
			newReg.Fields = []field{}

			numRegs := 1
			numBits := 0
			for j := 0; j < count; j++ {
				for _, f := range r.MultiReg.Fields {
					newField := f
					newField.Name = fmt.Sprintf("%s%d", newField.Name, j)

					bs, err := ParseBitSet(f.Bits)
					if err != nil {
						log.Fatal(err)
						continue
					}

					if numBits + bs.NumBits > 32 {
						newRegs = append(newRegs, newReg)
						newReg = *r.MultiReg
						newReg.Fields = []field{}
						newReg.Name = fmt.Sprintf("%s%d", newReg.Name, numRegs)

						numBits = 0
						numRegs++
					}
					bs.Offset = numBits
					numBits += bs.NumBits

					newField.Bits = bs.String()
				        newReg.Fields = append(newReg.Fields, newField)
				}
			}

			newRegs = append(newRegs, newReg)
		}
	}
	o.Registers = newRegs
}

func block(o *otdev) {
	fmt.Println("pub struct RegisterBlock {")
	for i, r := range o.Registers {
		reg(o, i, &r)
	}
	fmt.Println("}")
}

func reg(o *otdev, i int, r *register) {
	mode := "ReadWrite"
	if r.SWAccess == "ro" {
		mode = "ReadOnly"
	}
	comma := ""
	if i < len(o.Registers)-1 {
		comma = ","
	}
	fmt.Printf("\t%v: %s<%s, %s::Register>%s /* %s */\n", strings.ToLower(r.Name), mode, "u32", r.Name, comma, r.Desc)
}

func fields(o *otdev) {
	fmt.Println("register_bitfields! {\n\tu32,")
	for i, r := range o.Registers {
		fmt.Printf("\t%s [\n", r.Name)
		for i, f := range r.Fields {
			bs, err := ParseBitSet(f.Bits)
			if err != nil {
				log.Print(err)
				continue
			}

			comma := ""
			if i < len(r.Fields)-1 {
				comma = ","
			}
			if f.Name == "" {
				f.Name = "DATA"
			}
			fmt.Printf("\t\t%s OFFSET(%d) NUMBITS(%d) [", f.Name, bs.Offset, bs.NumBits)
			// Most fields are one bit. For now, if that is the case, emit ON and OFF values.
			if bs.NumBits == 1 {
				fmt.Printf("\n\t\t\tOFF = 0,\n\t\t\tON = 1\n\t\t")
			}
			fmt.Printf("]%s/* %s */\n", comma, f.Desc)
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
	multiregister(o)
	block(o)
	fields(o)
}
