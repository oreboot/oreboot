/*
 * MIT License
 *
 * Copyright (c) 2018 Andre Richter <andre.o.richter@gmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

ENTRY(_vector_table);

/* 0x0000_0000..0x0800_0000: Flash memory */
/* 0x0e00_0000..0x0eff_ffff: Secure SRAM */
/* 0x4000_0000..: RAM */

MEMORY
{
    FLASH_BOOT (rx) : ORIGIN = 0x0, LENGTH = 0x80000
    FLASH_DATA (r) : ORIGIN = 0x80000, LENGTH = 0x7F80000
    SRAM (rwx) : ORIGIN = 0xe000000, LENGTH = 0x1000000
    DRAM (rwx) : ORIGIN = 0x40000000, LENGTH = 0x1000000
}

SECTIONS
{
    .text :
    {
	KEEP(*(.bootblock.vectortable));
        KEEP(*(.text .text.*));
    } > FLASH_BOOT

    .rodata :
    {
        KEEP(*(.rodata .rodata.*));
    } > FLASH_BOOT

    .data :
    {
	_data = .;
	KEEP(*(.data .data.*));
	_edata = .;
    } > DRAM AT > FLASH_BOOT

    .bss (NOLOAD) :
    { 
	. = ALIGN(16);
	_bss = .;
	*(.sbss)
	*(.sbss.*)
	*(.bss)
	*(.bss.*)
	*(COMMON)
	. = ALIGN(16);
	_ebss = .;
    } > DRAM

    /* Stack and heap */
    .heap (NOLOAD) :
   {
         . = ALIGN(16);
        _heap = .;
        . += 64*1024;
        . = ALIGN(16);
        _eheap = .;
    } > DRAM

    .stack (NOLOAD) :
    {
        _estack = .;
        . += 64*1024;
        . = ALIGN(16);
        _stack = .;
    } > DRAM

    _end = .;

    .stack_sizes (INFO) :
    {
        KEEP(*(.stack_sizes));
    }

    /DISCARD/ : { *(.comment) *(.gnu*) *(.note*) *(.eh_frame*)
	    	/* Unused exception related info that only wastes space */
		*(.ARM.exidx.*);
		*(.ARM.extab.*);
		}
}
