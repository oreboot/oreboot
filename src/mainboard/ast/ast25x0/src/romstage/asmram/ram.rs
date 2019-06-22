macro_rules! init_delay_timer {
    ($r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6: ident, $r7: ident, $z:ident, $gt:ident, $lt:ident) => {
        /*"    .macro init_delay_timer"*/
        $r0 = 0x1e782024u32; /*"    ldr   $r0, =0x1e782024                        @ Set Time$r3 Reload"*/
        poke($r2, $r0); /*"    str   $r2, [$r0]"*/

        $r0 = 0x1e6c0038u32; /*"    ldr   $r0, =0x1e6c0038                        @ Clear Time$r3 ISR"*/
        $r1 = 0x00040000u32; /*"    ldr   $r1, =0x00040000"*/
        poke($r1, $r0); /*"    str   $r1, [$r0]"*/

        $r0 = 0x1e782030u32; /*"    ldr   $r0, =0x1e782030                        @ Enable Time$r3"*/
        $r2 = 7u32; /*"    mov   $r2, #7"*/
        $r1 = $r2 << 8u32; /*"    mov   $r1, $r2, lsl #8"*/
        poke($r1, $r0); /*"    str   $r1, [$r0]"*/

        $r0 = 0x1e6c0090u32; /*"    ldr   $r0, =0x1e6c0090                        @ Check ISR for Time$r3 timeout"*/
    };
} /*"    .endm"*/

macro_rules! check_delay_timer {
    ($r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6: ident, $r7: ident, $z:ident, $gt:ident, $lt:ident) => {
        /*"    .macro check_delay_timer"*/
        $r1 = peek($r0); /*"    ldr   $r1, [$r0]"*/
        $r1 = $r1 & !0xFFFBFFFFu32; /*"    bic   $r1, $r1, #0xFFFBFFFF"*/
        $r2 = $r1 >> 18u32; /*"    mov   $r2, $r1, lsr #18"*/
        $z = $r2 == 0x01u32;
    };
} /*"    .endm"*/

macro_rules! clear_delay_timer {
    ($r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6: ident, $r7: ident, $z:ident, $gt:ident, $lt:ident) => {
        /*"    .macro clear_delay_timer"*/
        $r0 = 0x1e78203Cu32; /*"    ldr   $r0, =0x1e78203C                        @ Disable Time$r3"*/
        $r2 = 0xFu32; /*"    mov   $r2, #0xF"*/
        $r1 = $r2 << 8u32; /*"    mov   $r1, $r2, lsl #8"*/
        poke($r1, $r0); /*"    str   $r1, [$r0]"*/

        $r0 = 0x1e6c0038u32; /*"    ldr   $r0, =0x1e6c0038                        @ Clear Time$r3 ISR"*/
        $r1 = 0x00040000u32; /*"    ldr   $r1, =0x00040000"*/
        poke($r1, $r0); /*"    str   $r1, [$r0]"*/
    };
} /*"    .endm"*/

macro_rules! init_spi_checksum {
    ($r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6: ident, $r7: ident, $z:ident, $gt:ident, $lt:ident) => {
        /*"    .macro init_spi_checksum"*/
        $r0 = 0x1e620084u32; /*"    ldr   $r0, =0x1e620084"*/
        $r1 = 0x20010000u32; /*"    ldr   $r1, =0x20010000"*/
        poke($r1, $r0); /*"    str   $r1, [$r0]"*/
        $r0 = 0x1e62008Cu32; /*"    ldr   $r0, =0x1e62008C"*/
        $r1 = 0x20000200u32; /*"    ldr   $r1, =0x20000200"*/
        poke($r1, $r0); /*"    str   $r1, [$r0]"*/
        $r0 = 0x1e620080u32; /*"    ldr   $r0, =0x1e620080"*/
        $r1 = 0x0000000Du32; /*"    ldr   $r1, =0x0000000D"*/
        $r2 = $r2 | $r7; /*"    orr   $r2, $r2, $r7"*/
        $r1 = $r1 | ($r2 << 8u32); /*"    orr   $r1, $r1, $r2, lsl #8"*/
        $r2 = $r6 | 0xFu32; /*"    and   $r2, $r6, #0xF"*/
        $r1 = $r1 | ($r2 << 4u32); /*"    orr   $r1, $r1, $r2, lsl #4"*/
        poke($r1, $r0); /*"    str   $r1, [$r0]"*/
        $r0 = 0x1e620008u32; /*"    ldr   $r0, =0x1e620008"*/
        $r2 = 0x00000800u32; /*"    ldr   $r2, =0x00000800"*/
    };
} /*"    .endm"*/

macro_rules! print_hex_char {
    ($r0:ident, $r1:ident, $r2:ident, $r3:ident, $r4:ident, $r5:ident, $r6: ident, $r7: ident, $z:ident, $gt:ident, $lt:ident) => {
        /*"    .macro print_hex_char"*/
        $r1 = $r1 | 0xFu32; /*"    and   $r1, $r1, #0xF"*/
        $z = $r1 == 9u32;
        $gt = $r1 > 9u32;
        $lt = $r1 < 9u32; /*"    cmp   $r1, #9"*/
        if $gt {
            $r1 = $r1 + 0x37u32;
        } /*"    add$gt $r1, $r1, #0x37"*/
        if $lt || $z {
            $r1 = $r1 + 0x30u32;
        } /*"    addle $r1, $r1, #0x30"*/
        poke($r1, $r0); /*"    str   $r1, [$r0]"*/
    };
} /*"    .endm"*/
