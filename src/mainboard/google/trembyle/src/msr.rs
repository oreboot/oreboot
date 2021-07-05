use x86_64::registers::model_specific::Msr;

/// read_msr reads the MSR, compares it to a known good value, and prints a message indicating whether they match
fn read_msr(w: &mut impl core::fmt::Write, address: u32, expected_value: u64) {
    write!(w, "{:x} ", address).expect("Failed to write!");
    let read_value = unsafe { Msr::new(address).read() };
    let d = if read_value != expected_value { "DIFF:" } else { "SAME:" };
    writeln!(w, "{}{:x} got {:x}\r", d, expected_value, read_value,).expect("Failed to write!");
}

/// read_write_msr reads the MSR, compares the existing contents to the input value,
/// then writes the input value to the MSR, finally the written to MSR is read and compared
/// to the input value. The comparison messages are printed
fn read_write_msr(w: &mut impl core::fmt::Write, address: u32, value: u64) {
    let mut msr = Msr::new(address);
    read_msr(w, address, value);
    unsafe {
        msr.write(value);
    }
    let checked_value = unsafe { msr.read() }; // Some msrs error on read so this has to be unsafe
    writeln!(w, " -- wrmsr: and got {:x}; \r", checked_value).expect("Failed to write!");
}

/// write_msr writes the input value to an MSR without checking
fn _write_msr(_w: &mut impl core::fmt::Write, address: u32, value: u64) {
    unsafe { Msr::new(address).write(value) }
}

/// The msrs function configures the MSRs of an AMD Rome CPU, read-only MSRs are checked and compared to known values
pub fn msrs(w: &mut impl core::fmt::Write) {
    read_write_msr(w, 0xc000_0080, 0xd01); // EFER - Extended Feature Enable
    read_write_msr(w, 0xc000_0081, 0x23001000000000); // STAR - SYSCALL Target Address
    read_write_msr(w, 0xc000_0082, 0xffffffff99a00000); // STAR64 - Long Mode SYSCALL Target Address
    read_write_msr(w, 0xc000_0083, 0xffffffff99a01230); // STARCOMPAT - Compatibility SYSCALL Target Address
    read_write_msr(w, 0xc000_0084, 0x47700); // SYSCALL_FLAG_MASK

    read_write_msr(w, 0xc000_00e7, 0x16497beaa820); // MPerfReadOnly - Read-Only Max Performance Frequency Clock Count
    read_write_msr(w, 0xc000_00e8, 0x1fafceec9a6e); // APerfReadOnly - Read-Only Actual Performance Frequency Clock Count

    // 0xc000_00e9                              // IRPerfCount - Instructions Retired Performance Count
    read_write_msr(w, 0xc000_0100, 0x5b6c50); // FS_BASE - expanded FS segment base
    read_write_msr(w, 0xc000_0101, 0xffffffff9a031000); // GS_BASE - expanded GS segment base
                                                        // 0xc000_102                               // KernelGSbase
                                                        // 0xc000_103                               // TSC_AUX - Auxiliary Time Stamp Counter
    read_write_msr(w, 0xc000_0104, 0x100000000); // TscRateMsr - Time Stamp Counter Ratio

    // NOTE: This is copied from Rome, but gets stuck here with a Ryzen 5 3400G
    // (Picasso); it's possibly not defined there
    // see 55570-B1 Rev 3.15 - Jul 9, 2020 / PPR for AMD Family 17h Model 18h B1
    // https://www.amd.com/system/files/TechDocs/55570-B1-PUB.zip
    // https://www.amd.com/en/support/tech-docs?keyword=ppr
    // docs saved: https://bugzilla.kernel.org/show_bug.cgi?id=206537
    // read_write_msr(w, 0xc000_0200, 0x800); // 020x L3 QOS Bandwidth Control
    // read_write_msr(w, 0xc000_0201, 0x800);
    // read_write_msr(w, 0xc000_0202, 0x800);
    // read_write_msr(w, 0xc000_0203, 0x800);
    // read_write_msr(w, 0xc000_0204, 0x800);
    // read_write_msr(w, 0xc000_0205, 0x800);
    // read_write_msr(w, 0xc000_0206, 0x800);
    // read_write_msr(w, 0xc000_0207, 0x800);
    // read_write_msr(w, 0xc000_0208, 0x800);
    // read_write_msr(w, 0xc000_0209, 0x800);
    // read_write_msr(w, 0xc000_020a, 0x800);
    // read_write_msr(w, 0xc000_020b, 0x800);
    // read_write_msr(w, 0xc000_020c, 0x800);
    // read_write_msr(w, 0xc000_020d, 0x800);
    // read_write_msr(w, 0xc000_020e, 0x800);
    // read_write_msr(w, 0xc000_020f, 0x800); // 020x L3 QOS Bandwidth Control

    read_write_msr(w, 0xc000_0410, 0x1001028); // McaIntrCfg - MCA Interrupt Configuration

    // LS - Load Store Unit
    //c000_2000                                 // MCA_CTL_LS - LS Machine Check Control
    //c000_2001                                 // MCA_STATUS_LS - LS Machine Check Status Thread 0
    //c000_2002                                 // MCA_ADDR_LS - LS Machine Check Address Thread 0
    //c000_2003                                 // MCA_MISC0_LS - LS Machine Check Miscellaneous 0 Thread 0
    read_write_msr(w, 0xc000_2004, 0x70000007d); // MCA_CONFIG_LS - LS Machine Check Configuration
    read_write_msr(w, 0xc000_2005, 0xb000000000); // MCA_IPID_LS - LS IP Identification

    //c000_2006                                     // MCA_SYND_LS - LS Machine Check Syndrome Thread 0
    //c000_2008                                     // MCA_DESTAT_LS - LS Machine Check Deferred Error Status Thread 0
    //c000_2009                                     // MCA_DEADDR_LS - LS Deffered Error Address Thread 0

    // IF - Instruction Fetch Unit
    read_write_msr(w, 0xc000_2014, 0x300000079); // MCA_CONFIG_IF - IF Machine Check Configuration
    read_write_msr(w, 0xc000_2015, 0x100b000000000); // MCA_IPID_IF - IF IP Identification

    // L2 - L2 Cache Unit
    read_write_msr(w, 0xc000_2024, 0x50000007f); // MCA_CONFIG_L2 - L2 Machine Check Configuration
    read_write_msr(w, 0xc000_2025, 0x200b000000000); // MCA_IPID_L2 - L2 IP Identification

    // DE - Decode Unit
    read_write_msr(w, 0xc000_2034, 0x300000079); // MCA_CONFIG_DE - DE Machine Check Configuration
    read_write_msr(w, 0xc000_2035, 0x300b000000000); // MCA_IPID_DE - DE IP Identification

    // EX - Execution Unit
    read_write_msr(w, 0xc000_2054, 0x300000079); // MCA_CONFIG_EX - EX Machine Check Configuration
    read_write_msr(w, 0xc000_2055, 0x500b000000000); // MCA_IPID_EX - EX IP Identification

    // FP - Floating Point Unit
    read_write_msr(w, 0xc000_2064, 0x300000079); // MCA_CONFIG_FP - L2 Machine Check Configuration
    read_write_msr(w, 0xc000_2065, 0x600b000000000); // MCA_IPID_FP - IF IP Identification

    // L3 - L3 Cache Unit
    read_write_msr(w, 0xc000_2074, 0x50000007f); // MCA_CONFIG_L3 - L3 Machine Check Configuration
    read_write_msr(w, 0xc000_2075, 0x700b020350000); // MCA_IPID_L3 - L3 IP Identification
    read_write_msr(w, 0xc000_2084, 0x50000007f); // MCA_CONFIG_L3
    read_write_msr(w, 0xc000_2085, 0x700b020350100); // MCA_IPID_L3
    read_write_msr(w, 0xc000_2094, 0x50000007f); // MCA_CONFIG_L3
    read_write_msr(w, 0xc000_2095, 0x700b020350200); // MCA_IPID_L3
    read_write_msr(w, 0xc000_20a4, 0x50000007f); // MCA_CONFIG_L3
    read_write_msr(w, 0xc000_20a5, 0x700b020350300); // MCA_IPID_L3
    read_write_msr(w, 0xc000_20b4, 0x50000007f); // MCA_CONFIG_L3
    read_write_msr(w, 0xc000_20b5, 0x700b020750000); // MCA_IPID_L3
    read_write_msr(w, 0xc000_20c4, 0x50000007f); // MCA_CONFIG_L3
    read_write_msr(w, 0xc000_20c5, 0x700b020750100); // MCA_IPID_L3
    read_write_msr(w, 0xc000_20d4, 0x50000007f); // MCA_CONFIG_L3
    read_write_msr(w, 0xc000_20d5, 0x700b020750200); // MCA_IPID_L3
    read_write_msr(w, 0xc000_20e4, 0x50000007f); // MCA_CONFIG_L3
    read_write_msr(w, 0xc000_20e5, 0x700b020750300); // MCA_IPID_L3

    // p251 PPR for AMD Family 17h Model 18h B1
    // MP5 - Microprocessor5 Management Controller
    read_write_msr(w, 0xc000_20f4, 0x300000079); // MCA_CONFIG_MP5 - MP5 Machine Check Configuration
    read_write_msr(w, 0xc000_20f5, 0x2000130430400); // MCA_IPID_MP5 - MP5 IP Identification

    // PB - Parameter Block
    read_write_msr(w, 0xc000_2104, 0x10000007b);
    read_write_msr(w, 0xc000_2105, 0x530082900);

    // UMC - Unified Memory Controller
    read_write_msr(w, 0xc000_2114, 0x70000007d); // MCA_CONFIG_UMC - UMC Machine Check Configuration
    read_write_msr(w, 0xc000_2115, 0x9600050f00); // MCA_IPID_UMC - UMC IP Identification

    //0xc000_2124                               // UMC
    //0xc000_2125                               // UMC

    // CS - Coherent Slave
    read_write_msr(w, 0xc000_2134, 0x50000007f); // MCA_CONFIG_CS - CS Machine Check Configuration
    read_write_msr(w, 0xc000_2135, 0x2002e00000001); // MCA_IPID_CS - CS IP Identification
    read_write_msr(w, 0xc000_2144, 0x50000007f); // MCA_CONFIG_CS - CS Machine Check Configuration
    read_write_msr(w, 0xc000_2145, 0x2002e00000101); // MCA_IPID_CS - CS IP Identification

    //0xc000_2154								// MCA_CONFIG_CS - CS Machine Check Configuration
    //0xc000_2155								// MCA_IPID_CS - CS IP Identification

    // NBIO - Northbridge IO Unit
    read_write_msr(w, 0xc000_2164, 0x70000007d); // MCA_CONFIG_NBIO - NBIO Machine Check Configuration
    read_write_msr(w, 0xc000_2165, 0x1813b17000); // MCA_IPID_NBIO - NBIO IP Identification

    // PCIE - PCIe Root Port
    read_write_msr(w, 0xc000_2174, 0x70000007d); // MCA_CONFIG_PCIE - PCIE Machine Check Configuration
    read_write_msr(w, 0xc000_2175, 0x46115c0000); // MCA_IPID_PCIE - PCIE IP Identification

    // SMU - System Management Controller Unit
    read_write_msr(w, 0xc000_2184, 0x300000079); // MCA_CONFIG_SMU - SMU Machine Check Configuration
    read_write_msr(w, 0xc000_2185, 0x1000103b30400); // MCA_IPID_SMU - SMU IP Identification

    // PSP - Platform Security Processor
    read_write_msr(w, 0xc000_2194, 0x300000079); // MCA_CONFIG_PSP - PSP Machine Check Configuration
    read_write_msr(w, 0xc000_2195, 0x100ff03830400); // MCA_IPID_PSP - PSP IP Identification

    // PB - Parameter Block
    read_write_msr(w, 0xc000_21a4, 0x10000007b); // MCA_CONFIG_PB - PB Machine Check Configuration
    read_write_msr(w, 0xc000_21a5, 0x50005e100); // MCA_IPID_PB - PB IP Identification

    // PIE - Power Management, Interrupts, Etc
    read_write_msr(w, 0xc000_21b4, 0x70000007d); // MCA_CONFIG_PIE - PIE Machine Check Configuration
    read_write_msr(w, 0xc000_21b5, 0x1002e00001e01); // MCA_IPID_PIE - PIE IP Identification

    // Performance Event Select
    // 0xc001_000[1..3] 						// PERF_LEGACY_CTL - Performance Event Select
    // 0xc001_000[4..7] 						// PERF_LEGACY_CTR - Performance Event Counter

    // read_write_msr(w, 0xc001_0010, 0xf40000);
    // 0xc001_0010		 						// SYS_CFG - System Configuration #*#
    read_write_msr(w, 0xc001_0015, 0x0000_0000_0900_0010); // HWCR - Hardware Configuration

    // 0xc001_001[6 / 8]	 					// IORR_BASE - IO Range Base
    // 0xc001_001[7 / 9]	 					// IORR_BASE - IO Range Mask

    // Top Of Memory
    read_write_msr(w, 0xc001_001a, 0x80000000); // TOP_MEM
    read_write_msr(w, 0xc001_001d, 0x450000000); // TOM2

    // read_write_msr(w, 0xc001_0022, 0x200); 		// McExcepRedir - Machine Check Exception Redirection

    // Processor Name String
    read_write_msr(w, 0xc001_0030, 0x4359504520444d41);
    // read_write_msr(w, 0xc001_0031, 0x3233203235343720);
    read_write_msr(w, 0xc001_0031, 0x4e20535554495420); // graffiti
    read_write_msr(w, 0xc001_0032, 0x72502065726f432d);
    read_write_msr(w, 0xc001_0033, 0x20726f737365636f);
    read_write_msr(w, 0xc001_0034, 0x2020202020202020);
    read_write_msr(w, 0xc001_0035, 0x20202020202020);

    // 0xc001_005[0..4]	 						// IO Traps

    read_write_msr(w, 0xc001_0056, 0x28000b2); // SmiTrigIoCycle - SMI Trigger IO Cycle
    read_write_msr(w, 0xc001_0058, 0xe0000021); // MmioCfgBaseAddr - MMIO Configuration Base Address

    //   read_write_msr(w, 0xc001_0061, 0x20); error on write.  // PStateCurLim - P-state Current Limit
    // 0xc001_0062								// PStateCtl - P-state Control
    // 0xc001_0063								// PStateStat - P-state Status
    read_write_msr(w, 0xc001_0064, 0x8000000045d2085e); // PStateDef
    read_write_msr(w, 0xc001_0065, 0x8000000045160a64);
    read_write_msr(w, 0xc001_0066, 0x8000000043da0c5a);
    // 0xc001_006[7..B]								// PstateDef

    read_write_msr(w, 0xc001_0073, 0x813); // CStateBaseAddr - C-state Base Address
    read_write_msr(w, 0xc001_0074, 0x289); // CpuWdtCfg - CPU Watchdog Timer
    read_write_msr(w, 0xc001_0111, 0xafba2000); // SMM_BASE - SMM Base Address
    read_write_msr(w, 0xc001_0112, 0xac000000); // SMMAddr - SMM TSeg Base Address
    read_write_msr(w, 0xc001_0113, 0xfffffc006003); // SMMMask - SMM TSeg Mask

    // 0xc001_0114  VM_CR Virtual Machine Control
    // 0xc001_0115  IGNEE
    // 0xc001_0116  SMM_CTL - SMM Control *#*#
    // 0xc001_0117  VM_HSAVE_PA - Host save physical address
    // 0xc001_0118  SVM Lock Key
    // 0xc001_011a  Local SMI status
    // 0xc001_011b  AVIC Doorbell
    // 0xc001_011e  VM Page flush
    // 0xc001_0130  Guest Host Communication Block
    // 0xc001_0131  SEV Status
    // 0xc001_0140  OS Visible Work-around Length
    // 0xc001_0141  OS Visible Work-around Status

    // 0xc001_020[0,2,4,6,8,A] 			// PERF_CTL - Performance Event Select
    // 0xc001_020[1,3,5,7,9,B] 			// PERF_CTR - Performance Event Counter
    read_write_msr(w, 0xc001_020b, 0xffff); // PERF_CTR - Performance Event Counter

    // 0xc001_023[0,2,4,6,8,A] 			// ChL3PmcCfg - L3 Performance Event Select
    // 0xc001_023[1,3,5,7,9,B] 			// ChL3Pmc - L3 Performance Event Counter
    // 0xc001_024[0,2,4,6]	 			// DF_PERF_CTL - Data Fabric Performance Event Select
    // 0xc001_024[1,3,5,7]	 			// DF_PERF_CTR - Data Fabric Performance Event Counter
    read_write_msr(w, 0xc001_0292, 0x40b8012); // undoc*
    read_write_msr(w, 0xc001_0293, 0x104886); // undoc*
    read_write_msr(w, 0xc001_0294, 0xf8e847f00008912); // undoc*
    read_write_msr(w, 0xc001_0296, 0x484848); // undoc*
    read_write_msr(w, 0xc001_0297, 0x380000fc000); // undoc*
    read_write_msr(w, 0xc001_0299, 0xa1003); // RAPL_PWR_UNIT
    read_write_msr(w, 0xc001_029a, 0x9731905d); // CORE_ENERGY_STAT
    read_write_msr(w, 0xc001_029b, 0x95073877); // PKG_ENERGY_STAT

    // NOTE: stuck on Picasso
    //read_write_msr(w, 0xc001_02b3, 0xfff0); // undoc

    // read_write_msr(w, 0xc001_02f0, 0x1);	    // PPIN_CTL - Protected Precessor Inventory Number Control
    // 0xc001_02f1							    // PPIN_CTL - Protected Precessor Inventory Number

    // Machine Check Control Masks
    read_write_msr(w, 0xc001_0400, 0x600); // MCA_CTL_MASK_LS - LS Machine Check Control Mask
    read_write_msr(w, 0xc001_0401, 0x2c00); // MCA_CTL_MASK_IF
    read_write_msr(w, 0xc001_0402, 0x8); // MCA_CTL_MASK_L2

    // 0xc001_0403 						    // MCA_CTL_MASK_DE
    // 0xc001_0405 						    // MCA_CTL_MASK_EX

    read_write_msr(w, 0xc001_0406, 0x40); // MCA_CTL_MASK_FP
    read_write_msr(w, 0xc001_0407, 0x80); // MCA_CTL_MASK_L3
    read_write_msr(w, 0xc001_0408, 0x80); // MCA_CTL_MASK_L3
    read_write_msr(w, 0xc001_0409, 0x80); // MCA_CTL_MASK_L3
    read_write_msr(w, 0xc001_040a, 0x80); // MCA_CTL_MASK_L3
    read_write_msr(w, 0xc001_040b, 0x80); // MCA_CTL_MASK_L3
    read_write_msr(w, 0xc001_040c, 0x80); // MCA_CTL_MASK_L3
    read_write_msr(w, 0xc001_040d, 0x80); // MCA_CTL_MASK_L3
    read_write_msr(w, 0xc001_040e, 0x80); // MCA_CTL_MASK_L3

    // 0xc001_040f						    // MCA_CTL_MASK_MP5
    // 0xc001_0410						    // MCA_CTL_MASK_PB
    // 0xc001_0411						    // MCA_CTL_MASK_UMC
    // 0xc001_0412						    // MCA_CTL_MASK_UMC

    read_write_msr(w, 0xc001_0413, 0x2); // MCA_CTL_MASK_CS
    read_write_msr(w, 0xc001_0414, 0x2); // MCA_CTL_MASK_CS

    // 0xc001_0415						    // MCA_CTL_MASK_CS - CCIX

    read_write_msr(w, 0xc001_0416, 0x6); // MCA_CTL_MASK_NBIO

    // 0xc001_0417						    // MCA_CTL_MASK_PCIE

    read_write_msr(w, 0xc001_0419, 0x3c0); // MCA_CTL_MASK_PSP

    // 0xc001_041a						    // MCA_CTL_MASK_PB
    // 0xc001_041b						    // MCA_CTL_MASK_PIE -

    read_write_msr(w, 0xc001_1000, 0x8000); // undoc
    read_write_msr(w, 0xc001_1002, 0x219c91a9); // CPUID_7_Features
    read_write_msr(w, 0xc001_1003, 0x1); // CPUID_PWR_THERM

    // boots ok, does not fix apic timer verification
    read_write_msr(w, 0xc001_1004, 0x7ed8320b178bfbff); // CPUID_Features
    read_write_msr(w, 0xc001_1005, 0x75c237ff2fd3fbff); // CPUID_ExtFeatures
    read_write_msr(w, 0xc001_100c, 0xff711b00); // undoc*

    // 0xc001_1019 									    // DR1_ADDR_MASK - Address Mask for DR1 Breakpoint
    // 0xc001_101a 									    // DR2_ADDR_MASK - Address Mask for DR2 Breakpoint
    // 0xc001_101b 									    // DR3_ADDR_MASK - Address Mask for DR3 Breakpoint
    read_write_msr(w, 0xc001_1020, 0x6404000000000); // undoc
    read_write_msr(w, 0xc001_1021, 0x2000000); // undoc
    read_write_msr(w, 0xc001_1022, 0xc000000002500000); // undoc* bits are undoc
    read_write_msr(w, 0xc001_1023, 0x2000000000020); // TW_CFG - Table Walker Config

    // 0xc001_1027									// DR0_ADDR_MASK - Addressmask for DR0 Breakpoints
    read_write_msr(w, 0xc001_1028, 0x200248000d4); // undoc
    read_write_msr(w, 0xc001_1029, 0x3000310e08002); // undoc
    read_write_msr(w, 0xc001_102a, 0x38080); // undoc
    read_write_msr(w, 0xc001_102b, 0x2008cc17); // undoc*
    read_write_msr(w, 0xc001_102c, 0x309c70000000000); // undoc
    read_write_msr(w, 0xc001_102d, 0x101c00000010); // undoc
    read_write_msr(w, 0xc001_102e, 0x12024000000000); // undoc

    // boots ok, does not fix apic timer
    // 0xc001_1030                              // IBS Fetch Control
    // 0xc001_1031                              // IBS Linear Address
    // 0xc001_1032                              // IBS Fetch Physical Address
    // 0xc001_1033                              // IBS Execution Control
    // 0xc001_1034                              // IBS Op Logical Address
    // 0xc001_1035                              // IBS Op Data
    // 0xc001_1036                              // IBS Op Data 2
    // 0xc001_1037                              // IBS Op Data 3
    // 0xc001_1038                              // IBS DC Linear Address
    // 0xc001_1039                              // IBS DC Physical Address
    read_msr(w, 0xc001_103a, 0x100); // gpf on write // IBS Control

    // 0xc001_103b IBS Branch Target Address
    // 0xc001_103c IBS Fetch Control Extended

    read_write_msr(w, 0xc001_1074, 0xa000000000000000); // undoc
    read_write_msr(w, 0xc001_1076, 0x14); // undoc
    read_write_msr(w, 0xc001_1077, 0x6d00000000000000); // undoc
    read_write_msr(w, 0xc001_1083, 0x38d6b5ad1bc6b5ad); // undoc
    read_write_msr(w, 0xc001_1092, 0x57840a05); // undoc*
    read_write_msr(w, 0xc001_1093, 0x6071f9fc); // undoc
    read_write_msr(w, 0xc001_1094, 0x110c); // undoc
    read_write_msr(w, 0xc001_1097, 0x5dbf); // undoc

    // boots ok, does not fix apic timer
    // NOTE: stuck on Picasso
    //read_write_msr(w, 0xc001_1098, 0xa); // undoc
    // commented for Picasso just to be sure
    //read_write_msr(w, 0xc001_10a2, 0xc9000000); // undoc
    //read_write_msr(w, 0xc001_10dc, 0x3030018cf757); // undoc

    // if these next four are enabled, we get past apic verification as failure
    //read_write_msr(w, 0xc001_10dd, 0x13bcff); // undoc
    //read_write_msr(w, 0xc001_10e1, 0x410e50400c2cb4e0); // undoc

    // if one of these two are disabled, apic fails.
    //read_write_msr(w, 0xc001_10e2, 0x2afa00082018); // It's this one. It's undocumented.
    //read_write_msr(w, 0xc001_10e3, 0x1); // undoc

    // one(w, 0x10, 0x6780a9b73d4, true); // TSC - Time Stamp Counter
    // one(w, 0x1b, 0xfee00900, true); // APIC_BAR - APIC Base Address
    // 0x2A EBL_CR_POWERON - Cluster ID
    // 0x48 SPEC_CTRL - Speculative Control
    // 0x49 PRED_CMD - Prediction Command
    // one(w, 0x8b, 0x8301038, true); // PATCH_LEVEL
    // read_write_msr(w, 0xe7, 0x322248bbed); don't boether. // MPERF - Max Performance Count
    // read_write_msr(w, 0xe8, 0x3a9d34b62b); don't bother // APERF - Actual Performance Count
    // one(w, 0xfe, 0x508, true); don't bother // MTRRcap - MTRR Capabilities
    // 0x174 SYSENTER_CS
    // 0x175 SYSENTER_ESP
    // 0x176 SYSENTER_EIP
    // one(w, 0x179, 0x11c, true); // MCG_CAP - Global Machine Check capabilities
    // 0x17a MCG_STAT - Global Machine Check Status
    // 0x17b MCG_CTL - Global Machine Check Exception Reporting Control
    // 0x1d9 DBG_CTL_MSR - Debug Control
    // 0x1db BR_FROM - Last Branch From IP
    // 0x1dc BR_TO - Last Branch To IP
    // 0x1dd LastExcpFromIp
    // 0x1de LastExcpToIp

    read_write_msr(w, 0x200, 0x6); // MtrrVarBase - Variable Size MTRRs Base
    read_write_msr(w, 0x201, 0xffff80000800); // MtrrVarMask - Variable Size MTRRs Mask
    read_write_msr(w, 0x202, 0x80000006); // MtrrVarBase
    read_write_msr(w, 0x203, 0xffffe0000800); // MtrrVarMask
    read_write_msr(w, 0x204, 0xa0000006); // MtrrVarBase
    read_write_msr(w, 0x205, 0xfffff0000800); // MtrrVarMask
    read_write_msr(w, 0x206, 0xff000005); // MtrrVarBase
    read_write_msr(w, 0x207, 0xffffff000800); // MtrrVarMask
    read_write_msr(w, 0x208, 0xac000000); // MtrrVarBase
    read_write_msr(w, 0x209, 0xfffffc000800); // MtrrVarMask
    read_write_msr(w, 0x20a, 0xa2fa0000); // MtrrVarBase
    read_write_msr(w, 0x20b, 0xffffffff0800); // MtrrVarMask

    // 0x20c // MtrrVarBase
    // 0x20d // MtrrVarMask
    // 0x20e // MtrrVarBase
    // 0x20f // MtrrVarMask

    // Fixed-Size MTRRs
    read_write_msr(w, 0x250, 0x606060606060606); // MtrrFix_64K
    read_write_msr(w, 0x258, 0x606060606060606); // MtrrFix_16K_0
    read_write_msr(w, 0x259, 0x404040404040404); // MtrrFix_16K_1
    read_write_msr(w, 0x268, 0x505050505050505); // MtrrFix_4K_0
    read_write_msr(w, 0x269, 0x505050505050505); // MtrrFix_4K_1
    read_write_msr(w, 0x26a, 0x505050505050505); // MtrrFix_4K_2
    read_write_msr(w, 0x26b, 0x505050505050505); // MtrrFix_4K_3
    read_write_msr(w, 0x26c, 0x505050505050505); // MtrrFix_4K_4
    read_write_msr(w, 0x26d, 0x505050505050505); // MtrrFix_4K_5
    read_write_msr(w, 0x26e, 0x505050505050505); // MtrrFix_4K_6
    read_write_msr(w, 0x26f, 0x505050505050505); // MtrrFix_4K_7
    read_write_msr(w, 0x277, 0x7040600070406); // PAT - Page Attribute Table
    read_write_msr(w, 0x2ff, 0xc00); // MTRRdefType - MTRR Default Type

    // these need X2APICEN to be useful
    // 0x802 APIC_ID
    // 0x803 ApicVersion
    // 0x808 TPR - Task Priority
    // 0x809 ArbitrationPriority
    // 0x80a ProcessorPriority
    // 0x80b EOI - End Of Interrupt
    // 0x80d LDR - Logical Destination Register
    // 0x80f SVR - Spurious Int
    // 0x81[0..7] ISR - In Service Register
    // 0x81[8..F] TMR - Trigger Mode Register
    // 0x82[0..7] IRR - Interrupt Request Register
    // 0x828 ESR - Error Status Register
    // 0x830 InterruptCommand
    // 0x832 TimerLvtEntry
    // 0x833 ThermalLvtEntry
    // 0x834 PerformanceCounterLvtEntry
    // 0x83[5..6] LVTLINT
    // 0x837 ErrorLvtEntry
    // 0x838 TimerInitialCount
    // 0x839 TimerCurrentCount
    // 0x83E TimerDivideConfiguration
    // 0x83F SelfIPI
    // 0x840 ExtendedApicFeature
    // 0x841 ExtendedApicControl
    // 0x842 SpecificEndOfInterrupt
    // 0x848 InterruptEnable0
    // 0x84[9..F] InterruptEnable71
    // 0x85[0..3] ExtendedInterruptLvtEntries
    // 0xc81 L3QoSCfg1 - L3 QoS Configuration
    // 0xc8d QM_EVT_SEL - Monitoring Event Select
    // 0xc8e QM_CTR - QOS L3 Counter
    // 0xc9[0..F] L3_QosAllocMask - L3 QOS Allocation Mask
}
