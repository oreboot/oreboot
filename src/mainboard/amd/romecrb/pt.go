package main

import (
	"fmt"
)

//#define PTLX(v, l)	(((v)>>(((l)*PTSHFT)+PGSHFT) & ((1<<PTSHFT)-1))
const (
	shft   = 9
	pgshft = 12
	msk    = (1 << shft) - 1
)

func ptlx(addr, level uint) uint {
	return (addr >> ((level * shft) + pgshft)) & msk
}

// how much address space a level covers.
func span(level uint) uint {
	//	#define PGLSZ(l)	(1<<(((l)*PTSHFT)+PGSHFT))
	return (1 << (((level) * shft) + pgshft))
}
func main() {
	var pml4 [1]uint64
	var pml3 [512]uint64
	// There are 4096 of these.
	//var pml2 [0x1000000 / 4096]uint64
	// that's fine -- 512G
	pml4[ptlx(0, 3)] = 0x2003
	// Each entry in the pml3 points to one ptp
	pml3[ptlx(0, 2)] = 0x83
	pml3[ptlx(0x40000000, 2)] = 0x40000083
	pml3[ptlx(0x80000000, 2)] = 0x80000083
	pml3[ptlx(0xc0000000, 2)] = 0xc0000083

	fmt.Printf("pml4:\n\t.long %#08x\n.align 0x1000\n", pml4[0])
	fmt.Printf("pml3:\n")
	for i := 0; i < len(pml3); i += 8 {
		fmt.Printf("\t.long ")
		for j := 0; j < 8; j++ {
			fmt.Printf("%#08x ", pml3[j+i])
		}
		fmt.Printf("\n")
	}
	// PML2
	fmt.Printf("pml2:\n")
	for i := 0x40000000; i < 0x80000000; i += 0x1000000 {
		fmt.Printf("\t.quad ")
		for j := 0; j < 8; j++ {
			fmt.Printf("%#08x", i + j * 0x200000 + 0x83)
			if j < 7 {
				fmt.Printf(",")
			}
		}
		fmt.Printf("\n")
	}

}
