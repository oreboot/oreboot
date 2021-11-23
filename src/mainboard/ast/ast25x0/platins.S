InitDram:
    /* save lr */
    mov   r4, lr

    /********************************************
       Initial Reset Procedure : Begin
     *******************************************/
    /* Clear AHB bus lock condition */
    ldr   r0, =0x1e600000
    ldr   r1, =0xAEED1A03
    str   r1, [r0]
    ldr   r0, =0x1e600084
    ldr   r1, =0x00010000
    str   r1, [r0]
    add   r0, r0, #0x4
    mov   r1, #0x0
    str   r1, [r0]

    ldr   r0, =0x1e6e2000
    ldr   r1, =0x1688a8a8
    str   r1, [r0]

    /* Reset again */
    ldr   r0, =0x1e6e2070                        @ check fast reset flag
    ldr   r2, =0x08000000
    ldr   r1, [r0]
    tst   r1, r2
    beq   BypassFirstReset

    ldr   r0, =0x1e785010
    ldr   r3, [r0]
    cmp   r3, #0x0
    beq   StartFirstReset
    add   r0, r0, #0x04
    mov   r3, #0x77
    str   r3, [r0]
    ldr   r0, =0x1e720004                        @ Copy initial strap register to 0x1e720004
    str   r1, [r0]
    add   r0, r0, #0x04                          @ Copy initial strap register to 0x1e720008
    str   r1, [r0]
    add   r0, r0, #0x04                          @ Copy initial strap register to 0x1e72000c
    str   r1, [r0]
    ldr   r0, =0x1e6e207c                        @ clear fast reset flag
    str   r2, [r0]
    ldr   r0, =0x1e6e203c                        @ clear watchdog reset flag
    ldr   r1, [r0]
    and   r1, r1, #0x01
    str   r1, [r0]
    ldr   r0, =0x1e78501c                        @ restore normal mask setting
    ldr   r1, =0x023FFFF3                        @ added 2016.09.06
    str   r1, [r0]
    b     BypassFirstReset

StartFirstReset:
#ifdef ASTMMC_INIT_RESET_MODE_FULL
    ldr   r0, =0x1e785004
    ldr   r1, =0x00000001
    str   r1, [r0]
    ldr   r0, =0x1e785008
    ldr   r1, =0x00004755
    str   r1, [r0]
    ldr   r0, =0x1e78500c                        @ enable Full reset
    ldr   r1, =0x00000033
    str   r1, [r0]
#else
    /***** Clear LPC status : Begin *****/
    mov   r2, #0                                 @ set r2 = 0, freezed
    ldr   r0, =0x1e787008
    mov   r1, #0x7
    str   r1, [r0]
    ldr   r0, =0x1e78700c
    mov   r1, #0x3
    str   r1, [r0]
    ldr   r0, =0x1e787020
    str   r2, [r0]
    ldr   r0, =0x1e787034
    str   r2, [r0]
    ldr   r0, =0x1e787004
    str   r2, [r0]
    ldr   r0, =0x1e787010
    str   r2, [r0]
    ldr   r0, =0x1e78701c
    str   r2, [r0]
    ldr   r0, =0x1e787014                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e787018                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e787008                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e78301c                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e78d01c                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e78e01c                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e78f01c                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e788020
    str   r2, [r0]
    ldr   r0, =0x1e788034
    str   r2, [r0]
    ldr   r0, =0x1e78800c
    str   r2, [r0]
    ldr   r0, =0x1e789008
    str   r2, [r0]
    ldr   r0, =0x1e789010
    mov   r1, #0x40
    str   r1, [r0]
    ldr   r0, =0x1e789024                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e789028                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e78902c                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e789114                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e789124                        @ read clear
    ldr   r1, [r0]
    ldr   r0, =0x1e78903c
    str   r2, [r0]
    ldr   r0, =0x1e789040
    str   r2, [r0]
    ldr   r0, =0x1e789044
    str   r2, [r0]
    ldr   r0, =0x1e78911c
    str   r2, [r0]
    ldr   r0, =0x1e78912c
    ldr   r1, =0x200
    str   r1, [r0]
    ldr   r0, =0x1e789104
    ldr   r1, =0xcc00
    str   r1, [r0]
    ldr   r0, =0x1e789108
    str   r2, [r0]
    ldr   r0, =0x1e78910c
    ldr   r1, =0x1f0
    str   r1, [r0]
    ldr   r0, =0x1e789170
    str   r2, [r0]
    ldr   r0, =0x1e789174
    str   r2, [r0]
    ldr   r0, =0x1e7890a0
    ldr   r1, =0xff00
    str   r1, [r0]
    ldr   r0, =0x1e7890a4
    str   r2, [r0]
    ldr   r0, =0x1e789080
    ldr   r1, =0x400
    str   r1, [r0]
    ldr   r0, =0x1e789084
    ldr   r1, =0x0001000f
    str   r1, [r0]
    ldr   r0, =0x1e789088
    ldr   r1, =0x3000fff8
    str   r1, [r0]
    ldr   r0, =0x1e78908c
    ldr   r1, =0xfff8f007
    str   r1, [r0]
    ldr   r0, =0x1e789098
    ldr   r1, =0x00000a30
    str   r1, [r0]
    ldr   r0, =0x1e78909c
    str   r2, [r0]
    ldr   r0, =0x1e789100
    str   r2, [r0]
    ldr   r0, =0x1e789130
    ldr   r1, =0x00000080
    str   r1, [r0]
    ldr   r0, =0x1e789138
    ldr   r1, =0x00010198
    str   r1, [r0]
    ldr   r0, =0x1e789140
    ldr   r1, =0x0000a000
    str   r1, [r0]
    ldr   r0, =0x1e789158
    ldr   r1, =0x00000080
    str   r1, [r0]
    ldr   r0, =0x1e789180
    ldr   r1, =0xb6db1bff
    str   r1, [r0]
    ldr   r0, =0x1e789184
    str   r2, [r0]
    ldr   r0, =0x1e789188
    str   r2, [r0]
    ldr   r0, =0x1e78918c
    str   r2, [r0]
    ldr   r0, =0x1e789190
    ldr   r1, =0x05020100
    str   r1, [r0]
    ldr   r0, =0x1e789194
    ldr   r1, =0x07000706
    str   r1, [r0]
    ldr   r0, =0x1e789198
    str   r2, [r0]
    ldr   r0, =0x1e78919c
    ldr   r1, =0x30
    str   r1, [r0]
    ldr   r0, =0x1e7891a0
    ldr   r1, =0x00008100
    str   r1, [r0]
    ldr   r0, =0x1e7891a4
    ldr   r1, =0x2000
    str   r1, [r0]
    ldr   r0, =0x1e7891a8
    ldr   r1, =0x3ff
    str   r1, [r0]
    ldr   r0, =0x1e7891ac
    str   r2, [r0]
    ldr   r0, =0x1e789240
    mov   r1, #0xff
    str   r1, [r0]
    ldr   r0, =0x1e789244
    str   r1, [r0]
    ldr   r0, =0x1e789248
    mov   r1, #0x80
    str   r1, [r0]
    ldr   r0, =0x1e789250
    str   r2, [r0]
    ldr   r0, =0x1e789254
    str   r2, [r0]
    /***** Clear LPC status : End *****/

    ldr   r0, =0x1e62009c                        @ clear software strap flag for doing again after reset
    ldr   r1, =0xAEEDFC20
    str   r1, [r0]
    ldr   r0, =0x1e785004
    ldr   r1, =0x00000001
    str   r1, [r0]
    ldr   r0, =0x1e785008
    ldr   r1, =0x00004755
    str   r1, [r0]
    ldr   r0, =0x1e78501c                        @ enable full mask of SOC reset
    ldr   r1, =0x03FFFFFF                        @ added 2016.09.06
    str   r1, [r0]
    ldr   r0, =0x1e78500c                        @ enable SOC reset
    ldr   r1, =0x00000013
    str   r1, [r0]
#endif
WaitFirstReset:
    b     WaitFirstReset

    /********************************************
       Initial Reset Procedure : End
     *******************************************/

BypassFirstReset:
    /* Enable Timer separate clear mode */
    ldr   r0, =0x1e782038
    mov   r1, #0xAE
    str   r1, [r0]

/* Test - DRAM initial time */
    ldr   r0, =0x1e78203c
    ldr   r1, =0x0000F000
    str   r1, [r0]

    ldr   r0, =0x1e782044
    ldr   r1, =0xFFFFFFFF
    str   r1, [r0]

    ldr   r0, =0x1e782030
    mov   r2, #3
    mov   r1, r2, lsl #12
    str   r1, [r0]
/* Test - DRAM initial time */

    /*Set Scratch register Bit 7 before initialize*/
    ldr   r0, =0x1e6e2000
    ldr   r1, =0x1688a8a8
    str   r1, [r0]

    ldr   r0, =0x1e6e2040
    ldr   r1, [r0]
    orr   r1, r1, #0x80
    str   r1, [r0]

    /* Change LPC reset source to PERST# when eSPI mode enabled */
    ldr   r0, =0x1e6e2070
    ldr   r1, [r0]
    ldr   r0, =0x1e6e207c
    ldr   r2, =0x02000000
    ldr   r3, =0x00004000
    tst   r1, r2
    strne r3, [r0]

    /* Configure USB ports to the correct pin state */
    ldr   r0, =0x1e6e200c                        @ enable portA clock
    ldr   r2, =0x00004000
    ldr   r1, [r0]
    orr   r1, r1, r2
    str   r1, [r0]
    ldr   r0, =0x1e6e2090                        @ set portA as host mode
    ldr   r1, =0x2000A000
    str   r1, [r0]
    ldr   r0, =0x1e6e2094                        @ set portB as host mode
    ldr   r1, =0x00004000
    str   r1, [r0]
    ldr   r0, =0x1e6e2070
    ldr   r2, =0x00800000
    ldr   r1, [r0]
    tst   r1, r2
    beq   BypassUSBInit
    ldr   r0, =0x1e6e207c
    str   r2, [r0]

    /* Delay about 1ms */
    clear_delay_timer
    ldr   r2, =0x000003E8                        @ Set Timer3 Reload = 1 ms
    init_delay_timer
WaitUsbInit:
    check_delay_timer
    bne   WaitUsbInit
    clear_delay_timer
    /* end delay 1ms */

    ldr   r0, =0x1e6e2070
    ldr   r1, =0x00800000
    str   r1, [r0]

BypassUSBInit:
    /* Enable AXI_P */
    ldr   r0, =0x00000016
    mrc   p15, 0, r1, c15, c2, 4
    mcr   p15, 0, r0, c15, c2, 4

/******************************************************************************
 Disable WDT2 for 2nd boot function
 ******************************************************************************/
/*
#ifndef CONFIG_FIRMWARE_2ND_BOOT
    ldr   r0, =0x1e78502c
    mov   r1, #0
    str   r1, [r0]
#endif
*/
/******************************************************************************
 Disable WDT3 for SPI Address mode (3 or 4 bytes) detection function
 ******************************************************************************/
    ldr   r0, =0x1e78504c
    mov   r1, #0
    str   r1, [r0]

    ldr   r0, =0x1e6e0000
    ldr   r1, =0xFC600309
    str   r1, [r0]

    /* Check Scratch Register Bit 6 */
    ldr   r0, =0x1e6e2040
    ldr   r1, [r0]
    bic   r1, r1, #0xFFFFFFBF
    mov   r2, r1, lsr #6
    cmp   r2, #0x01
    beq   PlatformExit

    /* Disable VGA display */
    ldr   r0, =0x1e6e202c
    ldr   r1, [r0]
    orr   r1, r1, #0x40
    str   r1, [r0]

    ldr   r0, =0x1e6e2070                        @ Load strap register
    ldr   r3, [r0]

    /* Set M-PLL */
#if   defined (CONFIG_DRAM_1333)
    ldr   r2, =0xC48066C0                        @ load PLL parameter for 24Mhz CLKIN (330)
#else
    ldr   r2, =0x93002400                        @ load PLL parameter for 24Mhz CLKIN (396)
#if   defined (CONFIG_DDR4_SUPPORT_HYNIX)
    mov   r1, r3, lsr #24                        @ Check DDR4
    tst   r1, #0x01
    beq   BypassMpllHynixMode1
#if   defined (CONFIG_DDR4_HYNIX_SET_1536)
    ldr   r2, =0x930023E0                        @ load PLL parameter for 24Mhz CLKIN (384)
#elif defined (CONFIG_DDR4_HYNIX_SET_1488)
    ldr   r2, =0x930023C0                        @ load PLL parameter for 24Mhz CLKIN (372)
#else
    ldr   r2, =0x930023A0                        @ load PLL parameter for 24Mhz CLKIN (360)
#endif
	b BypassMpllHynixMode1
#endif
#endif

BypassMpllHynixMode1:
    mov   r1, r3, lsr #23                        @ Check CLKIN = 25MHz
    tst   r1, #0x01
    beq   SetMPLL
#if   defined (CONFIG_DRAM_1333)
    ldr   r2, =0xC4806680                        @ load PLL parameter for 25Mhz CLKIN (331)
#else
    ldr   r2, =0x930023E0                        @ load PLL parameter for 25Mhz CLKIN (400)
#if   defined (CONFIG_DDR4_SUPPORT_HYNIX)
    mov   r1, r3, lsr #24                        @ Check DDR4
    tst   r1, #0x01
    beq   BypassMpllHynixMode2
#if   defined (CONFIG_DDR4_HYNIX_SET_1536)
    ldr   r2, =0x930023C0                        @ load PLL parameter for 24Mhz CLKIN (387.5)
#elif defined (CONFIG_DDR4_HYNIX_SET_1488)
    ldr   r2, =0x930023A0                        @ load PLL parameter for 24Mhz CLKIN (375)
#else
    ldr   r2, =0x93002380                        @ load PLL parameter for 24Mhz CLKIN (362.5)
    b   BypassMpllHynixMode2
#endif
#endif
#endif
BypassMpllHynixMode2:
    ldr   r0, =0x1e6e2160                        @ set 24M Jitter divider (HPLL=825MHz)
    ldr   r1, =0x00011320
    str   r1, [r0]

SetMPLL:
    ldr   r0, =0x1e6e2020                        @ M-PLL (DDR SDRAM) Frequency
    str   r2, [r0]

    clear_delay_timer

    /* Delay about 3ms */
    ldr   r2, =0x00000BB8                        @ Set Timer3 Reload = 3 ms
    init_delay_timer
WaitMpllInit:
    check_delay_timer
    bne   WaitMpllInit
    clear_delay_timer
    /* end delay 3ms */

    /* Reset MMC */
ResetMmc:
    ldr   r0, =0x1e78505c
    ldr   r1, =0x00000004
    str   r1, [r0]
    ldr   r0, =0x1e785044
    ldr   r1, =0x00000001
    str   r1, [r0]
    ldr   r0, =0x1e785048
    ldr   r1, =0x00004755
    str   r1, [r0]
    ldr   r0, =0x1e78504c
    ldr   r1, =0x00000013
    str   r1, [r0]
WaitMmcReset:
    ldr   r1, [r0]
    tst   r1, #0x02
    bne   WaitMmcReset

    ldr   r0, =0x1e78505c
    ldr   r1, =0x023FFFF3
    str   r1, [r0]
    ldr   r0, =0x1e785044
    ldr   r1, =0x000F4240
    str   r1, [r0]
    ldr   r0, =0x1e785048
    ldr   r1, =0x00004755
    str   r1, [r0]
    ldr   r0, =0x1e785054
    ldr   r1, =0x00000077
    str   r1, [r0]

    ldr   r0, =0x1e6e0000
    ldr   r1, =0xFC600309
WaitMmcResetDone:
    str   r1, [r0]
    ldr   r2, [r0]
    cmp   r2, #0x1
    bne   WaitMmcResetDone

    ldr   r0, =0x1e6e0034                        @ disable MMC request
    ldr   r1, =0x00020000
    str   r1, [r0]

    /* Delay about 10ms */
    ldr   r2, =0x00002710                        @ Set Timer3 Reload = 10 ms
    init_delay_timer
WaitDdrReset:
    check_delay_timer
    bne   WaitDdrReset
    clear_delay_timer
    /* end delay 10ms */

/* Debug - UART console message */
#ifdef CONFIG_DRAM_UART_TO_UART1
    ldr   r0, =0x1e78909c                        @ route UART5 to UART Port1, 2016.08.29
    ldr   r1, =0x10000004
    str   r1, [r0]

    ldr   r0, =0x1e6e2084
    ldr   r1, [r0]
    mov   r2, #0xC0                              @ Enable pinmux of TXD1/RXD1
    orr   r1, r1, r2, lsl #16
    str   r1, [r0]
#endif

    ldr   r0, =0x1e78400c
    mov   r1, #0x83
    str   r1, [r0]

    ldr   r0, =0x1e6e202c
    ldr   r2, [r0]
    mov   r2, r2, lsr #12
    tst   r2, #0x01
    ldr   r0, =0x1e784000
    moveq r1, #0x0D                              @ Baudrate 115200
    movne r1, #0x01                              @ Baudrate 115200, div13
#ifdef CONFIG_DRAM_UART_38400
    moveq r1, #0x27                              @ Baudrate 38400
    movne r1, #0x03                              @ Baudrate 38400 , div13
#endif
#ifdef CONFIG_DRAM_UART_57600
    moveq r1, #0x1A                              @ Baudrate 57600
    movne r1, #0x02                              @ Baudrate 57600 , div13
#endif
    str   r1, [r0]

    ldr   r0, =0x1e784004
    mov   r1, #0x00
    str   r1, [r0]

    ldr   r0, =0x1e78400c
    mov   r1, #0x03
    str   r1, [r0]

    ldr   r0, =0x1e784008
    mov   r1, #0x07
    str   r1, [r0]

    ldr   r0, =0x1e784000
    mov   r1, #0x0D                              @ '\r'
    str   r1, [r0]
    mov   r1, #0x0A                              @ '\n'
    str   r1, [r0]
    mov   r1, #0x44                              @ 'D'
    str   r1, [r0]
    mov   r1, #0x52                              @ 'R'
    str   r1, [r0]
    mov   r1, #0x41                              @ 'A'
    str   r1, [r0]
    mov   r1, #0x4D                              @ 'M'
    str   r1, [r0]
    mov   r1, #0x20                              @ ' '
    str   r1, [r0]
    mov   r1, #0x49                              @ 'I'
    str   r1, [r0]
    mov   r1, #0x6E                              @ 'n'
    str   r1, [r0]
    mov   r1, #0x69                              @ 'i'
    str   r1, [r0]
    mov   r1, #0x74                              @ 't'
    str   r1, [r0]
    mov   r1, #0x2D                              @ '-'
    str   r1, [r0]
    mov   r1, #0x56                              @ 'V'
    str   r1, [r0]
    mov   r1, #AstMmcInitVer
    mov   r1, r1, lsr #4
    print_hex_char
    mov   r1, #AstMmcInitVer
    print_hex_char
    mov   r1, #0x2D                              @ '-'
    str   r1, [r0]
    ldr   r0, =0x1e784014
WaitPrint:
    ldr   r1, [r0]
    tst   r1, #0x40
    beq   WaitPrint
    ldr   r0, =0x1e784000
    mov   r1, #0x44                              @ 'D'
    str   r1, [r0]
    mov   r1, #0x44                              @ 'D'
    str   r1, [r0]
    mov   r1, #0x52                              @ 'R'
    str   r1, [r0]
/* Debug - UART console message */

/******************************************************************************
 Init DRAM common registers
 ******************************************************************************/
    ldr   r0, =0x1e6e0034                        @ disable SDRAM reset
    ldr   r1, =0x00020080
    str   r1, [r0]

    ldr   r0, =0x1e6e0008
    ldr   r1, =0x2003000F                        /* VGA */
    str   r1, [r0]

    ldr   r0, =0x1e6e0038                        @ disable all DRAM requests except CPU during PHY init
    ldr   r1, =0xFFFFEBFF
    str   r1, [r0]

    ldr   r0, =0x1e6e0040
    ldr   r1, =0x88448844
    str   r1, [r0]

    ldr   r0, =0x1e6e0044
    ldr   r1, =0x24422288
    str   r1, [r0]

    ldr   r0, =0x1e6e0048
    ldr   r1, =0x22222222
    str   r1, [r0]

    ldr   r0, =0x1e6e004c
    ldr   r1, =0x22222222
    str   r1, [r0]

    ldr   r0, =0x1e6e0050
    ldr   r1, =0x80000000
    str   r1, [r0]

    ldr   r1, =0x00000000
    ldr   r0, =0x1e6e0208                        @ PHY Setting
    str   r1, [r0]
    ldr   r0, =0x1e6e0218
    str   r1, [r0]
    ldr   r0, =0x1e6e0220
    str   r1, [r0]
    ldr   r0, =0x1e6e0228
    str   r1, [r0]
    ldr   r0, =0x1e6e0230
    str   r1, [r0]
    ldr   r0, =0x1e6e02a8
    str   r1, [r0]
    ldr   r0, =0x1e6e02b0
    str   r1, [r0]

    ldr   r0, =0x1e6e0240
    ldr   r1, =0x86000000
    str   r1, [r0]

    ldr   r0, =0x1e6e0244
    ldr   r1, =0x00008600
    str   r1, [r0]

    ldr   r0, =0x1e6e0248
    ldr   r1, =0x80000000
    str   r1, [r0]

    ldr   r0, =0x1e6e024c
    ldr   r1, =0x80808080
    str   r1, [r0]

    /* Check DRAM Type by H/W Trapping */
    ldr   r0, =0x1e6e2070
    ldr   r1, [r0]
    ldr   r2, =0x01000000                        @ bit[24]=1 => DDR4
    tst   r1, r2
    bne   Ddr4Init
    b     Ddr3Init
.LTORG

/******************************************************************************
 DDR3 Init
 ******************************************************************************/
Ddr3Init:
/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x33                              @ '3'
    str   r1, [r0]
    mov   r1, #0x0D                              @ '\r'
    str   r1, [r0]
    mov   r1, #0x0A                              @ '\n'
    str   r1, [r0]
/* Debug - UART console message */

#if   defined (CONFIG_DRAM_1333)
    adrl  r5, TIME_TABLE_DDR3_1333               @ Init DRAM parameter table
#else
    adrl  r5, TIME_TABLE_DDR3_1600
#endif

    ldr   r0, =0x1e6e0004
#ifdef CONFIG_DDR3_8GSTACK
    ldr   r1, =0x00000323                        @ Init to 8GB stack
#else
    ldr   r1, =0x00000303                        @ Init to 8GB
#endif
    str   r1, [r0]

    ldr   r0, =0x1e6e0010
    ldr   r1, [r5, #ASTMMC_REGIDX_010]
    str   r1, [r0]

    ldr   r0, =0x1e6e0014
    ldr   r1, [r5, #ASTMMC_REGIDX_014]
    str   r1, [r0]

    ldr   r0, =0x1e6e0018
    ldr   r1, [r5, #ASTMMC_REGIDX_018]
    str   r1, [r0]

    /* DRAM Mode Register Setting */
    ldr   r0, =0x1e6e0020                        @ MRS_4/6
    ldr   r1, [r5, #ASTMMC_REGIDX_020]
    str   r1, [r0]

    ldr   r0, =0x1e6e0024                        @ MRS_5
    ldr   r1, [r5, #ASTMMC_REGIDX_024]
    str   r1, [r0]

    ldr   r0, =0x1e6e002c                        @ MRS_0/2
    ldr   r1, [r5, #ASTMMC_REGIDX_02C]
    mov   r2, #0x1
    orr   r1, r1, r2, lsl #8
    str   r1, [r0]

    ldr   r0, =0x1e6e0030                        @ MRS_1/3
    ldr   r1, [r5, #ASTMMC_REGIDX_030]
    str   r1, [r0]

    /* Start DDR PHY Setting */
    ldr   r0, =0x1e6e0200
    ldr   r1, =0x02492AAE
    str   r1, [r0]

    ldr   r0, =0x1e6e0204
#ifdef CONFIG_DDR3_8GSTACK
    ldr   r1, =0x10001001
#else
    ldr   r1, =0x00001001
#endif
    str   r1, [r0]

    ldr   r0, =0x1e6e020c
    ldr   r1, =0x55E00B0B
    str   r1, [r0]

    ldr   r0, =0x1e6e0210
    ldr   r1, =0x20000000
    str   r1, [r0]

    ldr   r0, =0x1e6e0214
    ldr   r1, [r5, #ASTMMC_REGIDX_214]
    str   r1, [r0]

    ldr   r0, =0x1e6e02e0
    ldr   r1, [r5, #ASTMMC_REGIDX_2E0]
    str   r1, [r0]

    ldr   r0, =0x1e6e02e4
    ldr   r1, [r5, #ASTMMC_REGIDX_2E4]
    str   r1, [r0]

    ldr   r0, =0x1e6e02e8
    ldr   r1, [r5, #ASTMMC_REGIDX_2E8]
    str   r1, [r0]

    ldr   r0, =0x1e6e02ec
    ldr   r1, [r5, #ASTMMC_REGIDX_2EC]
    str   r1, [r0]

    ldr   r0, =0x1e6e02f0
    ldr   r1, [r5, #ASTMMC_REGIDX_2F0]
    str   r1, [r0]

    ldr   r0, =0x1e6e02f4
    ldr   r1, [r5, #ASTMMC_REGIDX_2F4]
    str   r1, [r0]

    ldr   r0, =0x1e6e02f8
    ldr   r1, [r5, #ASTMMC_REGIDX_2F8]
    str   r1, [r0]

    ldr   r0, =0x1e6e0290
    ldr   r1, =0x00100008
    str   r1, [r0]

    ldr   r0, =0x1e6e02c0
    ldr   r1, =0x00000006
    str   r1, [r0]

    /* Controller Setting */
    ldr   r0, =0x1e6e0060                        @ Fire DDRPHY Init
    ldr   r1, =0x00000005
    str   r1, [r0]

    ldr   r0, =0x1e6e0034
    ldr   r1, =0x00020091
    str   r1, [r0]

/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x30                              @ '0'
    str   r1, [r0]
/* Debug - UART console message */

    ldr   r0, =0x1e6e0120
    mov   r1, #0x00
    str   r1, [r0]
    b     DdrPhyInitProcess

Ddr3PhyinitDone:

    /********************************************
     Check Read training margin
    ********************************************/
    ldr   r0, =0x1e6e03a0                        @ check Gate Training Pass Window
    ldr   r1, [r0]
    ldr   r2, =0x150
    bic   r0, r1, #0xFF000000
    bic   r0, r0, #0x00FF0000
    cmp   r0, r2
    blt   DdrTestFail
    mov   r0, r1, lsr #16
    cmp   r0, r2
    blt   DdrTestFail

    ldr   r0, =0x1e6e03d0                        @ check Read Data Eye Training Pass Window
    ldr   r1, [r0]
    ldr   r2, =0x90
    bic   r0, r1, #0x0000FF00
    cmp   r0, r2
    blt   DdrTestFail
    mov   r0, r1, lsr #8
    cmp   r0, r2
    blt   DdrTestFail
    /*******************************************/

/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x31                              @ '1'
    str   r1, [r0]
/* Debug - UART console message */

    ldr   r0, =0x1e6e000c
    ldr   r1, =0x00000040
    str   r1, [r0]

#ifdef CONFIG_DDR3_8GSTACK
    ldr   r0, =0x1e6e0028
    ldr   r1, =0x00000025
    str   r1, [r0]

    ldr   r0, =0x1e6e0028
    ldr   r1, =0x00000027
    str   r1, [r0]

    ldr   r0, =0x1e6e0028
    ldr   r1, =0x00000023
    str   r1, [r0]

    ldr   r0, =0x1e6e0028
    ldr   r1, =0x00000021
    str   r1, [r0]
#endif

    ldr   r0, =0x1e6e0028
    ldr   r1, =0x00000005
    str   r1, [r0]

    ldr   r0, =0x1e6e0028
    ldr   r1, =0x00000007
    str   r1, [r0]

    ldr   r0, =0x1e6e0028
    ldr   r1, =0x00000003
    str   r1, [r0]

    ldr   r0, =0x1e6e0028
    ldr   r1, =0x00000011
    str   r1, [r0]

    ldr   r0, =0x1e6e000c
    ldr   r1, =0x00005C41
    str   r1, [r0]

    ldr   r0, =0x1e6e0034
    ldr   r2, =0x70000000
Ddr3CheckDllrdy:
    ldr   r1, [r0]
    tst   r1, r2
    bne   Ddr3CheckDllrdy

    ldr   r0, =0x1e6e000c
#ifdef CONFIG_DRAM_EXT_TEMP
    ldr   r1, =0x42AA2F81
#else
    ldr   r1, =0x42AA5C81
#endif
    str   r1, [r0]

    ldr   r0, =0x1e6e0034
    ldr   r1, =0x0001AF93
    str   r1, [r0]

    ldr   r0, =0x1e6e0120                        @ VGA Compatible Mode
    ldr   r1, [r5, #ASTMMC_REGIDX_PLL]
    str   r1, [r0]

    b     CalibrationEnd
.LTORG
/******************************************************************************
 End DDR3 Init
 ******************************************************************************/
/******************************************************************************
 DDR4 Init
 ******************************************************************************/
Ddr4Init:
/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x34                              @ '4'
    str   r1, [r0]
    mov   r1, #0x0D                              @ '\r'
    str   r1, [r0]
    mov   r1, #0x0A                              @ '\n'
    str   r1, [r0]
/* Debug - UART console message */

#if   defined (CONFIG_DRAM_1333)
    adrl  r5, TIME_TABLE_DDR4_1333               @ Init DRAM parameter table
#else
    adrl  r5, TIME_TABLE_DDR4_1600
#endif

    ldr   r0, =0x1e6e0004
#ifdef CONFIG_DDR4_4GX8
    ldr   r1, =0x00002313                        @ Init to 8GB
#else
    ldr   r1, =0x00000313                        @ Init to 8GB
#endif
    str   r1, [r0]

    ldr   r0, =0x1e6e0010
    ldr   r1, [r5, #ASTMMC_REGIDX_010]
    str   r1, [r0]

    ldr   r0, =0x1e6e0014
    ldr   r1, [r5, #ASTMMC_REGIDX_014]
    str   r1, [r0]

    ldr   r0, =0x1e6e0018
    ldr   r1, [r5, #ASTMMC_REGIDX_018]
    str   r1, [r0]

    /* DRAM Mode Register Setting */
    ldr   r0, =0x1e6e0020                        @ MRS_4/6
    ldr   r1, [r5, #ASTMMC_REGIDX_020]
    str   r1, [r0]

    ldr   r0, =0x1e6e0024                        @ MRS_5
    ldr   r1, [r5, #ASTMMC_REGIDX_024]
    str   r1, [r0]

    ldr   r0, =0x1e6e002c                        @ MRS_0/2
    ldr   r1, [r5, #ASTMMC_REGIDX_02C]
    mov   r2, #0x1
    orr   r1, r1, r2, lsl #8
    str   r1, [r0]

    ldr   r0, =0x1e6e0030                        @ MRS_1/3
    ldr   r1, [r5, #ASTMMC_REGIDX_030]
    str   r1, [r0]

    /* Start DDR PHY Setting */
    ldr   r0, =0x1e6e0200
    ldr   r1, =0x42492AAE
    str   r1, [r0]

    ldr   r0, =0x1e6e0204
    ldr   r1, =0x09002800
    str   r1, [r0]

    ldr   r0, =0x1e6e020c
    ldr   r1, =0x55E00B0B
    str   r1, [r0]

    ldr   r0, =0x1e6e0210
    ldr   r1, =0x20000000
    str   r1, [r0]

    ldr   r0, =0x1e6e0214
    ldr   r1, [r5, #ASTMMC_REGIDX_214]
    str   r1, [r0]

    ldr   r0, =0x1e6e02e0
    ldr   r1, [r5, #ASTMMC_REGIDX_2E0]
    str   r1, [r0]

    ldr   r0, =0x1e6e02e4
    ldr   r1, [r5, #ASTMMC_REGIDX_2E4]
    str   r1, [r0]

    ldr   r0, =0x1e6e02e8
    ldr   r1, [r5, #ASTMMC_REGIDX_2E8]
    str   r1, [r0]

    ldr   r0, =0x1e6e02ec
    ldr   r1, [r5, #ASTMMC_REGIDX_2EC]
    str   r1, [r0]

    ldr   r0, =0x1e6e02f0
    ldr   r1, [r5, #ASTMMC_REGIDX_2F0]
    str   r1, [r0]

    ldr   r0, =0x1e6e02f4
    ldr   r1, [r5, #ASTMMC_REGIDX_2F4]
    str   r1, [r0]

    ldr   r0, =0x1e6e02f8
    ldr   r1, [r5, #ASTMMC_REGIDX_2F8]
    str   r1, [r0]

    ldr   r0, =0x1e6e0290
    ldr   r1, =0x00100008
    str   r1, [r0]

    ldr   r0, =0x1e6e02c4
    ldr   r1, =0x3C183C3C
    str   r1, [r0]

    ldr   r0, =0x1e6e02c8
    ldr   r1, =0x00631E0E
    str   r1, [r0]

    ldr   r0, =0x1e6e0034
    ldr   r1, =0x0001A991
    str   r1, [r0]

/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x30                              @ '0'
    str   r1, [r0]
/* Debug - UART console message */

    /********************************************
     Set Ron value to manual mode
     Target to fix DDR CK Vix issue
     Set Ron_pu = 0, Ron_pd = trained value
     *******************************************/
#ifdef ASTMMC_DDR4_MANUAL_RPU
    ldr   r0, =0x1e6e02c0
    ldr   r1, =0x00001806
    str   r1, [r0]
    ldr   r0, =0x1e6e02cc
    ldr   r1, =0x00005050
    str   r1, [r0]
    ldr   r0, =0x1e6e0120
    mov   r1, #0x04
    str   r1, [r0]
    ldr   r0, =0x1e6e0060                        @ Fire DDRPHY Init
    mov   r1, #0x05
    str   r1, [r0]
    b     DdrPhyInitProcess
#endif // place here by ron

Ddr4RonPhyinitDone:

    ldr   r0, =0x1e6e0300                        @ read calibrated Ron_pd
    ldr   r3, [r0]
    bic   r3, r3, #0xFFFFFF0F
    ldr   r0, =0x1e6e0240
    ldr   r1, [r0]
    bic   r1, r1, #0xFF000000
    mov   r2, #ASTMMC_DDR4_MANUAL_RPU
    orr   r1, r1, r2, lsl #24
#ifdef ASTMMC_DDR4_MANUAL_RPD
    mov   r2, #ASTMMC_DDR4_MANUAL_RPD
    orr   r1, r1, r2, lsl #28
#else
    orr   r1, r1, r3, lsl #24
#endif
    orr   r1, r1, #0x02
    str   r1, [r0]

    ldr   r0, =0x1e6e0060                        @ Reset PHY
    mov   r1, #0x00
    str   r1, [r0]
//#endif
    /********************************************
     PHY Vref Scan
     r6 : recorded vref value
     r7 : max read eye pass window
     r8 : passcnt
     r9 : CBRtest result
     r10: loopcnt
     r11: free
    ********************************************/
    ldr   r0, =0x1e720000                        @ retry count
    mov   r1, #0x5
    str   r1, [r0]
Ddr4VrefPhyCalStart:
    mov   r7, #0x0
    mov   r8, #0x0
    mov   r10, #0x3F

    ldr   r0, =0x1e720000
    ldr   r1, [r0]
    subs  r1, r1, #0x01
    beq   DdrTestFail
    str   r1, [r0]

    ldr   r0, =0x1e6e0120
    ldr   r1, =0x00000001
    str   r1, [r0]

/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x61                              @ 'a'
    str   r1, [r0]
/* Debug - UART console message */

    ldr   r0, =0x1e6e02c0
    ldr   r1, =0x00001C06
    str   r1, [r0]

Ddr4VrefPhyLoop:
    ldr   r0, =0x1e6e0060
    ldr   r1, =0x00000000
    str   r1, [r0]

    add   r10, r10, #0x01
    cmp   r10, #0x80
    beq   Ddr4VrefPhyTestFail                @ no valid margin and retry

    ldr   r0, =0x1e6e02cc
    orr   r1, r10, r10, lsl #8
    str   r1, [r0]

    ldr   r0, =0x1e6e0060
    ldr   r1, =0x00000005
    str   r1, [r0]
    b     DdrPhyInitProcess

Ddr4VrefPhyPhyinitDone:

    b     CbrTestStart

Ddr4VrefPhyCbrtestDone:
    ldr   r0, =0x1e6e03d0                        @ read eye pass window
    ldr   r1, [r0]
    ldr   r0, =0x1e720000
    add   r0, r0, r10, lsl #2
    str   r1, [r0]
    cmp   r9, #0x01
    bne   Ddr4VrefPhyTestFail
    add   r8, r8, #0x01
    ldr   r0, =0x1e6e03d0                        @ read eye pass window
    ldr   r1, [r0]
    mov   r2, r1, lsr #8                         @ r2 = DQH
    and   r1, r1, #0xFF                          @ r1 = DQL
    cmp   r1, r2
    movgt r1, r2                                 @ r1 = smaller one
    cmp   r1, r7
    movgt r6, r10
    movgt r7, r1
    b     Ddr4VrefPhyLoop

Ddr4VrefPhyTestFail:
    cmp   r8, #0x0
    bne   Ddr4VrefPhyLoopEnd
    cmp   r10, #0x80
    beq   Ddr4VrefPhyCalStart
    b     Ddr4VrefPhyLoop

Ddr4VrefPhyLoopEnd:
    cmp   r8, #16                                @ check phyvref margin >= 16
    blt   DdrTestFail
    ldr   r0, =0x1e6e02cc
    orr   r1, r6, r6, lsl #8
    str   r1, [r0]
    ldr   r0, =0x1e720010
    orr   r1, r6, r7, lsl #8
    orr   r1, r1, r8, lsl #16
    str   r1, [r0]

    /********************************************
     DDR Vref Scan
     r6 : min
     r7 : max
     r8 : passcnt
     r9 : CBRtest result
     r10: loopcnt
     r11: free
    ********************************************/
    ldr   r0, =0x1e720000                        @ retry count
    mov   r1, #0x5
    str   r1, [r0]
Ddr4VrefDdrCalStart:
    mov   r6, #0xFF
    mov   r7, #0x0
    mov   r8, #0x0
    mov   r10, #0x0

    ldr   r0, =0x1e720000
    ldr   r1, [r0]
    subs  r1, r1, #0x01
    beq   DdrTestFail
    str   r1, [r0]

    ldr   r0, =0x1e6e0120
    ldr   r1, =0x00000002
    str   r1, [r0]

/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x62                              @ 'b'
    str   r1, [r0]
/* Debug - UART console message */

Ddr4VrefDdrLoop:
    ldr   r0, =0x1e6e0060
    ldr   r1, =0x00000000
    str   r1, [r0]

    add   r10, r10, #0x01
    cmp   r10, #0x40
    beq   Ddr4VrefDdrTestFail                @ no valid margin and retry

    ldr   r0, =0x1e6e02c0
    mov   r1, #0x06
    orr   r1, r1, r10, lsl #8
    str   r1, [r0]

    ldr   r0, =0x1e6e0060
    ldr   r1, =0x00000005
    str   r1, [r0]
    b     DdrPhyInitProcess

Ddr4VrefDdrPhyinitDone:

    b     CbrTestStart

Ddr4VrefDdrCbrtestDone:
    cmp   r9, #0x01
    bne   Ddr4VrefDdrTestFail
    add   r8, r8, #0x01
    cmp   r6, r10
    movgt r6, r10
    cmp   r7, r10
    movlt r7, r10
    b     Ddr4VrefDdrLoop

Ddr4VrefDdrTestFail:
    cmp   r8, #0x0
    bne   Ddr4VrefDdrLoopEnd
    cmp   r10, #0x40
    beq   Ddr4VrefDdrCalStart
    b     Ddr4VrefDdrLoop

Ddr4VrefDdrLoopEnd:
    ldr   r0, =0x1e6e0060
    ldr   r1, =0x00000000
    str   r1, [r0]

    cmp   r8, #16                                @ check ddrvref margin >= 16
    blt   DdrTestFail
    ldr   r0, =0x1e6e02c0
    add   r1, r6, r7
    add   r1, r1, #0x01
    mov   r2, r1, lsr #1
    mov   r1, r2, lsl #8
    orr   r1, r1, #0x06
    str   r1, [r0]
    ldr   r0, =0x1e720014
    orr   r1, r6, r7, lsl #8
    orr   r1, r1, r8, lsl #16
    str   r1, [r0]

/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x63                              @ 'c'
    str   r1, [r0]
/* Debug - UART console message */

    ldr   r0, =0x1e6e0120
    ldr   r1, =0x00000003
    str   r1, [r0]

    ldr   r0, =0x1e6e0060                        @ Fire DDRPHY Init
    ldr   r1, =0x00000005
    str   r1, [r0]
    b     DdrPhyInitProcess

Ddr4PhyinitDone:

    /********************************************
     Check Read training margin
    ********************************************/
    ldr   r0, =0x1e6e03a0                        @ check Gate Training Pass Window
    ldr   r1, [r0]
    ldr   r2, =0x150
    bic   r0, r1, #0xFF000000
    bic   r0, r0, #0x00FF0000
    cmp   r0, r2
    blt   DdrTestFail
    mov   r0, r1, lsr #16
    cmp   r0, r2
    blt   DdrTestFail

    ldr   r0, =0x1e6e03d0                        @ check Read Data Eye Training Pass Window
    ldr   r1, [r0]
    ldr   r2, =0x90
    bic   r0, r1, #0x0000FF00
    cmp   r0, r2
    blt   DdrTestFail
    mov   r0, r1, lsr #8
    cmp   r0, r2
    blt   DdrTestFail
    /*******************************************/

    /*******************************************/
/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x31                              @ '1'
    str   r1, [r0]
/* Debug - UART console message */

    ldr   r0, =0x1e6e000c
#ifdef CONFIG_DRAM_EXT_TEMP
    ldr   r1, =0x42AA2F81
#else
    ldr   r1, =0x42AA5C81
#endif
    str   r1, [r0]

    ldr   r0, =0x1e6e0034
    ldr   r1, =0x0001AF93
    str   r1, [r0]

    ldr   r0, =0x1e6e0120                        @ VGA Compatible Mode
    ldr   r1, [r5, #ASTMMC_REGIDX_PLL]
    str   r1, [r0]

    b     CalibrationEnd

.LTORG
/******************************************************************************
 End DDR4 Init
 ******************************************************************************/
/******************************************************************************
 Global Process
 ******************************************************************************/
    /********************************************
     DDRPHY Init Process
    ********************************************/
DdrPhyInitProcess:
    clear_delay_timer
    /* Wait DDR PHY init done - timeout 300 ms */
    ldr   r2, =0x000493E0                        @ Set Timer3 Reload = 300 ms
    init_delay_timer
    ldr   r3, =0x1e6e0060
DdrPhyInit:
    check_delay_timer
    beq   DdrPhyInitTimeout
    ldr   r1, [r3]
    tst   r1, #0x01
    bne   DdrPhyInit

    /* Check DDR PHY init status */
    ldr   r0, =0x1e6e0300
    ldr   r2, =0x000A0000
    ldr   r1, [r0]
    tst   r1, r2
    beq   DdrPhyInitSuccess

DdrPhyInitTimeout:
    ldr   r0, =0x1e6e0060                        @ Reset PHY
    mov   r1, #0x00
    str   r1, [r0]

/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x2E                              @ '.'
    str   r1, [r0]
/* Debug - UART console message */

    clear_delay_timer
    /* Delay about 10us */
    ldr   r2, =0x0000000A                        @ Set Timer3 Reload = 10 us
    init_delay_timer
DdrPhyInitDelay0:
    check_delay_timer
    bne   DdrPhyInitDelay0
    clear_delay_timer
    /* end delay 10us */

    ldr   r0, =0x1e6e0060                        @ Fire PHY Init
    mov   r1, #0x05
    str   r1, [r0]
    b     DdrPhyInitProcess

DdrPhyInitSuccess:
    clear_delay_timer
    ldr   r0, =0x1e6e0060
    mov   r1, #0x06
    str   r1, [r0]

    ldr   r0, =0x1e6e0120
    ldr   r1, [r0]
    cmp   r1, #0
    beq   Ddr3PhyinitDone
    cmp   r1, #1
    beq   Ddr4VrefPhyPhyinitDone
    cmp   r1, #2
    beq   Ddr4VrefDdrPhyinitDone
#ifdef ASTMMC_DDR4_MANUAL_RPU
    cmp   r1, #4
    beq   Ddr4RonPhyinitDone
#endif
    b     Ddr4PhyinitDone

    /********************************************
     CBRTest
    ********************************************/
CbrTestStart:
    ldr   r0, =0x1e6e000c
    ldr   r1, =0x00005C01
    str   r1, [r0]
    ldr   r0, =0x1e6e0074
    ldr   r1, =0x0000FFFF                        @ test size = 64KB
    str   r1, [r0]
    ldr   r0, =0x1e6e007c
    ldr   r1, =0xFF00FF00
    str   r1, [r0]

CbrTestSingle:
    ldr   r0, =0x1e6e0070
    ldr   r1, =0x00000000
    str   r1, [r0]
    ldr   r1, =0x00000085
    str   r1, [r0]
    ldr   r3, =0x3000
    ldr   r11, =0x50000
CbrWaitEngineIdle0:
    subs  r11, r11, #1
    beq   CbrTestFail
    ldr   r2, [r0]
    tst   r2, r3                                 @ D[12] = idle bit
    beq   CbrWaitEngineIdle0

    ldr   r0, =0x1e6e0070                        @ read fail bit status
    ldr   r3, =0x2000
    ldr   r2, [r0]
    tst   r2, r3                                 @ D[13] = fail bit
    bne   CbrTestFail

CbrTestBurst:
    mov   r1, #0x00                              @ initialize loop index, r1 is loop index
CbrTestBurstLoop:
    ldr   r0, =0x1e6e0070
    ldr   r2, =0x00000000
    str   r2, [r0]
    mov   r2, r1, lsl #3
    orr   r2, r2, #0xC1                          @ test command = 0xC1 | (datagen << 3)
    str   r2, [r0]
    ldr   r3, =0x3000
    ldr   r11, =0x20000
CbrWaitEngineIdle1:
    subs  r11, r11, #1
    beq   CbrTestFail
    ldr   r2, [r0]
    tst   r2, r3                                 @ D[12] = idle bit
    beq   CbrWaitEngineIdle1

    ldr   r0, =0x1e6e0070                        @ read fail bit status
    ldr   r3, =0x2000
    ldr   r2, [r0]
    tst   r2, r3                                 @ D[13] = fail bit
    bne   CbrTestFail

    add   r1, r1, #1                             @ increase the test mode index
    cmp   r1, #0x04                              @ test 4 modes
    bne   CbrTestBurstLoop

    ldr   r0, =0x1e6e0070
    ldr   r1, =0x00000000
    str   r1, [r0]
    mov   r9, #0x1
    b     CbrTestPatternEnd                   @ CBRTest() return(1)

CbrTestFail:
    ldr   r0, =0x1e6e0070
    ldr   r1, =0x00000000
    str   r1, [r0]
    mov   r9, #0x0                               @ CBRTest() return(0)

CbrTestPatternEnd:
    ldr   r0, =0x1e6e000c
    ldr   r1, =0x00000000
    str   r1, [r0]
    ldr   r0, =0x1e6e0120
    ldr   r1, [r0]
    cmp   r1, #1
    beq   Ddr4VrefPhyCbrtestDone
    b     Ddr4VrefDdrCbrtestDone

.LTORG
/******************************************************************************
 Other features configuration
 *****************************************************************************/

CalibrationEnd:

    /*******************************
     Check DRAM Size
     1Gb : 0x80000000 ~ 0x87FFFFFF
     2Gb : 0x80000000 ~ 0x8FFFFFFF
     4Gb : 0x80000000 ~ 0x9FFFFFFF
     8Gb : 0x80000000 ~ 0xBFFFFFFF
    *******************************/
    ldr   r0, =0x1e6e0004
    ldr   r6, [r0]
    bic   r6, r6, #0x00000003                    @ record MCR04
    ldr   r7, [r5, #ASTMMC_REGIDX_RFC]

CheckDramSize:
    ldr   r0, =0xA0100000
    ldr   r1, =0x41424344
    str   r1, [r0]
    ldr   r0, =0x90100000
    ldr   r1, =0x35363738
    str   r1, [r0]
    ldr   r0, =0x88100000
    ldr   r1, =0x292A2B2C
    str   r1, [r0]
    ldr   r0, =0x80100000
    ldr   r1, =0x1D1E1F10
    str   r1, [r0]
    ldr   r0, =0xA0100000
    ldr   r1, =0x41424344
    ldr   r2, [r0]
    cmp   r2, r1                                 @ == 8Gbit
    orreq r6, r6, #0x03
    moveq r7, r7, lsr #24
    mov   r3, #0x38                              @ '8'
    beq   CheckDramSizeEnd
    ldr   r0, =0x90100000
    ldr   r1, =0x35363738
    ldr   r2, [r0]
    cmp   r2, r1                                 @ == 4Gbit
    orreq r6, r6, #0x02
    moveq r7, r7, lsr #16
    mov   r3, #0x34                              @ '4'
    beq   CheckDramSizeEnd
    ldr   r0, =0x88100000
    ldr   r1, =0x292A2B2C
    ldr   r2, [r0]
    cmp   r2, r1                                 @ == 2Gbit
    orreq r6, r6, #0x01
    moveq r7, r7, lsr #8
    mov   r3, #0x32                              @ '2'
    beq   CheckDramSizeEnd
    mov   r3, #0x31                              @ '1'

CheckDramSizeEnd:
    ldr   r0, =0x1e6e0004
    str   r6, [r0]
    ldr   r0, =0x1e6e0014
    ldr   r1, [r0]
    bic   r1, r1, #0x000000FF
    and   r7, r7, #0xFF
    orr   r1, r1, r7
    str   r1, [r0]

    /* Version Number */
    ldr   r0, =0x1e6e0004
    ldr   r1, [r0]
    mov   r2, #AstMmcInitVer
    orr   r1, r1, r2, lsl #20
    str   r1, [r0]

    ldr   r0, =0x1e6e0088
    ldr   r1, =ASTMMC_INIT_DATE
    str   r1, [r0]

/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x2D                              @ '-'
    str   r1, [r0]
    str   r3, [r0]
    mov   r1, #0x47                              @ 'G'
    str   r1, [r0]
    mov   r1, #0x62                              @ 'b'
    str   r1, [r0]
    mov   r1, #0x2D                              @ '-'
    str   r1, [r0]
/* Debug - UART console message */

    /* Enable DRAM Cache */
    ldr   r0, =0x1e6e0004
    ldr   r1, [r0]
    mov   r2, #1
    orr   r2, r1, r2, lsl #12
    str   r2, [r0]
    ldr   r3, =0x00080000
DramCacheInit:
    ldr   r2, [r0]
    tst   r2, r3
    beq   DramCacheInit
    mov   r2, #1
    orr   r1, r1, r2, lsl #10
    str   r1, [r0]

    /* Set DRAM requests threshold */
    ldr   r0, =0x1e6e001c
    ldr   r1, =0x00000008
    str   r1, [r0]
    ldr   r0, =0x1e6e0038
    ldr   r1, =0xFFFFFF00
    str   r1, [r0]

    /********************************************
     DDRTest
    ********************************************/
DdrTestStart:
    ldr   r0, =0x1e6e0074
    ldr   r1, =0x0000FFFF                        @ test size = 64KB
    str   r1, [r0]
    ldr   r0, =0x1e6e007c
    ldr   r1, =0xFF00FF00
    str   r1, [r0]

DdrTestBurst:
    mov   r1, #0x00                              @ initialize loop index, r1 is loop index
DdrTestBurstLoop:
    ldr   r0, =0x1e6e0070
    ldr   r2, =0x00000000
    str   r2, [r0]
    mov   r2, r1, lsl #3
    orr   r2, r2, #0xC1                          @ test command = 0xC1 | (datagen << 3)
    str   r2, [r0]
    ldr   r3, =0x3000
    ldr   r11, =0x20000
DdrWaitEngineIdle1:
    subs  r11, r11, #1
    beq   DdrTestFail
    ldr   r2, [r0]
    tst   r2, r3                                 @ D[12] = idle bit
    beq   DdrWaitEngineIdle1

    ldr   r0, =0x1e6e0070                        @ read fail bit status
    ldr   r3, =0x2000
    ldr   r2, [r0]
    tst   r2, r3                                 @ D[13] = fail bit
    bne   DdrTestFail

    add   r1, r1, #1                             @ increase the test mode index
    cmp   r1, #0x01                              @ test 1 modes
    bne   DdrTestBurstLoop

    ldr   r0, =0x1e6e0070
    ldr   r1, =0x00000000
    str   r1, [r0]
    b     SetScratch                            @ CBRTest() return(1)

DdrTestFail:
/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x46                              @ 'F'
    str   r1, [r0]
    mov   r1, #0x61                              @ 'a'
    str   r1, [r0]
    mov   r1, #0x69                              @ 'i'
    str   r1, [r0]
    mov   r1, #0x6C                              @ 'l'
    str   r1, [r0]
    mov   r1, #0x0D                              @ '\r'
    str   r1, [r0]
    mov   r1, #0x0A                              @ '\n'
    str   r1, [r0]
    ldr   r0, =0x1e784014
WaitPrint0:
    ldr   r1, [r0]
    tst   r1, #0x40
    beq   WaitPrint0
/* Debug - UART console message */
    b     ResetMmc

SetScratch:
    /*Set Scratch register Bit 6 after ddr initial finished */
    ldr   r0, =0x1e6e2040
    ldr   r1, [r0]
    orr   r1, r1, #0x41
    str   r1, [r0]

/* Debug - UART console message */
    ldr   r0, =0x1e784000
    mov   r1, #0x44                              @ 'D'
    str   r1, [r0]
    mov   r1, #0x6F                              @ 'o'
    str   r1, [r0]
    mov   r1, #0x6E                              @ 'n'
    str   r1, [r0]
    mov   r1, #0x65                              @ 'e'
    str   r1, [r0]
    mov   r1, #0x0D                              @ '\r'
    str   r1, [r0]
    mov   r1, #0x0A                              @ '\n'
    str   r1, [r0]
/* Debug - UART console message */

    /* Enable VGA display */
    ldr   r0, =0x1e6e202c
    ldr   r1, [r0]
    bic   r1, r1, #0x40
    str   r1, [r0]

/* Debug - UART console message */
   /* Print PHY timing information */
    ldr   r0, =0x1e784014
WaitPrint1:
    ldr   r1, [r0]
    tst   r1, #0x40
    beq   WaitPrint1

    ldr   r0, =0x1e784000
    mov   r1, #0x52                              @ 'R'
    str   r1, [r0]
    mov   r1, #0x65                              @ 'e'
    str   r1, [r0]
    mov   r1, #0x61                              @ 'a'
    str   r1, [r0]
    mov   r1, #0x64                              @ 'd'
    str   r1, [r0]
    mov   r1, #0x20                              @ ' '
    str   r1, [r0]
    mov   r1, #0x6D                              @ 'm'
    str   r1, [r0]
    mov   r1, #0x61                              @ 'a'
    str   r1, [r0]
    mov   r1, #0x72                              @ 'r'
    str   r1, [r0]
    mov   r1, #0x67                              @ 'g'
    str   r1, [r0]
    mov   r1, #0x69                              @ 'i'
    str   r1, [r0]
    mov   r1, #0x6E                              @ 'n'
    str   r1, [r0]
    mov   r1, #0x2D                              @ '-'
    str   r1, [r0]
    mov   r1, #0x44                              @ 'D'
    str   r1, [r0]
    mov   r1, #0x4C                              @ 'L'
    str   r1, [r0]
    mov   r1, #0x3A                              @ ':'
    str   r1, [r0]

    ldr   r0, =0x1e784014
WaitPrint2:
    ldr   r1, [r0]
    tst   r1, #0x40
    beq   WaitPrint2

    ldr   r7, =0x000001FE                        @ divide by 510
    mov   r8, #10                                @ multiply by 10
    mov   r9, #0                                 @ record violation
    ldr   r0, =0x1e6e0004
    ldr   r1, [r0]
    tst   r1, #0x10                              @ bit[4]=1 => DDR4
    movne r10, #0x9A                             @ DDR4 min = 0x99 (0.30)
    moveq r10, #0xB3                             @ DDR3 min = 0xB3 (0.35)
PrintDQLEyeMargin:
    ldr   r0, =0x1e6e03d0
    ldr   r2, [r0]
    and   r2, r2, #0xFF
    cmp   r2, r10                                @ check violation
    movlt r9, #1
    ldr   r0, =0x1e784000
    mov   r1, #0x30                              @ '0'
    str   r1, [r0]
    mov   r1, #0x2E                              @ '.'
    str   r1, [r0]
    mov   r3, #0x4                               @ print 4 digits
PrintDQLDivLoop:
    mul   r2, r8, r2
    cmp   r2, r7
    blt   PrintDQLDiv0
    mov   r6, #0x0
PrintDQLDivDigit:
    sub   r2, r2, r7
    add   r6, r6, #0x1
    cmp   r2, r7
    bge   PrintDQLDivDigit
    b     PrintDQLDivN

PrintDQLDiv0:
    mov   r1, #0x30                              @ '0'
    str   r1, [r0]
    b     PrintDQLNext
PrintDQLDivN:
    add   r1, r6, #0x30                          @ print n
    str   r1, [r0]
PrintDQLNext:
    subs  r3, r3, #1
    beq   PrintDQHEyeMargin
    cmp   r2, #0x0
    beq   PrintDQHEyeMargin
    b     PrintDQLDivLoop

PrintDQHEyeMargin:
    mov   r1, #0x2F                              @ '/'
    str   r1, [r0]
    mov   r1, #0x44                              @ 'D'
    str   r1, [r0]
    mov   r1, #0x48                              @ 'H'
    str   r1, [r0]
    mov   r1, #0x3A                              @ ':'
    str   r1, [r0]

    ldr   r0, =0x1e784014
WaitPrint3:
    ldr   r1, [r0]
    tst   r1, #0x40
    beq   WaitPrint3

    ldr   r0, =0x1e6e03d0
    ldr   r2, [r0]
    mov   r2, r2, lsr #8
    and   r2, r2, #0xFF
    cmp   r2, r10                                @ check violation
    movlt r9, #1
    ldr   r0, =0x1e784000
    mov   r1, #0x30                              @ '0'
    str   r1, [r0]
    mov   r1, #0x2E                              @ '.'
    str   r1, [r0]
    mov   r3, #0x4                               @ print 4 digits
PrintDQHDivLoop:
    mul   r2, r8, r2
    cmp   r2, r7
    blt   PrintDQHDiv0
    mov   r6, #0x0
PrintDQHDivDigit:
    sub   r2, r2, r7
    add   r6, r6, #0x1
    cmp   r2, r7
    bge   PrintDQHDivDigit
    b     PrintDQHDivN

PrintDQHDiv0:
    mov   r1, #0x30                              @ '0'
    str   r1, [r0]
    b     PrintDQHNext
PrintDQHDivN:
    add   r1, r6, #0x30                          @ print n
    str   r1, [r0]
PrintDQHNext:
    subs  r3, r3, #1
    beq   PrintDQEyeMarginLast
    cmp   r2, #0x0
    beq   PrintDQEyeMarginLast
    b     PrintDQHDivLoop

PrintDQEyeMarginLast:
    mov   r1, #0x20                              @ ' '
    str   r1, [r0]
    mov   r1, #0x43                              @ 'C'
    str   r1, [r0]
    mov   r1, #0x4B                              @ 'K'
    str   r1, [r0]

    ldr   r0, =0x1e6e0004
    ldr   r1, [r0]
    tst   r1, #0x10                              @ bit[4]=1 => DDR4
    movne r10, #0x30                             @ DDR4 min = 0.30
    moveq r10, #0x35                             @ DDR4 min = 0.35

    ldr   r0, =0x1e784014
WaitPrint4:
    ldr   r1, [r0]
    tst   r1, #0x40
    beq   WaitPrint4

    ldr   r0, =0x1e784000
    mov   r1, #0x20                              @ ' '
    str   r1, [r0]
    mov   r1, #0x28                              @ '('
    str   r1, [r0]
    mov   r1, #0x6D                              @ 'm'
    str   r1, [r0]
    mov   r1, #0x69                              @ 'i'
    str   r1, [r0]
    mov   r1, #0x6E                              @ 'n'
    str   r1, [r0]
    mov   r1, #0x3A                              @ ':'
    str   r1, [r0]
    mov   r1, #0x30                              @ '0'
    str   r1, [r0]
    mov   r1, #0x2E                              @ '.'
    str   r1, [r0]
    mov   r1, #0x33                              @ '3'
    str   r1, [r0]
    str   r10, [r0]
    mov   r1, #0x29                              @ ')'
    str   r1, [r0]

    cmp   r9, #0
    beq   PrintDQMarginLast
    mov   r1, #0x20                              @ ' '
    str   r1, [r0]
    ldr   r0, =0x1e784014
WaitPrint5:
    ldr   r1, [r0]
    tst   r1, #0x40
    beq   WaitPrint5

    ldr   r0, =0x1e784000
    mov   r1, #0x57                              @ 'W'
    str   r1, [r0]
    mov   r1, #0x61                              @ 'a'
    str   r1, [r0]
    mov   r1, #0x72                              @ 'r'
    str   r1, [r0]
    mov   r1, #0x6E                              @ 'n'
    str   r1, [r0]
    mov   r1, #0x69                              @ 'i'
    str   r1, [r0]
    mov   r1, #0x6E                              @ 'n'
    str   r1, [r0]
    mov   r1, #0x67                              @ 'g'
    str   r1, [r0]
    mov   r1, #0x3A                              @ ':'
    str   r1, [r0]
    mov   r1, #0x20                              @ ' '
    str   r1, [r0]
    mov   r1, #0x4D                              @ 'M'
    str   r1, [r0]
    mov   r1, #0x61                              @ 'a'
    str   r1, [r0]
    mov   r1, #0x72                              @ 'r'
    str   r1, [r0]
    mov   r1, #0x67                              @ 'g'
    str   r1, [r0]
    mov   r1, #0x69                              @ 'i'
    str   r1, [r0]
    mov   r1, #0x6E                              @ 'n'
    str   r1, [r0]
    ldr   r0, =0x1e784014
WaitPrint6:
    ldr   r1, [r0]
    tst   r1, #0x40
    beq   WaitPrint6
    ldr   r0, =0x1e784000
    mov   r1, #0x20                              @ ' '
    str   r1, [r0]
    mov   r1, #0x74                              @ 't'
    str   r1, [r0]
    mov   r1, #0x6F                              @ 'o'
    str   r1, [r0]
    mov   r1, #0x6F                              @ 'o'
    str   r1, [r0]
    mov   r1, #0x20                              @ ' '
    str   r1, [r0]
    mov   r1, #0x73                              @ 's'
    str   r1, [r0]
    mov   r1, #0x6D                              @ 'm'
    str   r1, [r0]
    mov   r1, #0x61                              @ 'a'
    str   r1, [r0]
    mov   r1, #0x6C                              @ 'l'
    str   r1, [r0]
    mov   r1, #0x6C                              @ 'l'
    str   r1, [r0]

PrintDQMarginLast:
    mov   r1, #0x0D                              @ '\r'
    str   r1, [r0]
    mov   r1, #0x0A                              @ '\n'
    str   r1, [r0]
/* Debug - UART console message */

PlatformExit:
#ifdef CONFIG_DRAM_ECC
    ldr   r0, =0x1e6e0004
    ldr   r2, =0x00000880                        @ add cache range control, 2016.09.02
    ldr   r1, [r0]
    orr   r1, r1, r2
    str   r1, [r0]

    ldr   r0, =0x1e6e0054
    ldr   r1, =ConfigDramEccSize              /* ECC protected memory size */
    str   r1, [r0]

    ldr   r0, =0x1e6e007C
    ldr   r1, =0x00000000
    str   r1, [r0]
    ldr   r0, =0x1e6e0074
    str   r1, [r0]

    ldr   r0, =0x1e6e0070
    ldr   r1, =0x00000221
    str   r1, [r0]

    ldr   r2, =0x00001000
#endif
b EccInitFlag

EccInitFlag:
    ldr   r1, [r0]
    tst   r1, r2                                 @ D[12] = 1, Done
    beq   EccInitFlag

    ldr   r1, =0x00000000
    str   r1, [r0]

    ldr   r0, =0x1e6e0050
    ldr   r1, =0x80000000
    str   r1, [r0]

    ldr   r0, =0x1e6e0050
    ldr   r1, =0x00000000
    str   r1, [r0]

    ldr   r0, =0x1e6e0070
    ldr   r1, =0x00000400                        @ Enable ECC auto-scrubbing
    str   r1, [r0]
//#endif

/******************************************************************************
 SPI Timing Calibration
 ******************************************************************************/
    mov   r2, #0x0
    mov   r6, #0x0
    mov   r7, #0x0
    init_spi_checksum
SpiChecksumWait0:
    ldr   r1, [r0]
    tst   r1, r2
    beq   SpiChecksumWait0
    ldr   r0, =0x1e620090
    ldr   r5, [r0]                               @ record golden checksum
    ldr   r0, =0x1e620080
    mov   r1, #0x0
    str   r1, [r0]

    ldr   r0, =0x1e620010                        @ set to fast read mode
    ldr   r1, =0x000B0041
    str   r1, [r0]

    ldr   r6, =0x00F7E6D0                        @ Init spiclk loop
    mov   r8, #0x0                               @ Init delay record

SpiCbrNextClkrate:
    mov   r6, r6, lsr #0x4
    cmp   r6, #0x0
    beq   SpiCbrEnd

    mov   r7, #0x0                               @ Init delay loop
    mov   r8, r8, lsl #4

SpiCbrNextDelayS:
    mov   r2, #0x8
    init_spi_checksum
SpiChecksumWait1:
    ldr   r1, [r0]
    tst   r1, r2
    beq   SpiChecksumWait1
    ldr   r0, =0x1e620090
    ldr   r2, [r0]                               @ read checksum
    ldr   r0, =0x1e620080
    mov   r1, #0x0
    str   r1, [r0]
    cmp   r2, r5
    bne   SpiCbrNextDelayE

    mov   r2, #0x0
    init_spi_checksum
SpiChecksumWait2:
    ldr   r1, [r0]
    tst   r1, r2
    beq   SpiChecksumWait2
    ldr   r0, =0x1e620090
    ldr   r2, [r0]                               @ read checksum
    ldr   r0, =0x1e620080
    mov   r1, #0x0
    str   r1, [r0]
    cmp   r2, r5
    bne   SpiCbrNextDelayE

    orr   r8, r8, r7                             @ record passed delay
    b     SpiCbrNextClkrate

SpiCbrNextDelayE:
    add   r7, r7, #0x1
    cmp   r7, #0x6
    blt   SpiCbrNextDelayS
    b     SpiCbrNextClkrate

SpiCbrEnd:
    ldr   r0, =0x1e620094
    str   r8, [r0]
    ldr   r0, =0x1e620010
    mov   r1, #0x0
    str   r1, [r0]

/******************************************************************************
 Miscellaneous Setting
 ******************************************************************************/
    /* Set UART DMA as AHB high priority master */
    ldr   r0, =0x1e600000
    ldr   r1, =0xAEED1A03
    str   r1, [r0]

    ldr   r0, =0x1e600080
    ldr   r2, =0x100
    ldr   r1, [r0]
    orr   r1, r1, r2
    str   r1, [r0]

    /* Enable UART3/4 clock and disable LHCLK */
    ldr   r0, =0x1e6e200c
    ldr   r1, [r0]
    ldr   r2, =0xF9FFFFFF
    and   r1, r1, r2
    ldr   r2, =0x10000000
    orr   r1, r1, r2
    str   r1, [r0]

    ldr   r0, =0x1e6e2008                        @ Set Video ECLK phase
    ldr   r1, [r0]
    ldr   r2, =0x0ffffff3
    and   r1, r1, r2
    str   r1, [r0]

    ldr r0, =0x1e6e2004                          @ Enable JTAG Master, solve ARM stucked by JTAG issue
    ldr r1, [r0]
    bic r1, r1, #0x00400000
    str r1, [r0]

/******************************************************************************
 Configure MAC timing
 ******************************************************************************/
    /* Enable D2PLL and set to 250MHz */
    ldr   r0, =0x1e6e213c
    ldr   r1, =0x00000585                        @ Reset D2PLL
    str   r1, [r0]

    ldr   r0, =0x1e6e202c
    ldr   r1, [r0]
    bic   r1, r1, #0x10                          @ Enable D2PLL
    ldr   r2, =0x00200000                        @ Set CRT = 40MHz
    orr   r1, r1, r2
    str   r1, [r0]

    ldr   r2, =0x8E00A17C                        @ Set to 250MHz

    ldr   r0, =0x1e6e2070                        @ Check CLKIN = 25MHz
    ldr   r1, [r0]
    mov   r1, r1, lsr #23
    tst   r1, #0x01
    beq   SetD2PLL
    ldr   r2, =0x8E00A177

SetD2PLL:
    ldr   r0, =0x1e6e201c
    str   r2, [r0]
    ldr   r0, =0x1e6e213c                        @ Enable D2PLL
    ldr   r1, =0x00000580
    str   r1, [r0]

    ldr   r0, =0x1e6e204c
    ldr   r1, [r0]
    bic   r1, r1, #0xFF0000
    ldr   r2, =0x00040000                        @ Set divider ratio
    orr   r1, r1, r2
    str   r1, [r0]

    ldr   r0, =0x1e6e2048                        @ Set MAC interface delay timing = 1G
    ldr   r1, =0x80082208                        @ Select internal 125MHz
    str   r1, [r0]
    ldr   r0, =0x1e6e20b8                        @ Set MAC interface delay timing = 100M
    str   r1, [r0]
    ldr   r0, =0x1e6e20bc                        @ Set MAC interface delay timing = 10M
    str   r1, [r0]

    ldr   r0, =0x1e6e2070                        @ Set MAC AHB bus clock
    ldr   r1, [r0]
    mov   r2, #0x04                              @ Default RMII, set MHCLK = HPLL/10
    tst   r1, #0xC0
    movne r2, #0x02                              @ if RGMII,     set MHCLK = HPLL/6
    ldr   r0, =0x1e6e2008
    ldr   r1, [r0]
    bic   r1, r1, #0x00070000
    orr   r1, r1, r2, lsl #16
    str   r1, [r0]

    ldr   r0, =0x1e6e21dc                        @ Set MAC duty
    ldr   r1, =0x00666400
    str   r1, [r0]

    ldr   r0, =0x1e6e2090                        @ Enable MAC interface pull low
    ldr   r1, [r0]
    bic   r1, r1, #0x0000F000
    bic   r1, r1, #0x20000000                    @ Set USB portA as Device mode
    str   r1, [r0]

/* Test - DRAM initial time */
    ldr   r0, =0x1e782040
    ldr   r1, [r0]
    ldr   r0, =0xFFFFFFFF
    sub   r1, r0, r1
    ldr   r0, =0x1e6e008c
    str   r1, [r0]
    ldr   r0, =0x1e78203c
    ldr   r1, =0x0000F000
    str   r1, [r0]
/* Test - DRAM initial time */

    ldr   r0, =0x1e6e0000                        @ disable MMC password
    mov   r1, #0x0
    str   r1, [r0]

    /* Disable Timer separate mode */
    ldr   r0, =0x1e782038
    ldr   r1, =0xEA
    str   r1, [r0]

	b Exit
    /* restore lr */
    mov   lr, r4

    /* back to arch calling code */
    mov   pc, lr

// HERE works
