package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"strings"
)

var (
	macros = map[string]bool{}
	labels string
	enum   int
	b      = flag.Bool("b", true, "Wrap with function boilerplate")
)

func lines(s string) ([][]string, error) {
	b, err := ioutil.ReadFile(s)
	if err != nil {
		return nil, err
	}
	lines := strings.Split(string(b), "\n")
	var r [][]string
	for _, l := range lines {
		ll := []string{l}
		ll = append(ll, strings.FieldsFunc(l, func(r rune) bool {
			switch r {
			case ',', '\t', ' ':
				return true
			default:
				return false
			}
		})...)
		r = append(r, ll)
	}
	return r, nil
}

func fixImm(il []string) []string {
	rl := il[:2]
	for _, l := range il[2:] {
		if len(l) < 2 {
			rl = append(rl, l)
			continue
		}
		if l[:2] == "#'" {
			rl = append(rl, "'"+l[2:]+"'")
			continue
		}
		if l[:2] == "=#" {
			rl = append(rl, l[2:]+" as u32")
			continue
		}
		if l[0] == '#' {
			rl = append(rl, strings.TrimSuffix(l[1:], "]")+" as u32")
			continue
		}
		if l[0] == '=' {
			rl = append(rl, l[1:]+" as u32")
			continue
		}
		rl = append(rl, l)
	}
	return rl
}

func shifty(x int, l ...string) string {
	if len(l) < x+2 {
		return l[x]
	}
	s := l[x]
	switch l[x+1] {
	case "lsl":
		return fmt.Sprintf("(%s << %s)", s, l[x+2])
	case "lsr":
		return fmt.Sprintf("(%s >> %s)", s, l[x+2])
	}
	return s
}

func main() {
	flag.Parse()
	if flag.NArg() < 1 {
		log.Fatalf("Usage: %v name", os.Args[0])
	}

	i, err := lines(flag.Arg(0))
	if err != nil {
		log.Fatal(err)
	}
	if *b {
		fmt.Println(`#![no_std]
#![allow(non_snake_case)]
#![allow(unused_attributes)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(non_camel_case_types)]

pub mod ramtable;
#[macro_use]pub mod ram;
use core::ptr;
use core::fmt;
use crate::print;

// u-bmc modified
// Setting lifted from ast-g5-phy.h from OpenBMC u-boot
const CONFIG_DRAM_ECC_SIZE: u32 = 0x10000000;

/******************************************************************************
 r4 : return program counter
 r5 : DDR speed timing table base address
 Free registers:
 r0, r1, r2, r3, r6, r7, r8, r9, r10, r11
******************************************************************************/
const ASTMMC_INIT_VER: u32 = 0x12; //        @ 8bit verison
const ASTMMC_INIT_DATE: u32 = 0x20171027; //     @ Release

/******************************************************************************
 BMC side DDR IO driving manual mode fine-tuning, used to improve CK/CKN Vix violation.
 Default disabled, the driver setting is hardware auto tuned.

 ASTMMC_DDR4_MANUAL_RPU | ASTMMC_DDR4_MANUAL_RPD
 -----------------------+-----------------------
           No           |           x          : manual mode disabled
           Yes          |          No          : enable Rpu     manual setting
           Yes          |          Yes         : enable Rpu/Rpd manual setting
******************************************************************************/
//const ASTMMC_DDR4_MANUAL_RPU 0x0             @ 0x0-0xF, larger value means weaker: u32 = driving;
//const ASTMMC_DDR4_MANUAL_RPD 0x0             @ 0x0-0xF, larger value means stronger: u32 = driving;

/******************************************************************************
 Select initial reset mode as WDT_Full
 WDT_Full is a more complete reset mode than WDT_SOC.
 But if FW has other initial code executed before platform.S, then it should use WDT_SOC mode.
 Use WDT_Full may clear the initial result of prior initial code.
******************************************************************************/
//#define ASTMMC_INIT_RESET_MODE_FULL

/******************************************************************************
 There is a compatibility issue for Hynix DDR4 SDRAM.
 Hynix DDR4 SDRAM is more weak on noise margin compared to Micron and Samsung DDR4.
 To well support Hynix DDR4, it requlres to slow down the DDR4 operating frequency
 from 1600Mbps to 1536/1488/1440 Mbps. The target frequency that can be used depends
 on the MB layout. Customer can find the appropriate frequency for their products.
 Below are the new defined parameters for the Hynix DDR4 supporting.
******************************************************************************/
//const CONFIG_DDR4_SUPPORT_HYNIX              @ Enable this when Hynix DDR4 included in the: u32 = BOM;
//#define CONFIG_DDR4_HYNIX_SET_1536
//#define CONFIG_DDR4_HYNIX_SET_1488
const CONFIG_DDR4_HYNIX_SET_1440: u32 = 1;

const ASTMMC_REGIDX_010: u32 = 0x00/4;
const ASTMMC_REGIDX_014: u32 = 0x04/4;
const ASTMMC_REGIDX_018: u32 = 0x08/4;
const ASTMMC_REGIDX_020: u32 = 0x0C/4;
const ASTMMC_REGIDX_024: u32 = 0x10/4;
const ASTMMC_REGIDX_02C: u32 = 0x14/4;
const ASTMMC_REGIDX_030: u32 = 0x18/4;
const ASTMMC_REGIDX_214: u32 = 0x1C/4;
const ASTMMC_REGIDX_2E0: u32 = 0x20/4;
const ASTMMC_REGIDX_2E4: u32 = 0x24/4;
const ASTMMC_REGIDX_2E8: u32 = 0x28/4;
const ASTMMC_REGIDX_2EC: u32 = 0x2C/4;
const ASTMMC_REGIDX_2F0: u32 = 0x30/4;
const ASTMMC_REGIDX_2F4: u32 = 0x34/4;
const ASTMMC_REGIDX_2F8: u32 = 0x38/4;
const ASTMMC_REGIDX_RFC: u32 = 0x3C/4;
const ASTMMC_REGIDX_PLL: u32 = 0x40/4;
const ASTMMC_INIT_RESET_MODE_FULL: u32 = 0x0;

// We need to figure out the settings for these at some point.
const CONFIG_DRAM_1333: u32 = 0;
const CONFIG_DRAM_ECC: u32 = 0;
const CONFIG_DDR4_SUPPORT_HYNIX: u32 = 0;
const CONFIG_DDR4_HYNIX_SET_1536: u32 = 0;
const CONFIG_DDR4_HYNIX_SET_1488: u32 = 0;
const CONFIG_DRAM_UART_TO_UART1: u32 = 0;
const CONFIG_DRAM_UART_38400: u32 = 0;
const CONFIG_DRAM_UART_57600: u32 = 0;
const CONFIG_DDR3_8GSTACK: u32 = 0;
const CONFIG_DRAM_EXT_TEMP: u32 = 0;
const CONFIG_DDR4_4GX8: u32 = 0;
const ASTMMC_DDR4_MANUAL_RPD: u32 = 0;
const ASTMMC_DDR4_MANUAL_RPU: u32 = 0;


fn poke(a: u32, v: u32) -> () {
	let y = a as *mut u32;
	unsafe {ptr::write_volatile(y, v);}
		}
		fn peek(a: u32) -> u32 {
			let y = a as *const u32;
			unsafe {ptr::read_volatile(y)}
		}
		pub fn ram () -> () {
			let mut tptr = ramtable::TIME_TABLE_DDR3_1333;
			let mut r0 = 0u32;
			let mut r1= 0u32;
			let mut r2= 0u32;
			let mut r3= 0u32;
			let mut r4= 0u32;
			let mut r5= 0u32;
			let mut r6= 0u32;
			let mut r7= 0u32;
			let mut r8= 0u32;
			let mut r9= 0u32;
			let mut r10= 0u32;
			let mut r11= 0u32;
			let mut z= false;
			let mut gt= false;
			let mut lt= false;
	let mut s = State::init_dram;
	loop {
		s = match s {
			State::Exit => {
				break;
			`)
	}
	// This code deals with the very simple line-by-line structure
	// of the platform code. The bias is to emitting lines as-is.
	// In a small number of cases we rewrite the lines. Those
	// cases cover most of the code in the file, however.
	for _, l := range i {
		switch {
		case len(l) < 1:
			fmt.Println()
			continue
		case l[0] == "":
			fmt.Println(l[0])
			continue
		case l[1][0] == '/':
			fmt.Println(l[0])
			continue
		case l[1][0] == '*':
			fmt.Println(l[0])
			continue
		case l[1] == "#ifdef":
			fmt.Printf("\tif %s == 1  { // %s\n", l[2], l[0])
			continue
		case l[1] == "#ifndef":
			fmt.Printf("\tif %s == 0  { // %s\n", l[2], l[0])
			continue
		case l[1] == "#if" && l[2] == "defined":
			fmt.Printf("\tif %s == 1  { // %s\n", strings.TrimSuffix(strings.TrimPrefix(l[3], "("), ")"), l[0])
			continue
		case l[1] == "#elif" && l[2] == "defined":
			fmt.Printf("\n } else if %s == 1  { // %s\n", strings.TrimSuffix(strings.TrimPrefix(l[3], "("), ")"), l[0])
			continue
		case l[1] == "#else":
			fmt.Printf("} else { // %s ", l[0])
			continue
		case l[1] == "#endif":
			fmt.Printf("} // %s\n", l[0])
			continue
		case l[1][0] >= 'A' && l[1][0] <= 'Z':
			fmt.Println(l[0])
			continue
		case l[1] == ".LTORG":
			continue
		case len(l) == 2 && strings.HasSuffix(l[1], ":"):
			label := strings.TrimSuffix(l[1], ":")
			labels = labels + "\n" + fmt.Sprintf("%s = %d", label, enum) + ","
			fmt.Printf("State::%s\n}, \nState::%s => {\n", label, label)
			enum++
			continue
		}
		if _, ok := macros[l[1]]; ok {
			fmt.Println(l[1])
			continue
		}
		l = fixImm(l)

		// This could all be done much more nicely but ...
		// it's an unlikely to be used program ...
		pref, suf := "", ""
		if strings.HasSuffix(l[1], "gt") {
			pref, suf = "if gt {", "}"
			l[1] = strings.TrimSuffix(l[1], "gt")
		}
		if strings.HasSuffix(l[1], "ge") {
			pref, suf = "if gt || z {", "}"
			l[1] = strings.TrimSuffix(l[1], "ge")
		}
		if strings.HasSuffix(l[1], "le") {
			pref, suf = "if lt || z {", "}"
			l[1] = strings.TrimSuffix(l[1], "le")
		}
		if strings.HasSuffix(l[1], "lt") {
			pref, suf = "if lt {", "}"
			l[1] = strings.TrimSuffix(l[1], "lt")
		}
		if strings.HasSuffix(l[1], "ne") {
			pref, suf = "if ! z {", "}"
			l[1] = strings.TrimSuffix(l[1], "ne")
		}
		if strings.HasSuffix(l[1], "eq") {
			pref, suf = "if z {", "}"
			l[1] = strings.TrimSuffix(l[1], "eq")
		}
		code := ""

		switch l[1] {
		default:
			code = fmt.Sprintf("//Can't find instruction for %v", l[0])
		case "add":
			code = fmt.Sprintf("%s = %s + %s;", l[2], l[3], l[4])
		case "b":
			code = "s = State::" + l[2] + "; continue;"
		case "bic":
			code = fmt.Sprintf("%s = %s & !%s;", l[2], l[3], l[4])
		case "cmp":
			code = fmt.Sprintf("z = %s == %s;", l[2], l[3])
			code += fmt.Sprintf("gt = %s > %s;", l[2], l[3])
			code += fmt.Sprintf("lt = %s < %s;", l[2], l[3])
		case "beq":
			code = fmt.Sprintf("if z {s = %s; continue;}", l[1])
		case "mov":
			// don't bother with lr
			if l[2] == "lr" || l[3] == "lr" {
				continue
			}
			l[3] = shifty(3, l...)
			code = fmt.Sprintf("%s = %s;", l[2], l[3])
		case "mul":
			code = fmt.Sprintf("%s = %s * %s;", l[2], l[3], l[4])
		case "orr":
			l[4] = shifty(4, l...)
			code = fmt.Sprintf("%s = %s | %s;", l[2], l[3], l[4])
		case "and":
			l[4] = shifty(4, l...)
			code = fmt.Sprintf("%s = %s | %s;", l[2], l[3], l[4])
		case "adrl":
			code = fmt.Sprintf("tptr = ramtable::%s", l[3])
		case "ldr":
			src := l[3]
			// we know that in all cases the non-simple form uses the table pointer [tptr]
			if strings.HasPrefix(src, "[") {
				// Simple form?
				if strings.HasSuffix(src, "]") {
					src = fmt.Sprintf("peek(%s)", strings.TrimPrefix(strings.TrimSuffix(src, "]"), "["))
				} else {
					src = fmt.Sprintf("tptr[(%s) as usize]", strings.TrimSuffix(l[4], "]"))
				}
			}
			code = fmt.Sprintf("\t%s = %s;", l[2], src)
		case "mcr":
			code = fmt.Sprintf("//mcr(%s, %s, %s, %s, %s, %s);", l[2], l[3], l[4], l[5], l[6], l[7])
		case "mrc":
			code = fmt.Sprintf("//mrc(%s, %s, %s, %s, %s, %s);", l[2], l[3], l[4], l[5], l[6], l[7])
		case "str":
			code = fmt.Sprintf("poke(%s,%s);", l[2], strings.TrimSuffix(l[3][1:], "]"))
		case "subs":
			code = fmt.Sprintf("%s = %s - %s;", l[2], l[3], l[4])
		case "sub":
			code = fmt.Sprintf("%s = %s - %s;", l[2], l[3], l[4])
		case "tst":
			code = fmt.Sprintf("z = %s == %s;", l[2], l[3])
		case ".macro":
			macros[l[2]] = true
			code = fmt.Sprintf("macro_rules! %s{\t()=>{\n", l[2])
		case ".endm":
			code = "}}"
		// hack hack -- none macros -- 
		case "check_delay_timer", "clear_delay_timer", "init_delay_timer", "init_spi_checksum", "print_hex_char":
			code = fmt.Sprintf("%s!(r0, r1, r2, r3,  r4, r5, r6, r7, z, gt, lt);", l[1])

		}
		fmt.Printf("%s%s%s/*%q*/\n", pref, code, suf, l[0])
	}
	if *b {
		fmt.Printf("}\n}\n")
	}
	fmt.Printf("\n}\n}\nenum State {\n\n%s\nExit\n}\n", labels)
}
