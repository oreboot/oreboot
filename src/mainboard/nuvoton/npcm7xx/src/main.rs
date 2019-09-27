#![feature(asm)]
#![feature(lang_items, start)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![deny(warnings)]

//use core::fmt;
//use core::fmt::Write;
use model::Driver;
//use print;
//use wrappers::DoD;
use uart::npcm::NPCM;
//use core::ptr;


pub const CLK_BASE: u32 = 0xF0801000;
pub const GCR_BASE: u32 = 0xF0800000;
pub const IPSRST1: u32 = CLK_BASE + 0x20;
pub const MFSEL1: u32 = GCR_BASE + 0x00C;
pub const MFSEL4: u32 = GCR_BASE + 0x0B0;
pub const SPSWC: u32 = GCR_BASE + 0x038;
pub const IPSRST1_UART01: u32 = 11;
pub const IPSRST1_UART23: u32 = 7;
pub const MFSEL4_BSPASEL : u32 = 1;
pub const MFSEL1_HSI2SEL : u32 = 11;
pub const SPSWC_SPMOD : u32 = 0;

//#[inline(always)]
//fn poke(pointer: u32, value: u32) -> () {
//    let addr = pointer as *mut u32;
//    unsafe {
//        ptr::write_volatile(addr, value);
//    }
//}
//
//#[inline(always)]
//fn set(pointer: u32, value: u32) -> () {
//    let v = peek(pointer);
//    poke(pointer, v | value);
//}
//
//#[inline(always)]
//fn clr(pointer: u32, value: u32) -> () {
//    let v = peek(pointer);
//    poke(pointer, v & value);
//}
//
//#[inline(always)]
//fn peek(pointer: u32) -> u32 {
//    let addr = pointer as *const u32;
//    unsafe { ptr::read_volatile(addr) }
//}


#[no_mangle]
#[link_section = ".bootblock.start"]
pub extern "C" fn _start() -> ! {
/*
   TODO: Rustify this:

   UINT32 FCR_Val      = 0;                                                       
   UART_DEV_T devNum = 0;                                                         
                                                                                  
 #if 0 // Seems to be needed sometimes, but not always. Probably only when temperature etc. has changed a lot
   // Reset UART                                                                  
   SET_REG_FIELD(IPSRST1, IPSRST1_UART01, 1);                                     
   SET_REG_FIELD(IPSRST1, IPSRST1_UART01, 0);                                     
                                                                                  
   SET_REG_FIELD(IPSRST1, IPSRST1_UART23, 1);                                     
   SET_REG_FIELD(IPSRST1, IPSRST1_UART23, 0);                                     
                                                                                  
   CLK_Delay_MicroSec(100);                                                       
                                                                                  
   // Set UART0 routing                                                           
   CHIP_Mux_Uart(0 /* uartNumber */, TRUE, FALSE, TRUE);                          
                                                                                  
   /*-----------------------------------------------------------------------------------------------------*/
   /* Disable interrupts                                                                                  */
   /*-----------------------------------------------------------------------------------------------------*/
   REG_WRITE(LCR(devNum), 0);            // prepare to Init UART                  
   REG_WRITE(IER(devNum), 0x0);          // Disable all UART interrupt            
 #endif                                                                           
                                                                                  
	// NOTE: This inits the UART CLK as well it seems
   UART_SetBaudrate(devNum, 115200);                                              
   UART_SetBitsPerChar(devNum, 8);                                                
   UART_SetStopBit(devNum, UART_STOPBIT_1);                                       
   UART_SetParity(devNum, UART_PARITY_NONE);                                      
*/

    //set(IPSRST1, 1 << IPSRST1_UART01);
    //clr(IPSRST1, !(1 << IPSRST1_UART01));

    //set(IPSRST1, 1 << IPSRST1_UART23);
    //clr(IPSRST1, !(1 << IPSRST1_UART23));

    // CHIP Mux Uart to enable sp2 for SI2
    //
    // 0 0 0: Mode 1 - HSP1 connected to SI2  , HSP2 connected to UART2 ,UART1 snoops HSP1, UART3 snoops SI2
    // 0 0 1: Mode 2 - HSP1 connected to UART1, HSP2 connected to SI2   ,UART2 snoops HSP2, UART3 snoops SI2
    // 0 1 0: Mode 3 - HSP1 connected to UART1, HSP2 connected to UART2 ,UART3 connected to SI2
    // 0 1 1: Mode 4 - HSP1 connected to SI1  , HSP2 connected to SI2   ,UART1 snoops SI1,  UART3 snoops SI2,   UART2 snoops HSP1 (default)
    // 1 0 0: Mode 5 - HSP1 connected to SI1  , HSP2 connected to UART2 ,UART1 snoops HSP1, UART3 snoops SI1
    // 1 0 1: Mode 6 - HSP1 connected to SI1  , HSP2 connected to SI2   ,UART1 snoops SI1,  UART3 snoops SI2,   UART2 snoops HSP2
    // 1 1 0: Mode 7 - HSP1 connected to SI1  , HSP2 connected to UART2 ,UART1 snoops HSP1, UART3 connected to SI2
    // - SET_REG_FIELD(SPSWC, SPSWC_SPMOD, value_from_array_above & 0x7)
    //clr(SPSWC, !(7 << SPSWC_SPMOD));
    //set(SPSWC, (0 /* mode */) << SPSWC_SPMOD);

    //
    // This enables TX2
    // - SET_REG_FIELD(MFSEL1, MFSEL1_HSI2SEL, 1)
    // - SET_REG_FIELD(MFSEL4, MFSEL4_BSPASEL, 0)
    //set(MFSEL1, 1 << MFSEL1_HSI2SEL);
    //clr(MFSEL4, !(1 << MFSEL4_BSPASEL));

    //
    //  if (STRP_get(STRP_11_BSPA) == 0)
    //  {
    //      SET_REG_FIELD(MFSEL4, MFSEL4_BSPASEL, 1);
    //      // Note: If this bit is set, MFSEL1 bit 9 and 11 must be set to 0.
    //      SET_REG_FIELD(MFSEL1, MFSEL1_BSPSEL, 0);
    //      SET_REG_FIELD(MFSEL1, MFSEL1_HSI2SEL, 0);
    //  }
    //  else
    //  {
    //      SET_REG_FIELD(MFSEL4, MFSEL4_BSPASEL, 0);
    //      SET_REG_FIELD(MFSEL1, MFSEL1_BSPSEL, 1);
    //  }
    let uart0 = &mut NPCM::new(0xF0001000, 115200);
    uart0.init();
    uart0.pwrite(b"Welcome to oreboot\r\n", 0).unwrap();

    loop {
        uart0.pwrite(b"Ping\r\n", 0).unwrap();
    }
}

// This function is called on panic.
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
