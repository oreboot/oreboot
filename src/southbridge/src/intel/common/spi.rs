use alloc::vec::Vec;
use arch::x86_64::mmio::{read16, read32, read8, write16, write32, write8};
use drivers::spi::{
    cbfs_spi::boot_device_spi_flash,
    spi_flash::{SpiFlash, SpiFlashOps, SPI_OPCODE_FAST_READ, SPI_OPCODE_WREN},
    spi_generic::{CtrlrProtType, SpiCtrlr, SpiCtrlrBuses, SpiOp, SpiSlave, VENDOR_ID_SST},
    Error,
};

use payload::drivers::pci_map_bus_ops::{pci_read_config32, pci_read_config8, pci_write_config8};
use spin::rwlock::RwLock;
use types::pci_type::{pci_dev, PciDevFnT};
use util::{
    region::Region,
    timer::{udelay, USECS_PER_MSEC},
};

pub const HSFC_FCYCLE_OFF: u16 = 1; /* 1-2: FLASH Cycle */
pub const HSFC_FCYCLE: u16 = 0x3 << HSFC_FCYCLE_OFF;
pub const HSFC_FDBC_OFF: u16 = 8; /* 8-13: Flash Data Byte Count */
pub const HSFC_FDBC: u16 = 0x3f << HSFC_FDBC_OFF;

pub const RCBA: usize = 0xf0;
pub const SBASE: usize = 0x54;

pub const MENU_BYTES: usize = 8;
pub const FDATA_LEN: usize = 16;

pub static SPI_CTRLR_BUS_MAP: [SpiCtrlrBuses; 1] = [SpiCtrlrBuses {
    bus_start: 0,
    bus_end: 0,
    ctrlr: &IntelSpiCtrlr::new(),
}];

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum Optype {
    ReadNoAddr = 0,
    WriteNoAddr = 1,
    ReadWithAddr = 2,
    WriteWithAddr = 3,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IntelSpiOp {
    pub op: u8,
    pub type_: Optype,
}

impl IntelSpiOp {
    pub const fn create(op: u8, type_: Optype) -> Self {
        Self { op, type_ }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct IntelSwseqSpiConfig {
    pub opprefixes: [u8; 2],
    pub ops: [IntelSpiOp; 8],
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub enum SpiOpcodeType {
    ReadNoAddress = 0,
    WriteNoAddress = 1,
    ReadWithAddress = 2,
    WriteWithAddress = 3,
    Undefined = 0xff,
}

#[repr(C)]
pub enum SpicMask {
    Scgo = 0x000002,
    Acs = 0x000004,
    Spop = 0x000008,
    Dbc = 0x003f00,
    Ds = 0x004000,
    Sme = 0x008000,
    SsfcScfMask = 0x070000,
    SsfcReserved = 0xf80000,
}

impl From<u8> for SpiOpcodeType {
    fn from(b: u8) -> Self {
        match b {
            0 => SpiOpcodeType::ReadNoAddress,
            1 => SpiOpcodeType::WriteNoAddress,
            2 => SpiOpcodeType::ReadWithAddress,
            3 => SpiOpcodeType::WriteWithAddress,
            _ => SpiOpcodeType::Undefined,
        }
    }
}

#[repr(C)]
pub enum Hsfs {
    Fdone = 0x0001,
    Fcerr = 0x0002,
    Ael = 0x0004,
    BeraseMask = 0x0018,
    BeraseShift = 3,
    Scip = 0x0020,
    Fdopss = 0x2000,
    Fdv = 0x4000,
    Flockdn = 0x5000,
}

#[repr(C)]
pub enum Hsfc {
    Fgo = 0x0001,
    FcycleMask = 0x0006,
    FdbcMask = 0x3f00,
    Fsmie = 0x8000,
}

#[repr(C)]
pub enum HsfcShift {
    FcycleShift = 1,
    FdbcShift = 8,
}

#[cfg(DEBUG_SPI_FLASH)]
pub fn readb_(addr: usize) -> u8 {
    let v = unsafe { read8(addr) };

    //debug!("read {:2x} from {:4x}", v, (addr & 0xffff) - 0xf020);

    v
}

#[cfg(DEBUG_SPI_FLASH)]
pub fn readw_(addr: usize) -> u16 {
    let v = unsafe { read16(addr) };

    //debug!("read {:4x} from {:4x}", v, (addr & 0xffff) - 0xf020);

    v
}

#[cfg(DEBUG_SPI_FLASH)]
pub fn readl_(addr: usize) -> u16 {
    let v = unsafe { read32(addr) };

    //debug!("read {:8x} from {:4x}", v, (addr & 0xffff) - 0xf020);

    v
}

#[cfg(DEBUG_SPI_FLASH)]
pub fn writeb_(b: u8, addr: usize) {
    unsafe { write8(addr, b) };

    //debug!("wrote {:2x} to {:4x}", b, (addr & 0xffff) - 0xf020);
}

#[cfg(DEBUG_SPI_FLASH)]
pub fn writew_(b: u16, addr: usize) {
    unsafe { write16(addr, b) };

    //debug!("wrote {:4x} to {:4x}", b, (addr & 0xffff) - 0xf020);
}

#[cfg(DEBUG_SPI_FLASH)]
pub fn writel_(b: u32, addr: usize) {
    unsafe { write32(addr, b) };

    //debug!("wrote {:8x} to {:4x}", b, (addr & 0xffff) - 0xf020);
}

#[cfg(not(DEBUG_SPI_FLASH))]
pub fn readb_(addr: usize) -> u8 {
    unsafe { read8(addr) }
}

#[cfg(not(DEBUG_SPI_FLASH))]
pub fn readw_(addr: usize) -> u16 {
    unsafe { read16(addr) }
}

#[cfg(not(DEBUG_SPI_FLASH))]
pub fn readl_(addr: usize) -> u32 {
    unsafe { read32(addr) }
}

#[cfg(not(DEBUG_SPI_FLASH))]
pub fn writeb_(b: u8, addr: usize) {
    unsafe { write8(addr, b) };
}

#[cfg(not(DEBUG_SPI_FLASH))]
pub fn writew_(b: u16, addr: usize) {
    unsafe { write16(addr, b) };
}

#[cfg(not(DEBUG_SPI_FLASH))]
pub fn writel_(b: u32, addr: usize) {
    unsafe { write32(addr, b) };
}

pub fn write_reg(value: &[u8], dest: &mut [u8]) {
    for (i, ch) in value.chunks(4).enumerate() {
        if ch.len() == 4 {
            dest[i..i + 4].copy_from_slice(ch);
        } else {
            for (j, b) in ch.iter().enumerate() {
                dest[i + j] = *b;
            }
        }
    }
}

pub fn read_reg(src: &[u8], value: &mut [u8]) {
    for (i, ch) in src.chunks(4).enumerate() {
        if ch.len() == 4 {
            value[i..i + 4].copy_from_slice(ch);
        } else {
            for (j, b) in ch.iter().enumerate() {
                value[i + j] = *b;
            }
        }
    }
}

pub fn spi_locked() -> bool {
    if cfg!(feature = "southbridge_intel_i82801gx") {
        (readw_(unsafe { (*CNTLR.read()).regs.ich7_spi.spis } as usize) & Hsfs::Flockdn as u16) != 0
    } else {
        (readw_(unsafe { (*CNTLR.read()).regs.ich9_spi.hsfs } as usize) & Hsfs::Flockdn as u16) != 0
    }
}

/// Wait for up to 6s til status register bit(s) turn 1 (in case wait_til_set
/// below is True) or 0. In case the wait was for the bit(s) to set - write
/// those bits back, which would cause resetting them.
///
/// Return the last read status value on success or -1 on failure.
pub fn ich_status_poll(bitmask: u16, wait_til_set: i32) -> Result<u16, Error> {
    let mut timeout = 600000; /* This will result in 6 seconds */
    let mut status = 0;

    while timeout > 0 {
        status = readw_((*CNTLR.read()).status as usize);
        if wait_til_set ^ (((status & bitmask) == 0) as i32) != 0 {
            if wait_til_set != 0 {
                writew_(status & bitmask, (*CNTLR.read()).status as usize);
            }
            return Ok(status);
        }
        udelay(10);
        timeout -= 1;
    }

    //debug!(
        "ICH SPI: SCIP timeout, read {:x}, bitmask {:x}",
        status, bitmask
    );

    Err(Error::Generic)
}

#[repr(C)]
pub struct SpiTransaction<'a, 'b> {
    pub out: &'a [u8],
    pub in_: &'b mut [u8],
    pub type_: SpiOpcodeType,
    pub opcode: u8,
    pub offset: u32,
}

impl<'a, 'b> SpiTransaction<'a, 'b> {
    pub fn spi_setup_type(&mut self) -> Result<(), Error> {
        self.type_ = SpiOpcodeType::Undefined;

        /* Try to guess spi type from read/write sizes. */
        if self.in_.len() == 0 {
            if self.out.len() > 4 {
                /*
                 * If bytesin = 0 and bytesout > 4, we presume this is
                 * a write data operation, which is accompanied by an
                 * address.
                 */
                self.type_ = SpiOpcodeType::WriteWithAddress;
            } else {
                self.type_ = SpiOpcodeType::WriteNoAddress;
            }
            return Ok(());
        }

        if self.out.len() == 1 {
            /* and bytesin is > 0 */
            self.type_ = SpiOpcodeType::ReadNoAddress;
            return Ok(());
        }

        if self.out.len() == 4 {
            /* and bytesin is > 0 */
            self.type_ = SpiOpcodeType::ReadWithAddress;
        }

        if self.out[0] == SPI_OPCODE_FAST_READ && self.out.len() == 5 {
            self.type_ = SpiOpcodeType::ReadWithAddress;
            self.out = &self.out[..self.out.len() - 2];
        }

        Ok(())
    }

    pub fn spi_setup_opcode(&mut self) -> Result<i32, Error> {
        self.opcode = self.out[0];
        let mut opmenu = [0u8; MENU_BYTES];

        self.spi_use_out(1);

        if spi_locked() {
            writeb_(
                self.opcode,
                &(*CNTLR.read()).opmenu[0] as *const u8 as usize,
            );
            let mut optypes = readw_((*CNTLR.read()).optype as usize);
            optypes = (optypes & 0xfffc) | ((self.type_ as u16) & 0x3);
            writew_(optypes, (*CNTLR.read()).optype as usize);
            return Ok(0);
        }

        /* The lock is on. See if what we need is on the menu. */
        /* Write Enable is handled as atomic prefix */
        if self.opcode == SPI_OPCODE_WREN {
            return Ok(0);
        }

        read_reg(&(*CNTLR.read()).opmenu, &mut opmenu);
        let mut opcode_index = -1;
        for (i, op) in opmenu.iter().enumerate() {
            if *op == self.opcode {
                opcode_index = i as i32;
            }
        }

        if opcode_index == -1 {
            //debug!("ICH SPI: Opcode {:x} not found", self.opcode);
            return Err(Error::Generic);
        }

        let optypes = readw_((*CNTLR.read()).optype as usize);
        let optype = SpiOpcodeType::from(((optypes >> (opcode_index * 2)) & 0x3) as u8);

        if self.type_ == SpiOpcodeType::WriteNoAddress
            && optype == SpiOpcodeType::WriteWithAddress
            && self.out.len() >= 3
        {
            /* We guessed wrong earlier. Fix it up. */
            self.type_ = optype;
        }
        if optype != self.type_ {
            //debug!("ICH SPI: Transaction doesn't fit type {}", optype as u8);
            return Err(Error::Generic);
        }

        Ok(opcode_index)
    }

    pub fn spi_use_out(&mut self, bytes: usize) {
        self.out = &self.out[bytes..];
    }

    pub fn spi_use_in(&'b mut self, bytes: usize) {
        self.in_ = &mut self.in_[bytes..];
    }

    pub fn spi_setup_offset(&mut self) -> Result<usize, Error> {
        /* Separate the SPI address and data. */
        match self.type_ {
            SpiOpcodeType::ReadNoAddress | SpiOpcodeType::WriteNoAddress => Ok(0),
            SpiOpcodeType::ReadWithAddress | SpiOpcodeType::WriteWithAddress => {
                self.offset = ((self.out[0] as u32) << 16)
                    | ((self.out[1] as u32) << 8)
                    | ((self.out[2] as u32) << 0);
                self.spi_use_out(3);
                Ok(1)
            }
            _ => {
                //debug!("Unrecognized SPI transaction type {:x}", self.type_ as u8);
                Err(Error::Generic)
            }
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ich7SpiRegs {
    spis: u16,
    spic: u16,
    spia: u32,
    spid: [u64; 8],
    _pad: u64,
    bbar: u32,
    preop: u16,
    optype: u16,
    opmenu: [u8; 8],
    pbr: [u32; 3],
}

impl Ich7SpiRegs {
    pub const fn new() -> Self {
        Self {
            spis: 0u16,
            spic: 0u16,
            spia: 0u32,
            spid: [0u64; 8],
            _pad: 0u64,
            bbar: 0u32,
            preop: 0u16,
            optype: 0u16,
            opmenu: [0u8; 8],
            pbr: [0u32; 3],
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Ich9SpiRegs {
    bfpr: u32,
    hsfs: u16,
    hsfc: u16,
    faddr: u32,
    _reserved0: u32,
    fdata: [u32; FDATA_LEN],
    frap: u32,
    freg: [u32; 5],
    _reserved1: [u32; 3],
    pr: [u32; 5],
    _reserved2: [u32; 2],
    ssfs: u8,
    ssfc: [u8; 3],
    preop: u16,
    optype: u16,
    opmenu: [u8; 8],
    bbar: u32,
    _reserved3: [u8; 12],
    fdoc: u32,
    fdod: u32,
    _reserved4: [u8; 8],
    afc: u32,
    lvscc: u32,
    uvscc: u32,
    _reserved5: [u8; 4],
    fpb: u32,
    _reserved6: [u8; 28],
    srdl: u32,
    srdc: u32,
    srd: u32,
}

impl Ich9SpiRegs {
    pub const fn new() -> Self {
        Self {
            bfpr: 0u32,
            hsfs: 0u16,
            hsfc: 0u16,
            faddr: 0u32,
            _reserved0: 0u32,
            fdata: [0u32; FDATA_LEN],
            frap: 0u32,
            freg: [0u32; 5],
            _reserved1: [0u32; 3],
            pr: [0u32; 5],
            _reserved2: [0u32; 2],
            ssfs: 0u8,
            ssfc: [0u8; 3],
            preop: 0u16,
            optype: 0u16,
            opmenu: [0u8; 8],
            bbar: 0u32,
            _reserved3: [0u8; 12],
            fdoc: 0u32,
            fdod: 0u32,
            _reserved4: [0u8; 8],
            afc: 0u32,
            lvscc: 0u32,
            uvscc: 0u32,
            _reserved5: [0u8; 4],
            fpb: 0u32,
            _reserved6: [0u8; 28],
            srdl: 0u32,
            srdc: 0u32,
            srd: 0u32,
        }
    }
}

#[repr(C)]
pub union IchRegsUnion {
    ich9_spi: Ich9SpiRegs,
    ich7_spi: Ich7SpiRegs,
}

#[repr(C)]
pub struct IchSpiController {
    pub locked: i32,
    pub flmap0: u32,
    pub flcomp: u32,
    pub hsfs: u32,
    pub regs: IchRegsUnion,
    pub opmenu: [u8; MENU_BYTES],
    pub menubytes: i32,
    pub preop: u16,
    pub optype: u16,
    pub addr: u32,
    pub data: Vec<u8>,
    pub databytes: u32,
    pub status: u8,
    pub control: u16,
    pub bbar: u32,
    pub fpr: u32,
    pub fpr_max: u8,
}

impl IchSpiController {
    pub const fn new() -> Self {
        let regs = if cfg!(feature = "southbridge_intel_i82801gx") {
            IchRegsUnion {
                ich7_spi: Ich7SpiRegs::new(),
            }
        } else {
            IchRegsUnion {
                ich9_spi: Ich9SpiRegs::new(),
            }
        };
        Self {
            locked: 0i32,
            flmap0: 0u32,
            flcomp: 0u32,
            hsfs: 0u32,
            regs,
            opmenu: [0u8; MENU_BYTES],
            menubytes: 0i32,
            preop: 0u16,
            optype: 0u16,
            addr: 0u32,
            data: Vec::new(),
            databytes: 0u32,
            status: 0u8,
            control: 0u16,
            bbar: 0u32,
            fpr: 0u32,
            fpr_max: 0u8,
        }
    }
}

static CNTLR: RwLock<IchSpiController> = RwLock::new(IchSpiController::new());

pub fn get_spi_bar<T: Copy>(dev: PciDevFnT) -> Result<T, Error> {
    if cfg!(feature = "southbridge_intel_i82801gx") {
        let rcba = pci_read_config32(dev, RCBA as u16) as usize;
        return Ok(unsafe { *(((rcba & 0xffffc000) + 0x3020) as *mut T) });
    }
    if cfg!(feature = "southbridge_intel_common_spi_silvermont") {
        let mut sbase = pci_read_config32(dev, SBASE as u16) as usize;
        sbase &= !0x1ff;
        return Ok(unsafe { *(sbase as *mut T) });
    }
    if cfg!(feature = "southbridge_intel_common_spi_ich9") {
        let rcba = pci_read_config32(dev, RCBA as u16) as usize;
        return Ok(unsafe { *(((rcba & 0xffffc000) + 0x3800) as *mut T) });
    }

    Err(Error::Generic)
}

#[repr(C)]
pub struct IntelSpiCtrlr {
    pub spi: SpiSlave,
    pub max_xfer_size: u32,
    pub flags: u32,
}

impl IntelSpiCtrlr {
    pub const fn new() -> Self {
        Self {
            spi: SpiSlave::new(),
            max_xfer_size: FDATA_LEN as u32,
            flags: 0,
        }
    }
}

#[repr(C)]
pub enum SpiMask {
    SpisScip = 0x0001,
    SpisGrant = 0x0002,
    SpisCds = 0x0004,
    SpisFcerr = 0x0008,
    SsfsAel = 0x0010,
    SpisLock = 0x8000,
    SpisReservedMask = 0x7ff0,
    SsfsReservedMask = 0x7fe2,
}

impl SpiCtrlr for IntelSpiCtrlr {
    fn xfer_vector(&self, vectors: &mut [SpiOp]) -> Result<(), Error> {
        xfer_vectors(&self.spi, vectors)
    }

    fn flash_probe(&self, flash: &mut SpiFlash) -> Result<(), Error> {
        spi_flash_programmer_probe(&self.spi, flash)
    }

    fn flash_protect(
        &self,
        flash: &SpiFlash,
        region: &Region,
        type_: CtrlrProtType,
    ) -> Result<(), Error> {
        spi_flash_protect(flash, region, type_)
    }

    fn max_xfer_size(&self) -> u32 {
        self.max_xfer_size
    }

    fn set_max_xfer_size(&mut self, size: u32) {
        self.max_xfer_size = size;
    }

    fn flags(&self) -> u32 {
        self.flags
    }

    fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
    }
}

pub fn spi_is_multichip() -> bool {
    if (*CNTLR.read()).hsfs & (Hsfs::Fdv as u32) == 0 {
        false
    } else {
        (((*CNTLR.read()).flmap0 >> 8) & 3) != 0
    }
}

pub fn spi_ctrlr_xfer(_spi: &SpiSlave, dout: &[u8], din: &mut [u8]) -> Result<(), Error> {
    let spi_xfer_exit = || -> Result<(), Error> {
        writew_(0, (*CNTLR.read()).preop as usize);
        Ok(())
    };

    let mut trans = SpiTransaction {
        out: dout,
        in_: din,
        type_: SpiOpcodeType::Undefined,
        opcode: 0xff,
        offset: 0,
    };

    if dout.len() == 0 {
        //debug!("ICH SPI: No opcode for transfer");
        return Err(Error::Generic);
    }

    ich_status_poll(SpiMask::SpisScip as u16, 0)?;

    trans.spi_setup_type()?;

    let opcode_index = trans.spi_setup_opcode()?;
    let with_address = trans.spi_setup_offset()?;

    if trans.opcode == SPI_OPCODE_WREN {
        /*
         * Treat Write Enable as Atomic Pre-Op if possible
         * in order to prevent the Management Engine from
         * issuing a transaction between WREN and DATA.
         */
        if !spi_locked() {
            writew_(trans.opcode as u16, (*CNTLR.read()).preop as usize);
        }
        return Ok(());
    }

    /* Preset control fields */
    let mut control = (SpicMask::Scgo as u16) | (((opcode_index as u16) & 0x07) << 4);

    /* Issue atomic preop cycle if needed */
    if readw_((*CNTLR.read()).preop as usize) != 0 {
        control |= SpicMask::Acs as u16;
    }

    if trans.out.len() == 0 && trans.in_.len() == 0 {
        /* SPI addresses are 24 bit only */
        if with_address != 0 {
            writel_(trans.offset & 0x00ff_ffff, (*CNTLR.read()).addr as usize);
        }

        /*
         * This is a 'no data' command (like Write Enable), its
         * bitesout size was 1, decremented to zero while executing
         * spi_setup_opcode() above. Tell the chip to send the
         * command.
         */
        writew_(control, (*CNTLR.read()).control as usize);

        /* wait for the result */
        let status = ich_status_poll((SpiMask::SpisCds as u16) | (SpiMask::SpisFcerr as u16), 1)?;

        if (status & (SpiMask::SpisFcerr as u16)) != 0 {
            //debug!("ICH SPI: Command transaction error");
            return Err(Error::Generic);
        }

        return spi_xfer_exit();
    }

    /*
     * Check if this is a write command attempting to transfer more bytes
     * than the controller can handle. Iterations for writes are not
     * supported here because each SPI write command needs to be preceded
     * and followed by other SPI commands, and this sequence is controlled
     * by the SPI chip driver.
     */
    if trans.out.len() > (*CNTLR.read()).data.len() {
        //debug!("ICH SPI: Too much to write. Does your SPI chip driver use spi_crop_chunk()?");
        return Err(Error::Generic);
    }

    /*
     * Read or write up to databytes bytes at a time until everything has
     * been sent.
     */
    while trans.out.len() != 0 || trans.in_.len() != 0 {
        /* SPI addresses are 24 bit only */
        writel_(trans.offset & 0x00ff_ffff, (*CNTLR.read()).addr as usize);

        let data_length = if trans.out.len() != 0 {
            core::cmp::min(trans.out.len(), (*CNTLR.read()).data.len())
        } else {
            core::cmp::min(trans.in_.len(), (*CNTLR.read()).data.len())
        };

        /* Program data into FDATA0 to N */
        if trans.out.len() != 0 {
            write_reg(trans.out, &mut (*CNTLR.write()).data);
            trans.spi_use_out(data_length);
            if with_address != 0 {
                trans.offset += data_length as u32;
            }
        }

        control &= !(((*CNTLR.read()).data.len() - 1) << 8) as u16;
        control |= SpicMask::Ds as u16;
        control |= ((data_length - 1) << 8) as u16;

        /* write it */
        writew_(control, (*CNTLR.read()).control as usize);

        /* Wait for Cycle Done Status or Flash Cycle Error. */
        let status = ich_status_poll((SpiMask::SpisCds as u16) | (SpiMask::SpisFcerr as u16), 1)?;

        if status & (SpiMask::SpisFcerr as u16) != 0 {
            //debug!("ICH SPI: Data transaction error");
            return Err(Error::Generic);
        }

        if trans.in_.len() != 0 {
            read_reg(
                &(*CNTLR.read()).data[..data_length],
                &mut trans.in_[..data_length],
            );
            /* trans.spi_use_in(data_length); */
            trans.in_ = &mut trans.in_[data_length..];
            if with_address != 0 {
                trans.offset += data_length as u32;
            }
        }
    }

    spi_xfer_exit()
}

/// Sets FLA in FADDR to (addr & 0x01FFFFFF) without touching other bits.
pub fn ich_hwseq_set_addr(addr: u32) {
    let addr_old = readl_(&unsafe { (*CNTLR.read()).regs.ich9_spi.faddr } as *const u32 as usize)
        & !0x01ff_ffff;
    writel_(
        (addr & 0x01ff_ffff) | addr_old,
        unsafe { (*CNTLR.read()).regs.ich9_spi.faddr } as usize,
    );
}

/// Polls for Cycle Done Status, Flash Cycle Error or timeout in 8 us intervals.
/// Resets all error flags in HSFS.
/// Returns 0 if the cycle completes successfully without errors within
/// timeout us, 1 on errors.
pub fn ich_hwseq_wait_for_cycle_complete(mut timeout: u32, len: u32) -> Result<(), Error> {
    timeout /= 8; /* scale timeout duration to counter */
    let mut hsfs = readw_(&unsafe { (*CNTLR.read()).regs.ich9_spi.hsfs } as *const u16 as usize);
    loop {
        timeout -= 1;
        if hsfs & ((Hsfs::Fdone as u16) | (Hsfs::Fcerr as u16)) != 0 || timeout == 0 {
            break;
        }
        udelay(8);
        hsfs = readw_(&unsafe { (*CNTLR.read()).regs.ich9_spi.hsfs } as *const u16 as usize);
    }
    writew_(
        readw_(&unsafe { (*CNTLR.read()).regs.ich9_spi.hsfs } as *const u16 as usize),
        &unsafe { (*CNTLR.read()).regs.ich9_spi.hsfs } as *const u16 as usize,
    );

    if timeout == 0 {
        let addr = readl_(&unsafe { (*CNTLR.read()).regs.ich9_spi.faddr } as *const u32 as usize)
            & 0x01ff_ffff;
        let hsfc = readw_(&unsafe { (*CNTLR.read()).regs.ich9_spi.hsfc } as *const u16 as usize);
        //error!("Transaction timeout between offset 0x{:8x} and 0x{:8x} (= 0x{:8x} + {}) HSFC={:x} HSFS={:x}!", addr, addr + len - 1, addr, len - 1, hsfc, hsfs);
        return Err(Error::Generic);
    }

    if hsfs & Hsfs::Fcerr as u16 != 0 {
        let addr = readl_(&unsafe { (*CNTLR.read()).regs.ich9_spi.faddr } as *const u32 as usize)
            & 0x01ff_ffff;
        let hsfc = readw_(&unsafe { (*CNTLR.read()).regs.ich9_spi.hsfc } as *const u16 as usize);
        //error!("Transaction error between offset 0x{:8x} and 0x{:8x} (= 0x{:8x} + {}) HSFC={:x} HSFS={:x}!", addr, addr + len - 1, addr, len - 1, hsfc, hsfs);
    }

    Ok(())
}

pub fn ich_hwseq_erase(flash: &SpiFlash, mut offset: u32, len: usize) -> Result<(), Error> {
    let out = |flash: &SpiFlash, ret: Result<(), Error>| -> Result<(), Error> {
        flash.spi.release_bus()?;
        ret
    };

    let timeout = 1000 * USECS_PER_MSEC;
    let erase_size = flash.sector_size;

    if (offset % erase_size as u32 != 0) || (len as u32 % erase_size != 0) {
        //error!("SF: Erase offset/length not multiple of erase size");
        return Err(Error::Generic);
    }

    if let Err(e) = flash.spi.claim_bus() {
        //error!("SF: Unable to claim SPI bus");
        return Err(e);
    }

    let start = offset as usize;
    let end = start + len;

    while offset < end as u32 {
        /* make sure FDONE, FCERR, AEL are cleared by writing 1 to them */
        writew_(
            readw_(&unsafe { (*CNTLR.read()).regs.ich9_spi.hsfs } as *const u16 as usize),
            &unsafe { (*CNTLR.read()).regs.ich9_spi.hsfs } as *const u16 as usize,
        );

        ich_hwseq_set_addr(offset);

        offset += erase_size;

        let mut hsfc =
            readw_(&unsafe { (*CNTLR.read()).regs.ich9_spi.hsfc } as *const u16 as usize);
        hsfc &= !HSFC_FCYCLE; /* clear operation */
        hsfc |= 0x3 << HSFC_FCYCLE_OFF; /* set erase operation */
        hsfc |= Hsfc::Fgo as u16; /* start */
        writew_(
            hsfc,
            &unsafe { (*CNTLR.read()).regs.ich9_spi.hsfc } as *const u16 as usize,
        );
        if ich_hwseq_wait_for_cycle_complete(timeout as u32, len as u32).is_err() {
            //error!("SF: Erase failed at {:x}", offset - erase_size);
            return out(flash, Err(Error::Generic));
        }
    }

    //debug!("SF: Successfully erase {} bytes @ {:x}", len, start);

    out(flash, Ok(()))
}

pub fn ich_read_data(data: &mut [u8]) {
    let mut temp32 = 0;

    for i in 0..data.len() {
        if i % 4 == 0 {
            temp32 = readl_(&(*CNTLR.read()).data[i] as *const u8 as usize);
        }

        data[i] = ((temp32 >> ((i % 4) * 8)) & 0xff) as u8;
    }
}

pub fn ich_hwseq_read(flash: &SpiFlash, mut addr: u32, mut buf: &mut [u8]) -> Result<(), Error> {
    let mut len = buf.len();

    let timeout: u16 = 100 * 60;

    if addr as usize + len > flash.size as usize {
        //error!(
            "Attempt to read {:x}-{:x} which is out of chip",
            addr,
            addr + len as u32
        );
        return Err(Error::Generic);
    }

    /* clear FDONE, FCERR, AEL by writing 1 to them (if they are set) */
    writew_(
        readw_(&unsafe { (*CNTLR.read()).regs.ich9_spi.hsfs } as *const u16 as usize),
        &unsafe { (*CNTLR.read()).regs.ich9_spi.hsfs } as *const u16 as usize,
    );

    while len > 0 {
        let mut block_len = core::cmp::min(len, (*CNTLR.read()).data.len());
        if block_len > (!addr & 0xff) as usize {
            block_len = ((!addr & 0xff) + 1) as usize;
        }
        ich_hwseq_set_addr(addr);
        let mut hsfc =
            readw_(&unsafe { (*CNTLR.read()).regs.ich9_spi.hsfc } as *const u16 as usize);
        hsfc &= !HSFC_FCYCLE; /* set read operation */
        hsfc &= !HSFC_FDBC; /* clear byte count */
        /* set byte count */
        hsfc |= ((block_len - 1) << HSFC_FDBC_OFF) as u16 & HSFC_FDBC;
        hsfc |= Hsfc::Fgo as u16; /* start */
        writew_(
            hsfc,
            &unsafe { (*CNTLR.read()).regs.ich9_spi.hsfc } as *const u16 as usize,
        );

        ich_hwseq_wait_for_cycle_complete(timeout as u32, block_len as u32)?;

        ich_read_data(&mut buf[..block_len]);
        addr += block_len as u32;
        buf = &mut buf[block_len..];
        len -= block_len;
    }

    Ok(())
}

/// Fill len bytes from the data array into the fdata/spid registers.
///
/// Note that using len > flash->pgm->spi.max_data_write will trash the registers
/// following the data registers.
pub fn ich_fill_data(data: &[u8]) {
    let mut temp32 = 0u32;

    if data.len() == 0 {
        return;
    }

    for (i, d) in data.iter().enumerate() {
        if i % 4 == 0 {
            temp32 = 0;
        }

        temp32 |= (*d as u32) << ((i % 4) * 8);

        if i % 4 == 3 {
            /* 32 bits are full, write them to regs. */
            writel_(
                temp32,
                &(*CNTLR.read()).data[i - (i % 4)] as *const u8 as usize,
            );
        }
    }

    let i = data.len() - 1;
    if i % 4 != 3 {
        /* write remaining data to regs. */
        writel_(
            temp32,
            &(*CNTLR.read()).data[i - (i % 4)] as *const u8 as usize,
        );
    }
}

pub fn ich_hwseq_write(flash: &SpiFlash, mut addr: u32, mut buf: &[u8]) -> Result<(), Error> {
    let timeout: u16 = 100 * 60;
    let start = addr;
    let mut len = buf.len();

    if addr + buf.len() as u32 > flash.size {
        //error!(
            "Attempt to write 0x{:x}-0x{:x} which is out of chip",
            addr,
            addr + len as u32
        );
        return Err(Error::Generic);
    }

    /* clear FDONE, FCERR, AEL by writing 1 to them (if they are set) */
    writew_(
        readw_(&unsafe { (*CNTLR.read()).regs.ich9_spi.hsfs } as *const u16 as usize),
        &unsafe { (*CNTLR.read()).regs.ich9_spi.hsfs } as *const u16 as usize,
    );

    while len > 0 {
        let mut block_len = core::cmp::min(len, (*CNTLR.read()).data.len());
        if block_len as u32 > (!addr & 0xff) {
            block_len = ((!addr & 0xff) + 1) as usize;
        }

        ich_hwseq_set_addr(addr);

        ich_fill_data(&buf[..block_len]);
        let mut hsfc =
            readw_(&unsafe { (*CNTLR.read()).regs.ich9_spi.hsfc } as *const u16 as usize);
        hsfc &= !HSFC_FCYCLE; /* clear operation */
        hsfc |= 0x2 << HSFC_FCYCLE_OFF; /* set write operation */
        hsfc &= !HSFC_FDBC; /* clear byte count */
        /* set byte count */
        hsfc |= ((block_len as u16 - 1) << HSFC_FDBC_OFF) & HSFC_FDBC;
        hsfc |= Hsfc::Fgo as u16; /* start */
        writew_(
            hsfc,
            &unsafe { (*CNTLR.read()).regs.ich9_spi.hsfc } as *const u16 as usize,
        );

        if ich_hwseq_wait_for_cycle_complete(timeout as u32, block_len as u32).is_err() {
            //error!("SF: write failure at {:x}", addr);
            return Err(Error::Generic);
        }

        addr += block_len as u32;
        buf = &buf[block_len..];
        len -= block_len;
    }
    //debug!(
        "SF: Successfully written {} bytes @ {:x}",
        addr - start,
        start
    );
    Ok(())
}

pub struct IntelSpiFlashOps;

impl SpiFlashOps for IntelSpiFlashOps {
    fn read(&self, flash: &SpiFlash, offset: u32, buf: &mut [u8]) -> Result<(), Error> {
        ich_hwseq_read(flash, offset, buf)
    }

    fn write(&self, flash: &SpiFlash, offset: u32, buf: &[u8]) -> Result<(), Error> {
        ich_hwseq_write(flash, offset, buf)
    }

    fn erase(&self, flash: &SpiFlash, offset: u32, len: usize) -> Result<(), Error> {
        ich_hwseq_erase(flash, offset, len)
    }
}

pub static SPI_FLASH_OPS: IntelSpiFlashOps = IntelSpiFlashOps;

pub fn spi_flash_programmer_probe(spi: &SpiSlave, flash: &mut SpiFlash) -> Result<(), Error> {
    if cfg!(feature = "southbridge_intel_i82801gx") {
        return spi.spi_flash_generic_probe(flash);
    }

    if spi_is_multichip() && spi.spi_flash_generic_probe(flash).is_ok() {
        return Ok(());
    }

    flash.spi = spi.clone();

    ich_hwseq_set_addr(0);

    match ((*CNTLR.read()).hsfs >> 3) & 3 {
        0 => flash.sector_size = 256,
        1 => flash.sector_size = 4096,
        2 => flash.sector_size = 8192,
        3 => flash.sector_size = 65536,
        _ => unreachable!("bound is covered by range 0..=3"),
    }

    flash.size = 1 << (19 + ((*CNTLR.read()).flcomp & 7));

    /*
    flash.ops = Some(&SPI_FLASH_OPS);
    */

    if ((*CNTLR.read()).hsfs & Hsfs::Fdv as u32) != 0 && (((*CNTLR.read()).flmap0 >> 8) & 3) != 0 {
        flash.size += 1 << (19 + (((*CNTLR.read()).flcomp >> 3) & 7));
    }
    //debug!("flash size 0x{:x} bytes", flash.size);

    Ok(())
}

pub fn xfer_vectors(spi: &SpiSlave, vectors: &mut [SpiOp]) -> Result<(), Error> {
    return spi.spi_flash_vector_helper(vectors, spi_ctrlr_xfer);
}

pub const SPI_FPR_SHIFT: u32 = 12;
pub const ICH7_SPI_FPR_MASK: u32 = 0xfff;
pub const ICH9_SPI_FPR_MASK: u32 = 0x1fff;
pub const SPI_FPR_BASE_SHIFT: u32 = 0;
pub const ICH7_SPI_FPR_LIMIT_SHIFT: u32 = 12;
pub const ICH9_SPI_FPR_LIMIT_SHIFT: u32 = 16;
pub const ICH9_SPI_FPR_RPE: u32 = 1 << 15; /* Read Protect */
pub const SPI_FPR_WPE: u32 = 1 << 31; /* Write Protect */

pub fn spi_fpr(base: u32, limit: u32) -> u32 {
    let (mask, limit_shift) = if cfg!(feature = "southbridge_intel_i82801gx") {
        (ICH7_SPI_FPR_MASK, ICH7_SPI_FPR_LIMIT_SHIFT)
    } else {
        (ICH9_SPI_FPR_MASK, ICH9_SPI_FPR_LIMIT_SHIFT)
    };

    let mut ret = ((limit >> SPI_FPR_SHIFT) & mask) << limit_shift;
    ret |= ((base >> SPI_FPR_SHIFT) & mask) << SPI_FPR_BASE_SHIFT;

    ret
}

/// Protect range of SPI flash defined by [start, start+size-1] using Flash
/// Protected Range (FPR) register if available.
/// Returns 0 on success, -1 on failure of programming fpr registers.
pub fn spi_flash_protect(
    _flash: &SpiFlash,
    region: &Region,
    type_: CtrlrProtType,
) -> Result<(), Error> {
    let start = region.offset;
    let end = start + region.size - 1;
    let mut protect_mask = 0;

    let fpr_base = (*CNTLR.read()).fpr;

    let mut reg: u32;
    let mut fpr: u32 = (*CNTLR.read()).fpr_max as u32;
    for i in 0..(*CNTLR.read()).fpr_max {
        reg = unsafe { read32((fpr_base + i as u32) as usize) };
        if reg == 0 {
            fpr = i as u32;
            break;
        }
    }

    if fpr == (*CNTLR.read()).fpr_max as u32 {
        //error!("No SPI FPR free!");
        return Err(Error::Generic);
    }

    match type_ {
        CtrlrProtType::WriteProtect => protect_mask |= SPI_FPR_WPE,
        CtrlrProtType::ReadProtect => {
            if cfg!(feature = "southbridge_intel_i82801gx") {
                return Err(Error::Generic);
            }
            protect_mask |= ICH9_SPI_FPR_RPE;
        }
        CtrlrProtType::ReadWriteProtect => {
            if cfg!(feature = "southbridge_intel_i82801gx") {
                return Err(Error::Generic);
            }
            protect_mask |= ICH9_SPI_FPR_RPE | SPI_FPR_WPE;
        }
    }

    /* Set protected range base and limit */
    reg = spi_fpr(start as u32, end as u32) | protect_mask;

    /* Set the FPR register and verify it is protected */
    unsafe { write32((fpr_base + fpr) as usize, reg) };
    if reg != unsafe { read32((fpr_base + fpr) as usize) } {
        //error!("Unable to set SPI FPR {}", fpr);
        return Err(Error::Generic);
    }

    //debug!(
        "{}: FPR {} is enabled for range 0x{:8x}-0x{:8x}",
        "spi_flash_protect", fpr, start, end
    );

    Ok(())
}

pub fn spi_finalize_ops() -> Result<(), Error> {
    let mut optype = 0;

    let mut spi_config_default = IntelSwseqSpiConfig {
        opprefixes: [0x06, 0x50], /* OPPREFIXES: EWSR and WREN */
        ops: [
            IntelSpiOp::create(0x01, Optype::WriteNoAddr), /* WRSR: Write Status Register */
            IntelSpiOp::create(0x02, Optype::WriteWithAddr), /* BYPR: Byte Program */
            IntelSpiOp::create(0x03, Optype::ReadWithAddr), /* READ: Read Data */
            IntelSpiOp::create(0x05, Optype::ReadNoAddr),  /* RDSR: Read Status Register */
            IntelSpiOp::create(0x20, Optype::WriteWithAddr), /* SE20: Sector Erase 0x20 */
            IntelSpiOp::create(0x9f, Optype::ReadNoAddr),  /* RDID: Read ID */
            IntelSpiOp::create(0xd8, Optype::WriteWithAddr), /* BED8: Block Erase 0xd8 */
            IntelSpiOp::create(0x0b, Optype::ReadWithAddr), /* FAST: Fast Read */
        ],
    };

    let mut spi_config_aai_write = IntelSwseqSpiConfig {
        opprefixes: [0x06, 0x50], /* OPPREFIXES: EWSR and WREN */
        ops: [
            IntelSpiOp::create(0x01, Optype::WriteNoAddr), /* WRSR: Write Status Register */
            IntelSpiOp::create(0x02, Optype::WriteWithAddr), /* BYPR: Byte Program */
            IntelSpiOp::create(0x03, Optype::ReadWithAddr), /* READ: Read Data */
            IntelSpiOp::create(0x05, Optype::ReadNoAddr),  /* RDSR: Read Status Register */
            IntelSpiOp::create(0x20, Optype::WriteWithAddr), /* SE20: Sector Erase 0x20 */
            IntelSpiOp::create(0x9f, Optype::ReadNoAddr),  /* RDID: Read ID */
            IntelSpiOp::create(0xad, Optype::WriteNoAddr), /* Auto Address Increment Word Program */
            IntelSpiOp::create(0x04, Optype::WriteNoAddr), /* Write Disable */
        ],
    };

    let flash = boot_device_spi_flash().ok_or(Error::Generic)?;
    let mut spi_config = &mut spi_config_default;

    // Some older SST SPI flashes support AAI write but use 0xaf opcode for
    // that. Flashrom uses the byte program opcode to write those flashes,
    // so this configuration is fine too. SST25VF064C (id = 0x4b) is an
    // exception.
    if (*flash.read()).vendor == VENDOR_ID_SST && ((*flash.read()).model & 0x00ff) != 0x4b {
        spi_config = &mut spi_config_aai_write;
    }

    if spi_locked() {
        return Ok(());
    }

    override_spi(spi_config);

    let spi_opprefix = (spi_config.opprefixes[0] as u16) | ((spi_config.opprefixes[1] as u16) << 8);
    writew_(spi_opprefix, (*CNTLR.read()).preop as usize);
    for (i, op) in spi_config.ops.iter().enumerate() {
        optype |= ((op.type_ as u16) & 3) << (i * 2);
        writeb_(op.op, (*CNTLR.read()).opmenu[i] as usize);
    }
    writew_(optype, (*CNTLR.read()).optype as usize);

    spi_set_smm_only_flashing(cfg!(feature = "bootmedia_smm_bwp"));

    Ok(())
}

pub fn override_spi(_spi_config: &mut IntelSwseqSpiConfig) {}

pub const BIOS_CNTL: u8 = 0xdc;
pub const BIOS_CNTL_BIOSWE: u8 = 1 << 0;
pub const BIOS_CNTL_BLE: u8 = 1 << 1;
pub const BIOS_CNTL_SMM_BWP: u8 = 1 << 5;

pub fn spi_set_smm_only_flashing(enable: bool) {
    if !(cfg!(feature = "southbridge_intel_i82801gx")
        || cfg!(feature = "southbridge_intel_common_spi_ich9"))
    {
        return;
    }

    let dev = pci_dev(0, 31, 0);
    let mut bios_cntl = pci_read_config8(dev, BIOS_CNTL as u16);

    if enable {
        bios_cntl &= !BIOS_CNTL_BIOSWE;
        bios_cntl |= BIOS_CNTL_BLE | BIOS_CNTL_SMM_BWP;
    } else {
        bios_cntl &= !(BIOS_CNTL_BLE | BIOS_CNTL_SMM_BWP);
        bios_cntl |= BIOS_CNTL_BIOSWE;
    }

    pci_write_config8(dev, BIOS_CNTL as u16, bios_cntl);
}
