#![doc = include_str!("README.md")]
#![feature(naked_functions, asm_sym, asm_const)]
#![feature(default_alloc_error_handler)]
#![feature(int_abs_diff)]
#![no_std]
#![no_main]

use core::intrinsics::transmute;
use core::ptr::{read_volatile, write_volatile};
use core::{arch::asm, panic::PanicInfo};
use embedded_hal::digital::blocking::OutputPin;
use oreboot_soc::sunxi::d1::pac::{ccu::smhc0_clk, CCU};
use oreboot_soc::sunxi::d1::{
    ccu::Clocks,
    gpio::{portc, Function, Gpio, Pin},
    jtag::Jtag,
    pac::{Peripherals, SMHC0, SPI0},
    spi::{Spi, MODE_3},
    time::U32Ext,
    uart::{Config, Parity, Serial, StopBits, WordLength},
};

#[macro_use]
mod logging;
mod flash;
mod mctl;

#[cfg(feature = "nand")]
use flash::SpiNand;
#[cfg(feature = "nor")]
use flash::SpiNor;
use mctl::RAM_BASE;

// taken from oreboot
pub type EntryPoint = unsafe extern "C" fn(r0: usize, r1: usize);

const STACK_SIZE: usize = 1 * 1024; // 1KiB

#[link_section = ".bss.uninit"]
static mut BT0_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

/// Jump over head data to executable code.
///
/// # Safety
///
/// Naked function.
#[naked]
#[link_section = ".head.text"]
#[export_name = "head_jump"]
pub unsafe extern "C" fn head_jump() {
    asm!(
        ".option push",
        ".option rvc",
        "c.j    0x68", // 0x60: eGON.BT0 header; 0x08: FlashHead
        ".option pop",
        // sym start,
        options(noreturn)
    )
}

// todo: option(noreturn) generates an extra `unimp` insn

// eGON.BT0 header. This header is identified by D1 ROM code
// to copy BT0 stage bootloader into SRAM memory.
// This header takes 0x60 bytes.
#[repr(C)]
pub struct EgonHead {
    magic: [u8; 8],
    checksum: u32,
    length: u32,
    pub_head_size: u32,
    fel_script_address: u32,
    fel_uenv_length: u32,
    dt_name_offset: u32,
    dram_size: u32,
    boot_media: u32,
    string_pool: [u32; 13],
}

const STAMP_CHECKSUM: u32 = 0x5F0A6C39;

// TODO: determine offets/sizes at build time
// memory load addresses
const LIN_ADDR: usize = RAM_BASE + 0x0400_0000; // Linux will be decompressed in payloader
const DTB_ADDR: usize = RAM_BASE + 0x01a0_0000; // dtb must be 2MB aligned and behind Linux
const ORE_ADDR: usize = RAM_BASE;
const ORE_SIZE: usize = 0x1_8000; // 96K
const DTF_SIZE: usize = 0x1_0000; // 64K
const LIN_SIZE: usize = 0x00fc_0000;
const DTB_SIZE: usize = 0x0001_0000;

// clobber used by KEEP(*(.head.egon)) in link script
#[link_section = ".head.egon"]
pub static EGON_HEAD: EgonHead = EgonHead {
    magic: *b"eGON.BT0",
    checksum: STAMP_CHECKSUM, // real checksum filled by blob generator
    length: 0,                // real size filled by blob generator
    pub_head_size: 0,
    fel_script_address: 0,
    fel_uenv_length: 0,
    dt_name_offset: 0,
    dram_size: 0,
    boot_media: 0,
    string_pool: [0; 13],
};

// Private use; not designed as conventional header structure.
// Real data filled by xtask.
// This header takes 0x8 bytes. When modifying this structure, make sure
// the offset in `head_jump` function is also modified.
#[repr(C)]
pub struct MainStageHead {
    offset: u32,
    length: u32,
}

// clobber used by KEEP(*(.head.main)) in link script
// To avoid optimization, always read from flash page. Do NOT use this
// variable directly.
#[link_section = ".head.main"]
pub static MAIN_STAGE_HEAD: MainStageHead = MainStageHead {
    offset: 0, // real offset filled by xtask
    length: 0, // real size filled by xtask
};

/*
see https://linux-sunxi.org/D1 for the C906 manual
original document is in Chinese, translated by Google here for reference

p19:

The C906 supports two types of memory storage, namely memory and peripherals
(distinguished by the SO bit). Among them, the memory type is further divided
into cacheable memory (Cacheable memory) and non-cacheable memory (Non-cacheable
memory) according to whether it can be cached (Cacheable, C). The feature of
device type is that speculative execution is not allowed, so device must have
the property of not being cached. device is divided into bufferable device
(Bufferable device) and non-bufferable device (Non-bufferable device)
according to whether it can be buffered (Bufferable, B). The node returns the
write response quickly; non-cacheable means that it will only return after the
slave device is actually written. Write the response.

Table 3.11 shows the page attributes corresponding to each memory type. There
are two ways to configure page properties: . Where cacheable means that the
slave is allowed to be in an intermediate

1. In all cases without virtual address and physical address translation:
machine mode permissions or MMU off, the page attribute of the address is
determined by the macro definition in the sysmap.h file. sysmap.h is an address
attribute configuration file extended by C906, which is open to users. Users can
define page attributes of different address segments according to their own
needs. The upper limit of the number of address areas is 8.

2. In all cases of virtual address and physical address translation: non-machine
mode permissions and MMU open, the page attribute of the address can be
configured in two ways: the sysmap.h file and the page attribute extended by
C906 in pte. Which configuration method is used depends on the value of the MAEE
field in the C906 extended register MXSTATUS.
If the MAEE field value is 1, the page attribute of the address is determined by
the extended page attribute in the corresponding pte.
If the MAEE field value is 0, the page attribute of the address is determined by
sysmap.h.

2. In all cases where virtual address and physical address conversion are
performed: when the authority is not in machine mode and the MMU is turned on,
there are two ways to configure the page properties of the address.
Type: sysmap.h file and page attributes expanded by C906 in PTE.
Which configuration method is used depends on the C906 extension register
MXSTATUS.
The value of the MAEE field in. If the value of the MAEE field is 1, the page
attribute of the address is determined by the extended page attribute in the
corresponding pte. If the value of the MAEE field is 0, the page attribute of
the address is determined by sysmap.h.

p52 (on mxstatus):

Flags – 63:59 bit page attributes

C906 extended page attribute, exists when the MAEE bit of the MXSTATUS register
is 1, and the function is as described in the MMU EntryLo register (SMEL).

p581 ff:

16.1.7 Machine mode processor control and status extension register group

C906 extends some registers for processor and status, including: machine mode
extended status register (MXSTATUS), machine mode hardware control register
(MHCR), machine mode hardware operation register ( MCOR), Machine Mode Implicit
Operation Register (MHINT), Machine Mode Reset Vector Base Address Register
(MRVBR), Machine Mode Counter Write Enable Authorization Register (MCOUNTERWEN),
Machine Mode Event Counter Overflow Interrupt Enable Register (MCOUNTERINTEN),
Machine Mode The Mode Event Counter Overflow Callout Register (MCOUNTEROF).

16.1.7.1 MXSTATUS

15 MM - Unaligned Access Enable Bit:
 • When MM is 0, unaligned access is not supported, and unaligned access will
   generate an unaligned exception.
 • When MM is 1, unaligned accesses are supported, and unaligned accesses are
   handled by the hardware.
This bit will be reset to 1’b1.

16 UCME - U state executes extended cache instructions:
 • When UCME is 0, user mode cannot execute extended cache operation
   instructions, resulting in an illegal instruction exception.
 • When UCME is 1, user mode can execute extended
   DCACHE.CIVA/DCACHE.CVA/ICACHE.IVA instructions.
This bit will be set to 1’b0 by reset.

17 CLINTEE - CLINT Timer/Software Interrupt Supervisor Extended Enable Bit:
 • When CLINTEE is 0, supervisor software interrupts and timer interrupts from
   CLINT will not be serviced.
 • When CLINTEE is 1, supervisor software interrupts and timer interrupts from
   CLINT can be serviced.
This bit will be set to 1’b0 by reset.

18 MHRD - Turn off hardware backfill:
 • When MHRD is 0, hardware backfill occurs after TLB is missing.
 • When MHRD is 1, the hardware does not perform hardware backfill after a TLB
   is missing.
This bit will be set to 1’b0 by reset.

21 MAEE - extended MMU address attributes:
• When MAEE is 0, the MMU address attribute is not extended.
• When MAEE is 1, the address attribute bit in the pte of the MMU is extended,
  and the user can configure the address attribute of the page.
This bit will be reset to 1’b0.

22 THEADISAEE - enable extended instruction set:
• When THEADISAEE is 0, an illegal instruction exception will be triggered when
  the C906 extended instruction is executed.
• When THEADISAEE is 1, the C906 extended instruction can be executed normally.
This bit will be reset to 1’b0.

// MAEE (Memory Attribute Extension Enable)
#define    MAEE             (0x1 << 21)
// EN_THEADISAEE (T-Head ISA Extension Enable)
#define    EN_THEADISAEE    (0x1 << 22)
*/

/// Jump over head data to executable code.
///
/// # Safety
///
/// Naked function.
///
/// NOTE: `mxstatus` is a custom T-Head register. Do not confuse with `mstatus`.
/// It allows for configuring special eXtensions. See further below for details.
///
/// See also what mainline U-Boot does
/// <https://github.com/smaeul/u-boot/blob/55103cc657a4a84eabc9ae2dabfcab149b07934f/board/sunxi/board-riscv.c#L72-L75>
#[naked]
#[export_name = "start"]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn start() -> ! {
    asm!(
        // 1. clear cache and processor states
        "csrw   mie, zero",
        // enable theadisaee and maee
        "li     t1, 0x1 << 22 | 0x1 << 21",
        "csrs   0x7c0, t1", // MXSTATUS
        // invalidate ICACHE/DCACHE/BTB/BHT
        "li     t2, 0x30013",
        "csrw   0x7c2, t2", // MCOR
        // 2. initialize programming language runtime
        // clear bss segment
        "la     t0, sbss",
        "la     t1, ebss",
        "1:",
        "bgeu   t0, t1, 1f",
        "sd     x0, 0(t0)",
        "addi   t0, t0, 4",
        "j      1b",
        "1:",
        // does not init data segment as BT0 runs in sram
        // 3. prepare stack
        "la     sp, {stack}",
        "li     t0, {stack_size}",
        "add    sp, sp, t0",
        "la     a0, {egon_head}",
        "call   {main}",
        // function `main` returns address of next stage,
        // it drops all peripherals it holds when goes out of scope
        // now, jump to dram code
        "j      {finish}",
        stack      =   sym BT0_STACK,
        stack_size = const STACK_SIZE,
        egon_head  =   sym EGON_HEAD,
        main       =   sym main,
        finish     =   sym finish,
        options(noreturn)
    )
}

#[cfg(feature = "nor")]
fn load(
    skip: usize,
    base: usize,
    size: usize,
    f: &mut SpiNor<
        SPI0,
        (
            Pin<'C', 2, Function<2>>,
            Pin<'C', 3, Function<2>>,
            Pin<'C', 4, Function<2>>,
            Pin<'C', 5, Function<2>>,
        ),
    >,
) {
    let chunks = 16;
    let sz = size >> 2;
    // println!("load {:x} bytes from {:x} to {:x}", size, skip, base);
    print!("load {:08x} bytes to {:x}: ", size, base);
    for i in 0..sz / chunks {
        let off = skip + i * 4 * chunks;
        let buf = f.copy_into([(off >> 16) as u8, (off >> 8) as u8, off as u8]);

        for j in 0..chunks {
            let jw = 4 * j;
            let addr = base + i * 4 * chunks + jw;
            /*
            let s = [buf[jw], buf[jw + 1], buf[jw + 2], buf[jw + 3]];
            println!("{:x}\n\n", s[0]);
            println!(
                "a {:x} o {:08x} v {:02x}{:02x}{:02x}{:02x}",
                addr, off, s[0], s[1], s[2], s[3]
            );
            */
            let s = buf[jw..(jw + 4)].try_into().unwrap();
            // transform bytes from slice to u32
            let val = u32::from_le_bytes(s);
            // for debugging
            // println!("a {:x} o {:08x} v {:08x}", addr, off, val);
            unsafe { write_volatile(addr as *mut u32, val) };
        }
        // progress indicator each 2MB
        if (off - skip) % 0x10_0000 == 0 {
            print!("➡️");
            // for debugging
            // println!("a {:x} o {:08x} v {:08x}", addr, off, val);
        }
    }
    println!(".");
}

const SUNXI_AUDIO_CODEC: u32 = 0x0203_0000;
const AC_SMTH: u32 = SUNXI_AUDIO_CODEC + 0x348;
const SUNXI_SID_BASE: u32 = 0x0300_6000;
const SOC_VER_REG: u32 = SUNXI_SID_BASE + 0x200;
const BANDGAP_TRIM_REG: u32 = SUNXI_SID_BASE + 0x228;

const CCU_BASE: usize = 0x0200_1000;
const CCMU_PLL_PERI0_CTRL_REG: usize = CCU_BASE + 0x0020;
const CCMU_DMA_BGR_REG: usize = CCU_BASE + 0x070c;
const CCU_AUDIO_SMTH: usize = CCU_BASE + 0x0a5c;
const CCMU_CPUX_AXI_CFG_REG: usize = CCU_BASE + 0x0d00;
const RISCV_CFG_BGR: usize = CCU_BASE + 0x0d0c;
const RISCV_CFG_BASE: usize = 0x0601_0000;
const WAKEUP_MASK_REG0: usize = RISCV_CFG_BASE + 0x0024;

const PLL_CPU_CTRL: usize = CCU_BASE;
const PLL_EN: u32 = 1 << 31;
const PLL_N: u32 = 42 << 8; // frequency: input_freq * (PLL_N+1)

fn clrbits_le32(reg: u32, val: u32) {
    unsafe {
        let cval = read_volatile(reg as *mut u32);
        write_volatile(reg as *mut u32, cval & !val);
    }
}

fn setbits_le32(reg: u32, val: u32) {
    unsafe {
        let cval = read_volatile(reg as *mut u32);
        write_volatile(reg as *mut u32, cval | val);
    }
}

const GATING_BIT: u32 = 1 << 0;
const RST_BIT: u32 = 1 << 16;

fn udelay(micros: usize) {
    unsafe {
        for _ in 0..micros {
            core::arch::asm!("nop")
        }
    }
}

/* Trim bandgap reference voltage. */
fn trim_bandgap_ref_voltage() {
    let soc_version = (unsafe { read_volatile(SOC_VER_REG as *mut u32) >> 22 }) & 0x3f;
    println!("v {}", soc_version);

    let mut bg_trim = (unsafe { read_volatile(BANDGAP_TRIM_REG as *mut u32) } >> 16) & 0xff;
    if bg_trim == 0 {
        bg_trim = 0x19;
    }

    let reg = CCU_AUDIO_SMTH as u32;
    clrbits_le32(reg, GATING_BIT);
    udelay(2);
    clrbits_le32(reg, RST_BIT);
    udelay(2);
    /* deassert audio codec reset */
    setbits_le32(reg, RST_BIT);
    /* open the clock for audio codec */
    setbits_le32(reg, GATING_BIT);

    if soc_version == 0b1010 || soc_version == 0 {
        setbits_le32((SUNXI_AUDIO_CODEC + 0x31C) as u32, 1 << 1);
        setbits_le32((SUNXI_AUDIO_CODEC + 0x348) as u32, 1 << 30);
    }

    // TODO: recheck
    let val = unsafe { read_volatile(AC_SMTH as *mut u32) };
    unsafe { write_volatile(AC_SMTH as *mut u32, (val & 0xffffff00) | bg_trim) };
}

extern "C" fn main() -> usize {
    // there was configure_ccu_clocks, but ROM code have already done configuring for us
    let p = Peripherals::take().unwrap();
    // rom provided clock frequency, it's hard coded in bt0 stage
    let clocks = Clocks {
        psi: 600_000_000.hz(),
        apb1: 24_000_000.hz(),
    };
    let gpio = Gpio::new(p.GPIO);

    #[cfg(feature = "jtag")]
    {
        // configure jtag interface
        let tms = gpio.portf.pf0.into_function_4();
        let tck = gpio.portf.pf5.into_function_4();
        let tdi = gpio.portf.pf1.into_function_4();
        let tdo = gpio.portf.pf3.into_function_4();
        let _jtag = Jtag::new((tms, tck, tdi, tdo));
    }

    // prepare serial port logger
    let tx = gpio.portb.pb8.into_function_6();
    let rx = gpio.portb.pb9.into_function_6();
    //  let tx = gpio.portg.pg17.into_function_7();
    //  let rx = gpio.portg.pg18.into_function_7();
    let config = Config {
        baudrate: 115200.bps(),
        wordlength: WordLength::Eight,
        parity: Parity::None,
        stopbits: StopBits::One,
    };
    let serial = Serial::new(p.UART0, (tx, rx), config, &clocks);
    crate::logging::set_logger(serial);

    println!("");
    println!("oreboot 🦀");

    let smhc0 = p.SMHC0;
    #[cfg(not(feature = "jtag"))]
    {
        /*
         # SDHCI and SDC bring up. Links:

         * https://docs.tockos.org/capsules/sdcard/index.html
         * https://github.com/tock/tock/blob/master/capsules/src/sdcard.rs
         * https://github.com/rust-embedded-community/embedded-sdmmc-rs#using-the-crate
         * https://docs.rs/embedded-sdmmc/latest/embedded_sdmmc/

         ## D1 Specific:

         * https://dev.to/xphoniex/how-to-call-c-code-from-rust-56do
         * https://gitlab.com/pnru/xv6-d1/-/blob/master/boot0/sdhost.c
         * https://gitlab.com/pnru/xv6-d1/-/blob/master/boot0/sdcard.c
         * In linux there's a specific driver for sunxi-mmc
           https://github.com/orangecms/linux/blob/5.19-smaeul-plus-dts/drivers/mmc/host/sunxi-mmc.c
        */
        println!("configure pins for SD Card");
        // turn GPIOs into SD card mode; 1 for clock, 1 for cmd, 4 data pins
        gpio.portf.pf0.into_function_2();
        gpio.portf.pf1.into_function_2();
        gpio.portf.pf2.into_function_2();
        gpio.portf.pf3.into_function_2();
        gpio.portf.pf4.into_function_2();
        gpio.portf.pf5.into_function_2();

        // let smhc = Smhc::new(p.SMHC0);

        // STEP 1: reset gating, set up clock
        print!("setup SDHCI gates");
        let ccu = unsafe { &*CCU::ptr() };
        ccu.smhc_bgr
            .write(|w| w.smhc0_rst().deassert().smhc0_gating().set_bit());
        // TODO: optimize based on speed
        println!("...done");
        print!("setup SDHCI clock");
        let factor_n = smhc0_clk::FACTOR_N_A::N2;
        let factor_m = 1;
        ccu.smhc0_clk.write(|w| {
            w.clk_src_sel()
                .pll_peri_1x()
                .factor_n()
                .variant(factor_n)
                .factor_m()
                .variant(factor_m)
                .clk_gating()
                .set_bit()
        });
        println!("...done");

        // STEP2: reset FIFO, enable interrupt; enable SDIO interrupt
        // TODO: register interrupt function; how about simple write!() ?
        print!("reset FIFO, enable SIDO vector");
        smhc0
            .smhc_ctrl
            .write(|w| w.fifo_rst().set_bit().ine_enb().enable());
        smhc0.smhc_intmask.write(|w| w.sdio_int_en().set_bit());
        println!("...done");

        // STEP3: set clock divider for devices (what devices?) and change clock command
        print!("set clock divider and issue change clock command");
        smhc0.smhc_clkdiv.write(|w| w.cclk_enb().off());
        println!("...Issued");

        // see boot0/sdhost.c l98 (cmd in host_update_clk); manual p615
        // CMD_LOAD | PRG_CLK | WAIT_PRE_OVER
        print!("send command with 0x80202000");
        unsafe {
            smhc0.smhc_cmd.write(|w| w.bits(0x80202000));
        }
        println!("...Sent");

        // turn clock back on
        print!("Enable clock");
        smhc0.smhc_clkdiv.write(|w| w.cclk_enb().on());
        println!("...done");
        // bus width: 4 data pins
        print!("Set Bus Width to 4");
        smhc0.smhc_ctype.write(|w| w.card_wid().b4());
        println!("...done");

        // identify card
        const MMC_RSP_PRESENT: u32 = 1 << 6;
        const MMC_RSP_136: u32 = 1 << 7;
        const MMC_RSP_CRC: u32 = 1 << 8;
        let flags = MMC_RSP_PRESENT | MMC_RSP_136 | MMC_RSP_CRC;
        print!("Issue command to ID Card with 0x80000002");
        unsafe {
            smhc0.smhc_cmd.write(|w| w.bits(0x80000002 | flags));
        }
        println!("...done");

        let card_present = smhc0.smhc_status.read().card_present().is_present();
        println!("SD card present? {}", card_present);

        while smhc0.smhc_status.read().card_busy().is_busy() {}
        for _ in 0..1_000_000 {
            core::hint::spin_loop();
        }

        let r0 = smhc0.smhc_resp0.read().bits();
        let r1 = smhc0.smhc_resp1.read().bits();
        let r2 = smhc0.smhc_resp2.read().bits();
        let r3 = smhc0.smhc_resp3.read().bits();

        println!("SD card {:02x}{:02x}{:02x}{:02x}", r0, r1, r2, r3);
    }

    // FIXME: Much of the below can be removed or moved over to the main stage.
    trim_bandgap_ref_voltage();

    let mut cpu_pll = unsafe { read_volatile(PLL_CPU_CTRL as *mut u32) };
    println!("cpu_pll {:x}", cpu_pll); // 0xFA00_1000
    cpu_pll &= 0xFFFF_00FF;
    cpu_pll |= PLL_EN | PLL_N;
    unsafe { write_volatile(PLL_CPU_CTRL as *mut u32, cpu_pll) };

    // dma_bgr::RST_W::set_bit(dma_bgr::RST_A::ASSERT);
    //  DMA_BGR::write(|w| w.reset().set_bit());
    let dma_bgr = unsafe { read_volatile(CCMU_DMA_BGR_REG as *mut u32) };
    unsafe { write_volatile(CCMU_DMA_BGR_REG as *mut u32, dma_bgr | 1 << 16) };
    let dma_bgr = unsafe { read_volatile(CCMU_DMA_BGR_REG as *mut u32) };
    unsafe { write_volatile(CCMU_DMA_BGR_REG as *mut u32, dma_bgr | 1 << 0) };

    for _ in 0..1000 {
        core::hint::spin_loop();
    }
    let mut cpu_axi = unsafe { read_volatile(CCMU_CPUX_AXI_CFG_REG as *mut u32) };
    println!("cpu_axi {:x}", cpu_axi); // 0xFA00_1000
    cpu_axi &= 0x07 << 24 | 0x3 << 8 | 0xf << 0;
    cpu_axi |= 0x05 << 24 | 0x1 << 8;
    unsafe { write_volatile(CCMU_CPUX_AXI_CFG_REG as *mut u32, cpu_axi) };
    for _ in 0..1000 {
        core::hint::spin_loop();
    }
    println!("cpu_axi {:x}", cpu_axi); // 0xFA00_1000

    let peri0_ctrl = unsafe { read_volatile(CCMU_PLL_PERI0_CTRL_REG as *mut u32) };
    println!("peri0_ctrl was: {:x}", peri0_ctrl); // f8216300

    // unsafe { write_volatile(CCMU_PLL_PERI0_CTRL_REG as *mut u32, 0x63 << 8) };
    // println!("peri0_ctrl default");
    // enable lock
    let peri0_ctrl = unsafe { read_volatile(CCMU_PLL_PERI0_CTRL_REG as *mut u32) };
    unsafe { write_volatile(CCMU_PLL_PERI0_CTRL_REG as *mut u32, peri0_ctrl | 1 << 29) };
    println!("peri0_ctrl lock en");
    // enabe PLL: 600M(1X)  1200M(2x)
    let peri0_ctrl = unsafe { read_volatile(CCMU_PLL_PERI0_CTRL_REG as *mut u32) };
    unsafe { write_volatile(CCMU_PLL_PERI0_CTRL_REG as *mut u32, peri0_ctrl | 1 << 31) };
    println!("peri0_ctrl PLLs");
    let peri0_ctrl = unsafe { read_volatile(CCMU_PLL_PERI0_CTRL_REG as *mut u32) };
    println!("peri0_ctrl set: {:x}", peri0_ctrl);

    /* Initialize RISCV_CFG. */
    unsafe {
        write_volatile(RISCV_CFG_BGR as *mut u32, 0x0001_0001);
        for i in 0..5 {
            write_volatile((WAKEUP_MASK_REG0 + 4 * i) as *mut u32, 0xffffffff);
        }
    }

    let ram_size = mctl::init();
    println!("{}M 🐏", ram_size);

    #[cfg(any(feature = "nor", feature = "mmc"))]
    let spi_speed = 48_000_000.hz();
    #[cfg(feature = "nand")]
    let spi_speed = 100_000_000.hz();

    // prepare spi interface to use in flash
    let sck = gpio.portc.pc2.into_function_2();
    let scs = gpio.portc.pc3.into_function_2();
    let mosi = gpio.portc.pc4.into_function_2();
    let miso = gpio.portc.pc5.into_function_2();
    let spi = Spi::new(p.SPI0, (sck, scs, mosi, miso), MODE_3, spi_speed, &clocks);

    #[cfg(feature = "mmc")]
    {
        println!("TODO: load from SD card");
    }

    #[cfg(feature = "nor")]
    {
        let mut flash = SpiNor::new(spi);

        // e.g., GigaDevice (GD) is 0xC8 and GD25Q128 is 0x4018
        // see flashrom/flashchips.h for details and more
        let id = flash.read_id();
        println!("NOR flash: {:x}/{:x}{:x}", id[0], id[1], id[2],);

        // TODO: Either read sizes from dtfs at runtime or at build time

        // println!("💾");
        let skip = 0x1 << 15; // 32K, the size of boot0
        load(skip, ORE_ADDR, ORE_SIZE, &mut flash);

        // 32K + oreboot + dtfs, see oreboot dtfs
        let skip = skip + ORE_SIZE + DTF_SIZE;
        load(skip, LIN_ADDR, LIN_SIZE, &mut flash);

        // 32K + oreboot + dtfs + payload
        let skip = skip + LIN_SIZE;
        load(skip, DTB_ADDR, DTB_SIZE, &mut flash);

        let _ = flash.free().free();
    }

    #[cfg(feature = "nand")]
    {
        let mut flash = SpiNand::new(spi);
        println!("NAND flash: {:?}", flash.read_id());

        // TODO: Either read sizes from dtfs at runtime or at build time

        let mut main_stage_head = [0u8; 8];
        flash.copy_into(0x60, &mut main_stage_head);
        let main_stage_head: MainStageHead = unsafe { core::mem::transmute(main_stage_head) };
        println!(
            "flash offset: {}, length: {}",
            main_stage_head.offset, main_stage_head.length
        );
        let ddr_buffer = unsafe {
            core::slice::from_raw_parts_mut(RAM_BASE as *mut u8, main_stage_head.length as usize)
        };
        flash.copy_into(main_stage_head.offset, ddr_buffer);
        // flash is freed when it goes out of scope
    }

    println!("Run payload at 0x{:x}", RAM_BASE);
    unsafe {
        let f: unsafe extern "C" fn() = transmute(RAM_BASE);
        f();
    }

    // returns an address of dram payload; now cpu would jump to this address
    // and run code inside
    RAM_BASE
}

// jump to dram
extern "C" fn finish(main_stage: extern "C" fn()) -> ! {
    main_stage();
    loop {
        unsafe { asm!("wfi") }
    }
}

#[cfg_attr(not(test), panic_handler)]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("panic in '{}' line {}", location.file(), location.line(),);
    } else {
        println!("panic at unknown location");
    };
    loop {
        core::hint::spin_loop();
    }
}
