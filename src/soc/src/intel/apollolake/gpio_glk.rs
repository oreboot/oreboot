/* SPDX-License-Identifier: GPL-2.0-or-later */

use crate::intel::{
    apollolake::pcr_ids::{PID_GPIO_AUDIO, PID_GPIO_N, PID_GPIO_NW, PID_GPIO_SCC},
    common::block::gpio::{
        gpio_defs::{
            PAD_CFG0_LOGICAL_RESET_DEEP, PAD_CFG0_LOGICAL_RESET_PLTRST,
            PAD_CFG0_LOGICAL_RESET_PWROK,
        },
        PadCommunity, PadGroup, ResetMapping,
    },
};
use alloc::vec::Vec;
use util::nvramtool::cbfs::align_up;

/* North West community pads */
/* For DFx GPIO, Display, USB, I2C, UART, and Thermal GPIO*/

pub const NW_OFFSET: u16 = 0;
pub const GPIO_0: u16 = NW_OFFSET + 0;
pub const GPIO_1: u16 = NW_OFFSET + 1;
pub const GPIO_2: u16 = NW_OFFSET + 2;
pub const GPIO_3: u16 = NW_OFFSET + 3;
pub const GPIO_4: u16 = NW_OFFSET + 4;
pub const GPIO_5: u16 = NW_OFFSET + 5;
pub const GPIO_6: u16 = NW_OFFSET + 6;
pub const GPIO_7: u16 = NW_OFFSET + 7;
pub const GPIO_8: u16 = NW_OFFSET + 8;
pub const GPIO_9: u16 = NW_OFFSET + 9;
pub const GPIO_10: u16 = NW_OFFSET + 10;
pub const GPIO_11: u16 = NW_OFFSET + 11;
pub const GPIO_12: u16 = NW_OFFSET + 12;
pub const GPIO_13: u16 = NW_OFFSET + 13;
pub const GPIO_14: u16 = NW_OFFSET + 14;
pub const GPIO_15: u16 = NW_OFFSET + 15;
pub const GPIO_16: u16 = NW_OFFSET + 16;
pub const GPIO_17: u16 = NW_OFFSET + 17;
pub const GPIO_18: u16 = NW_OFFSET + 18;
pub const GPIO_19: u16 = NW_OFFSET + 19;
pub const GPIO_20: u16 = NW_OFFSET + 20;
pub const GPIO_21: u16 = NW_OFFSET + 21;
pub const GPIO_22: u16 = NW_OFFSET + 22;
pub const GPIO_23: u16 = NW_OFFSET + 23;
pub const GPIO_24: u16 = NW_OFFSET + 24;
pub const GPIO_25: u16 = NW_OFFSET + 25;
pub const GPIO_26: u16 = NW_OFFSET + 26;
pub const GPIO_27: u16 = NW_OFFSET + 27;
pub const GPIO_28: u16 = NW_OFFSET + 28;
pub const GPIO_29: u16 = NW_OFFSET + 29;
pub const GPIO_30: u16 = NW_OFFSET + 30;
pub const GPIO_31: u16 = NW_OFFSET + 31;
pub const GPIO_32: u16 = NW_OFFSET + 32;
pub const GPIO_33: u16 = NW_OFFSET + 33;
pub const GPIO_34: u16 = NW_OFFSET + 34;
pub const GPIO_35: u16 = NW_OFFSET + 35;
pub const GPIO_36: u16 = NW_OFFSET + 36;
pub const GPIO_37: u16 = NW_OFFSET + 37;
pub const GPIO_38: u16 = NW_OFFSET + 38;
pub const GPIO_39: u16 = NW_OFFSET + 39;
pub const GPIO_40: u16 = NW_OFFSET + 40;
pub const GPIO_41: u16 = NW_OFFSET + 41;
pub const GPIO_42: u16 = NW_OFFSET + 42;
pub const GPIO_43: u16 = NW_OFFSET + 43;
pub const GPIO_44: u16 = NW_OFFSET + 44;
pub const GPIO_45: u16 = NW_OFFSET + 45;
pub const GPIO_46: u16 = NW_OFFSET + 46;
pub const GPIO_47: u16 = NW_OFFSET + 47;
pub const GPIO_48: u16 = NW_OFFSET + 48;
pub const GPIO_49: u16 = NW_OFFSET + 49;
pub const GPIO_50: u16 = NW_OFFSET + 50;
pub const GPIO_51: u16 = NW_OFFSET + 51;
pub const GPIO_52: u16 = NW_OFFSET + 52;
pub const GPIO_53: u16 = NW_OFFSET + 53;
pub const GPIO_54: u16 = NW_OFFSET + 54;
pub const GPIO_55: u16 = NW_OFFSET + 55;
pub const GPIO_56: u16 = NW_OFFSET + 56;
pub const GPIO_57: u16 = NW_OFFSET + 57;
pub const GPIO_58: u16 = NW_OFFSET + 58;
pub const GPIO_59: u16 = NW_OFFSET + 59;
pub const GPIO_60: u16 = NW_OFFSET + 60;
pub const GPIO_61: u16 = NW_OFFSET + 61;
pub const GPIO_62: u16 = NW_OFFSET + 62;
pub const GPIO_63: u16 = NW_OFFSET + 63;
pub const GPIO_64: u16 = NW_OFFSET + 64;
pub const GPIO_65: u16 = NW_OFFSET + 65;
pub const GPIO_66: u16 = NW_OFFSET + 66;
pub const GPIO_67: u16 = NW_OFFSET + 67;
pub const GPIO_68: u16 = NW_OFFSET + 68;
pub const GPIO_69: u16 = NW_OFFSET + 69;
pub const GPIO_70: u16 = NW_OFFSET + 70;
pub const GPIO_71: u16 = NW_OFFSET + 71;
pub const GPIO_72: u16 = NW_OFFSET + 72;
pub const GPIO_73: u16 = NW_OFFSET + 73;
pub const GPIO_74: u16 = NW_OFFSET + 74;
pub const GPIO_75: u16 = NW_OFFSET + 75;
pub const GPIO_211: u16 = NW_OFFSET + 76;
pub const GPIO_212: u16 = NW_OFFSET + 77;
pub const GPIO_213: u16 = NW_OFFSET + 78;
pub const GPIO_214: u16 = NW_OFFSET + 79;
pub const TOTAL_NW_PADS: u16 = 80;

/* North Community Pads */
/* For power management GPIO, I2C, Display, LPC/eSPI, SPI */

pub const N_OFFSET: u16 = NW_OFFSET + 80;
pub const GPIO_76: u16 = N_OFFSET + 0;
pub const GPIO_77: u16 = N_OFFSET + 1;
pub const GPIO_78: u16 = N_OFFSET + 2;
pub const GPIO_79: u16 = N_OFFSET + 3;
pub const GPIO_80: u16 = N_OFFSET + 4;
pub const GPIO_81: u16 = N_OFFSET + 5;
pub const GPIO_82: u16 = N_OFFSET + 6;
pub const GPIO_83: u16 = N_OFFSET + 7;
pub const GPIO_84: u16 = N_OFFSET + 8;
pub const GPIO_85: u16 = N_OFFSET + 9;
pub const GPIO_86: u16 = N_OFFSET + 10;
pub const GPIO_87: u16 = N_OFFSET + 11;
pub const GPIO_88: u16 = N_OFFSET + 12;
pub const GPIO_89: u16 = N_OFFSET + 13;
pub const GPIO_90: u16 = N_OFFSET + 14;
pub const GPIO_91: u16 = N_OFFSET + 15;
pub const GPIO_92: u16 = N_OFFSET + 16;
pub const GPIO_93: u16 = N_OFFSET + 17;
pub const GPIO_94: u16 = N_OFFSET + 18;
pub const GPIO_95: u16 = N_OFFSET + 19;
pub const GPIO_96: u16 = N_OFFSET + 20;
pub const GPIO_97: u16 = N_OFFSET + 21;
pub const GPIO_98: u16 = N_OFFSET + 22;
pub const GPIO_99: u16 = N_OFFSET + 23;
pub const GPIO_100: u16 = N_OFFSET + 24;
pub const GPIO_101: u16 = N_OFFSET + 25;
pub const GPIO_102: u16 = N_OFFSET + 26;
pub const GPIO_103: u16 = N_OFFSET + 27;
pub const GPIO_104: u16 = N_OFFSET + 28;
pub const GPIO_105: u16 = N_OFFSET + 29;
pub const GPIO_106: u16 = N_OFFSET + 30;
pub const GPIO_107: u16 = N_OFFSET + 31;
pub const GPIO_108: u16 = N_OFFSET + 32;
pub const GPIO_109: u16 = N_OFFSET + 33;
pub const GPIO_110: u16 = N_OFFSET + 34;
pub const GPIO_111: u16 = N_OFFSET + 35;
pub const GPIO_112: u16 = N_OFFSET + 36;
pub const GPIO_113: u16 = N_OFFSET + 37;
pub const GPIO_114: u16 = N_OFFSET + 38;
pub const GPIO_115: u16 = N_OFFSET + 39;
pub const GPIO_116: u16 = N_OFFSET + 40;
pub const GPIO_117: u16 = N_OFFSET + 41;
pub const GPIO_118: u16 = N_OFFSET + 42;
pub const GPIO_119: u16 = N_OFFSET + 43;
pub const GPIO_120: u16 = N_OFFSET + 44;
pub const GPIO_121: u16 = N_OFFSET + 45;
pub const GPIO_122: u16 = N_OFFSET + 46;
pub const GPIO_123: u16 = N_OFFSET + 47;
pub const GPIO_124: u16 = N_OFFSET + 48;
pub const GPIO_125: u16 = N_OFFSET + 49;
pub const GPIO_126: u16 = N_OFFSET + 50;
pub const GPIO_127: u16 = N_OFFSET + 51;
pub const GPIO_128: u16 = N_OFFSET + 52;
pub const GPIO_129: u16 = N_OFFSET + 53;
pub const GPIO_130: u16 = N_OFFSET + 54;
pub const GPIO_131: u16 = N_OFFSET + 55;
pub const GPIO_132: u16 = N_OFFSET + 56;
pub const GPIO_133: u16 = N_OFFSET + 57;
pub const GPIO_134: u16 = N_OFFSET + 58;
pub const GPIO_135: u16 = N_OFFSET + 59;
pub const GPIO_136: u16 = N_OFFSET + 60;
pub const GPIO_137: u16 = N_OFFSET + 61;
pub const GPIO_138: u16 = N_OFFSET + 62;
pub const GPIO_139: u16 = N_OFFSET + 63;
pub const GPIO_140: u16 = N_OFFSET + 64;
pub const GPIO_141: u16 = N_OFFSET + 65;
pub const GPIO_142: u16 = N_OFFSET + 66;
pub const GPIO_143: u16 = N_OFFSET + 67;
pub const GPIO_144: u16 = N_OFFSET + 68;
pub const GPIO_145: u16 = N_OFFSET + 69;
pub const GPIO_146: u16 = N_OFFSET + 70;
pub const GPIO_147: u16 = N_OFFSET + 71;
pub const GPIO_148: u16 = N_OFFSET + 72;
pub const GPIO_149: u16 = N_OFFSET + 73;
pub const GPIO_150: u16 = N_OFFSET + 74;
pub const GPIO_151: u16 = N_OFFSET + 75;
pub const GPIO_152: u16 = N_OFFSET + 76;
pub const GPIO_153: u16 = N_OFFSET + 77;
pub const GPIO_154: u16 = N_OFFSET + 78;
pub const GPIO_155: u16 = N_OFFSET + 79;
pub const TOTAL_N_PADS: u16 = 80;

/* Audio Community Pads */
pub const AUDIO_OFFSET: u16 = N_OFFSET + 80;
pub const GPIO_156: u16 = AUDIO_OFFSET + 0;
pub const GPIO_157: u16 = AUDIO_OFFSET + 1;
pub const GPIO_158: u16 = AUDIO_OFFSET + 2;
pub const GPIO_159: u16 = AUDIO_OFFSET + 3;
pub const GPIO_160: u16 = AUDIO_OFFSET + 4;
pub const GPIO_161: u16 = AUDIO_OFFSET + 5;
pub const GPIO_162: u16 = AUDIO_OFFSET + 6;
pub const GPIO_163: u16 = AUDIO_OFFSET + 7;
pub const GPIO_164: u16 = AUDIO_OFFSET + 8;
pub const GPIO_165: u16 = AUDIO_OFFSET + 9;
pub const GPIO_166: u16 = AUDIO_OFFSET + 10;
pub const GPIO_167: u16 = AUDIO_OFFSET + 11;
pub const GPIO_168: u16 = AUDIO_OFFSET + 12;
pub const GPIO_169: u16 = AUDIO_OFFSET + 13;
pub const GPIO_170: u16 = AUDIO_OFFSET + 14;
pub const GPIO_171: u16 = AUDIO_OFFSET + 15;
pub const GPIO_172: u16 = AUDIO_OFFSET + 16;
pub const GPIO_173: u16 = AUDIO_OFFSET + 17;
pub const GPIO_174: u16 = AUDIO_OFFSET + 18;
pub const GPIO_175: u16 = AUDIO_OFFSET + 19;
pub const TOTAL_AUDIO_PADS: u16 = 20;

/* SCC community pads */
/* For SMBus, SD-Card, Clock, CNV/SDIO, eMMC */

pub const SCC_OFFSET: u16 = AUDIO_OFFSET + 20;
pub const GPIO_176: u16 = SCC_OFFSET + 0;
pub const GPIO_177: u16 = SCC_OFFSET + 1;
pub const GPIO_178: u16 = SCC_OFFSET + 2;
pub const GPIO_187: u16 = SCC_OFFSET + 3;
pub const GPIO_179: u16 = SCC_OFFSET + 4;
pub const GPIO_180: u16 = SCC_OFFSET + 5;
pub const GPIO_181: u16 = SCC_OFFSET + 6;
pub const GPIO_182: u16 = SCC_OFFSET + 7;
pub const GPIO_183: u16 = SCC_OFFSET + 8;
pub const GPIO_184: u16 = SCC_OFFSET + 9;
pub const GPIO_185: u16 = SCC_OFFSET + 10;
pub const GPIO_186: u16 = SCC_OFFSET + 11;
pub const GPIO_188: u16 = SCC_OFFSET + 12;
pub const GPIO_210: u16 = SCC_OFFSET + 13;
pub const GPIO_189: u16 = SCC_OFFSET + 14;
pub const GPIO_190: u16 = SCC_OFFSET + 15;
pub const GPIO_191: u16 = SCC_OFFSET + 16;
pub const GPIO_192: u16 = SCC_OFFSET + 17;
pub const GPIO_193: u16 = SCC_OFFSET + 18;
pub const GPIO_194: u16 = SCC_OFFSET + 19;
pub const GPIO_195: u16 = SCC_OFFSET + 20;
pub const GPIO_196: u16 = SCC_OFFSET + 21;
pub const GPIO_197: u16 = SCC_OFFSET + 22;
pub const GPIO_198: u16 = SCC_OFFSET + 23;
pub const GPIO_199: u16 = SCC_OFFSET + 24;
pub const GPIO_200: u16 = SCC_OFFSET + 25;
pub const GPIO_201: u16 = SCC_OFFSET + 26;
pub const GPIO_202: u16 = SCC_OFFSET + 27;
pub const GPIO_203: u16 = SCC_OFFSET + 28;
pub const GPIO_204: u16 = SCC_OFFSET + 29;
pub const GPIO_205: u16 = SCC_OFFSET + 30;
pub const GPIO_206: u16 = SCC_OFFSET + 31;
pub const GPIO_207: u16 = SCC_OFFSET + 32;
pub const GPIO_208: u16 = SCC_OFFSET + 33;
pub const GPIO_209: u16 = SCC_OFFSET + 34;
pub const TOTAL_SCC_PADS: u16 = 35;
pub const TOTAL_PADS: u16 = SCC_OFFSET + 35;

/// Miscellaneous Configuration register(MISCCFG).These are community specific
/// registers and are meant to house miscellaneous configuration fields per
/// community. There are 8 GPIO groups: GPP_0 -> GPP_8 (Group 3 is absent)
pub const GPIO_MISCCFG: u16 = 0x10; /* Miscellaneous Configuration offset */

pub const GPIO_GPE_NW_31_0: u16 = 0;
pub const GPIO_GPE_NW_63_32: u16 = 1;
pub const GPIO_GPE_NW_95_64: u16 = 2;
pub const GPIO_GPE_N_31_0: u16 = 4;
pub const GPIO_GPE_N_63_32: u16 = 5;
pub const GPIO_GPE_N_95_64: u16 = 6;
pub const GPIO_GPE_AUDIO_31_0: u16 = 7;
pub const GPIO_GPE_SCC_31_0: u16 = 8;
pub const GPIO_GPE_SCC_63_32: u16 = 9;

pub const GPIO_MAX_NUM_PER_GROUP: u16 = 32;

/// Host Software Pad Ownership Register.
/// The pins in the community are divided into 3 groups :
/// GPIO 0 ~ 31, GPIO 32 ~ 63, GPIO 64 ~ 95
pub const HOSTSW_OWN_REG_0: u16 = 0xB0;

pub const GPI_INT_STS_0: u16 = 0x100;
pub const GPI_INT_EN_0: u16 = 0x110;

pub const GPI_SMI_STS_0: u16 = 0x170;
pub const GPI_SMI_EN_0: u16 = 0x190;

/// PERST_0 not defined
pub const GPIO_PRT0_UDEF: u16 = 0xFF;

pub const NUM_NW_PADS: u16 = TOTAL_NW_PADS;
pub const NUM_N_PADS: u16 = TOTAL_N_PADS;
pub const NUM_AUDIO_PADS: u16 = TOTAL_AUDIO_PADS;
pub const NUM_SCC_PADS: u16 = TOTAL_SCC_PADS;

pub const NUM_NW_GPI_REGS: u16 =
    align_up(NUM_NW_PADS as u64, GPIO_MAX_NUM_PER_GROUP as u64) as u16 / GPIO_MAX_NUM_PER_GROUP;
pub const NUM_N_GPI_REGS: u16 =
    align_up(NUM_NW_PADS as u64, GPIO_MAX_NUM_PER_GROUP as u64) as u16 / GPIO_MAX_NUM_PER_GROUP;
pub const NUM_AUDIO_GPI_REGS: u16 =
    align_up(NUM_AUDIO_PADS as u64, GPIO_MAX_NUM_PER_GROUP as u64) as u16 / GPIO_MAX_NUM_PER_GROUP;
pub const NUM_SCC_GPI_REGS: u16 =
    align_up(NUM_SCC_PADS as u64, GPIO_MAX_NUM_PER_GROUP as u64) as u16 / GPIO_MAX_NUM_PER_GROUP;
pub const NUM_GPI_STATUS_REGS: u16 =
    NUM_N_GPI_REGS + NUM_NW_GPI_REGS + NUM_AUDIO_GPI_REGS + NUM_SCC_GPI_REGS;

/* Functions for translating a global pad offset to a local offset */

pub fn pad_nw(pad: u16) -> u16 {
    pad - NW_OFFSET
}

pub fn pad_n(pad: u16) -> u16 {
    pad - N_OFFSET
}

pub fn pad_audio(pad: u16) -> u16 {
    pad - AUDIO_OFFSET
}

pub fn pad_scc(pad: u16) -> u16 {
    pad - SCC_OFFSET
}

/* Linux names of the GPIO devices. */
pub const CROS_GPIO_DEVICE_NAME: &str = "INT3453";
pub const GPIO_COMM_NW_NAME: &str = "INT3453:00";
pub const GPIO_COMM_N_NAME: &str = "INT3453:01";
pub const GPIO_COMM_AUDIO_NAME: &str = "INT3453:02";
pub const GPIO_COMM_SCC_NAME: &str = "INT3453:03";

/* Following is used in gpio asl */
pub const GPIO_COMM_NAME: &str = "INT3453";
pub const GPIO_COMM_0_DESC: &str = "General Purpose Input/Output (GPIO) Controller - Northwest";
pub const GPIO_COMM_1_DESC: &str = "General Purpose Input/Output (GPIO) Controller - North";
pub const GPIO_COMM_2_DESC: &str = "General Purpose Input/Output (GPIO) Controller - Audio";
pub const GPIO_COMM_3_DESC: &str = "General Purpose Input/Output (GPIO) Controller - SCC";

pub const GPIO_COMM0_PID: u16 = PID_GPIO_NW;
pub const GPIO_COMM1_PID: u16 = PID_GPIO_N;
pub const GPIO_COMM2_PID: u16 = PID_GPIO_AUDIO;
pub const GPIO_COMM3_PID: u16 = PID_GPIO_SCC;

pub const GPIO_8_IRQ: u16 = 0x32;
pub const GPIO_9_IRQ: u16 = 0x33;
pub const GPIO_10_IRQ: u16 = 0x34;
pub const GPIO_11_IRQ: u16 = 0x35;
pub const GPIO_12_IRQ: u16 = 0x36;
pub const GPIO_13_IRQ: u16 = 0x37;
pub const GPIO_14_IRQ: u16 = 0x38;
pub const GPIO_15_IRQ: u16 = 0x39;
pub const GPIO_16_IRQ: u16 = 0x3a;
pub const GPIO_17_IRQ: u16 = 0x3b;
pub const GPIO_18_IRQ: u16 = 0x3c;
pub const GPIO_19_IRQ: u16 = 0x3d;
pub const GPIO_20_IRQ: u16 = 0x3e;
pub const GPIO_21_IRQ: u16 = 0x3f;
pub const GPIO_22_IRQ: u16 = 0x40;
pub const GPIO_23_IRQ: u16 = 0x41;
pub const GPIO_24_IRQ: u16 = 0x42;
pub const GPIO_25_IRQ: u16 = 0x43;
pub const GPIO_26_IRQ: u16 = 0x44;
pub const GPIO_27_IRQ: u16 = 0x45;
pub const GPIO_28_IRQ: u16 = 0x46;
pub const GPIO_29_IRQ: u16 = 0x47;
pub const GPIO_30_IRQ: u16 = 0x48;
pub const GPIO_31_IRQ: u16 = 0x49;
pub const GPIO_32_IRQ: u16 = 0x4a;
pub const GPIO_33_IRQ: u16 = 0x4b;
pub const GPIO_34_IRQ: u16 = 0x4c;
pub const GPIO_35_IRQ: u16 = 0x4d;
pub const GPIO_36_IRQ: u16 = 0x4e;
pub const GPIO_37_IRQ: u16 = 0x4f;
pub const GPIO_38_IRQ: u16 = 0x50;
pub const GPIO_39_IRQ: u16 = 0x51;
pub const GPIO_40_IRQ: u16 = 0x52;
pub const GPIO_41_IRQ: u16 = 0x53;
pub const GPIO_42_IRQ: u16 = 0x54;
pub const GPIO_43_IRQ: u16 = 0x55;
pub const GPIO_44_IRQ: u16 = 0x56;
pub const GPIO_45_IRQ: u16 = 0x57;
pub const GPIO_46_IRQ: u16 = 0x58;
pub const GPIO_47_IRQ: u16 = 0x59;
pub const GPIO_48_IRQ: u16 = 0x5a;
pub const GPIO_49_IRQ: u16 = 0x5b;
pub const GPIO_50_IRQ: u16 = 0x5c;
pub const GPIO_51_IRQ: u16 = 0x5d;
pub const GPIO_52_IRQ: u16 = 0x5e;
pub const GPIO_53_IRQ: u16 = 0x5f;
pub const GPIO_54_IRQ: u16 = 0x60;
pub const GPIO_55_IRQ: u16 = 0x61;
pub const GPIO_56_IRQ: u16 = 0x62;
pub const GPIO_57_IRQ: u16 = 0x63;
pub const GPIO_58_IRQ: u16 = 0x64;
pub const GPIO_59_IRQ: u16 = 0x65;
pub const GPIO_60_IRQ: u16 = 0x66;
pub const GPIO_61_IRQ: u16 = 0x67;
pub const GPIO_62_IRQ: u16 = 0x68;
pub const GPIO_63_IRQ: u16 = 0x69;
pub const GPIO_64_IRQ: u16 = 0x6a;
pub const GPIO_65_IRQ: u16 = 0x6b;
pub const GPIO_66_IRQ: u16 = 0x6c;
pub const GPIO_67_IRQ: u16 = 0x6d;
pub const GPIO_68_IRQ: u16 = 0x6e;
pub const GPIO_69_IRQ: u16 = 0x6f;
pub const GPIO_70_IRQ: u16 = 0x70;
pub const GPIO_71_IRQ: u16 = 0x71;
pub const GPIO_72_IRQ: u16 = 0x72;
pub const GPIO_73_IRQ: u16 = 0x73;
pub const GPIO_211_IRQ: u16 = 0x74;
pub const GPIO_212_IRQ: u16 = 0x75;
pub const GPIO_213_IRQ: u16 = 0x76;
pub const GPIO_214_IRQ: u16 = 0x77;
pub const GPIO_79_IRQ: u16 = 0x32;
pub const GPIO_80_IRQ: u16 = 0x33;
pub const GPIO_81_IRQ: u16 = 0x34;
pub const GPIO_82_IRQ: u16 = 0x35;
pub const GPIO_83_IRQ: u16 = 0x36;
pub const GPIO_84_IRQ: u16 = 0x37;
pub const GPIO_85_IRQ: u16 = 0x38;
pub const GPIO_86_IRQ: u16 = 0x39;
pub const GPIO_87_IRQ: u16 = 0x3a;
pub const GPIO_88_IRQ: u16 = 0x3b;
pub const GPIO_89_IRQ: u16 = 0x3c;
pub const GPIO_90_IRQ: u16 = 0x3d;
pub const GPIO_91_IRQ: u16 = 0x3e;
pub const GPIO_92_IRQ: u16 = 0x3f;
pub const GPIO_93_IRQ: u16 = 0x40;
pub const GPIO_94_IRQ: u16 = 0x41;
pub const GPIO_95_IRQ: u16 = 0x42;
pub const GPIO_96_IRQ: u16 = 0x43;
pub const GPIO_105_IRQ: u16 = 0x44;
pub const GPIO_110_IRQ: u16 = 0x45;
pub const GPIO_111_IRQ: u16 = 0x46;
pub const GPIO_112_IRQ: u16 = 0x47;
pub const GPIO_113_IRQ: u16 = 0x48;
pub const GPIO_114_IRQ: u16 = 0x49;
pub const GPIO_115_IRQ: u16 = 0x4a;
pub const GPIO_116_IRQ: u16 = 0x4b;
pub const GPIO_117_IRQ: u16 = 0x4c;
pub const GPIO_118_IRQ: u16 = 0x4d;
pub const GPIO_119_IRQ: u16 = 0x4e;
pub const GPIO_120_IRQ: u16 = 0x4f;
pub const GPIO_121_IRQ: u16 = 0x50;
pub const GPIO_122_IRQ: u16 = 0x51;
pub const GPIO_123_IRQ: u16 = 0x52;
pub const GPIO_124_IRQ: u16 = 0x53;
pub const GPIO_125_IRQ: u16 = 0x54;
pub const GPIO_126_IRQ: u16 = 0x55;
pub const GPIO_127_IRQ: u16 = 0x56;
pub const GPIO_128_IRQ: u16 = 0x57;
pub const GPIO_129_IRQ: u16 = 0x58;
pub const GPIO_130_IRQ: u16 = 0x59;
pub const GPIO_131_IRQ: u16 = 0x5a;
pub const GPIO_132_IRQ: u16 = 0x5b;
pub const GPIO_133_IRQ: u16 = 0x5c;
pub const GPIO_134_IRQ: u16 = 0x5d;
pub const GPIO_135_IRQ: u16 = 0x5e;
pub const GPIO_136_IRQ: u16 = 0x5f;
pub const GPIO_137_IRQ: u16 = 0x60;
pub const GPIO_138_IRQ: u16 = 0x61;
pub const GPIO_139_IRQ: u16 = 0x62;
pub const GPIO_140_IRQ: u16 = 0x63;
pub const GPIO_141_IRQ: u16 = 0x64;
pub const GPIO_142_IRQ: u16 = 0x65;
pub const GPIO_143_IRQ: u16 = 0x66;
pub const GPIO_144_IRQ: u16 = 0x67;
pub const GPIO_145_IRQ: u16 = 0x68;
pub const GPIO_146_IRQ: u16 = 0x69;
pub const GPIO_147_IRQ: u16 = 0x6a;
pub const GPIO_148_IRQ: u16 = 0x6b;
pub const GPIO_149_IRQ: u16 = 0x6c;
pub const GPIO_150_IRQ: u16 = 0x6d;
pub const GPIO_151_IRQ: u16 = 0x6e;
pub const GPIO_152_IRQ: u16 = 0x6f;
pub const GPIO_153_IRQ: u16 = 0x70;
pub const GPIO_154_IRQ: u16 = 0x71;
pub const GPIO_155_IRQ: u16 = 0x72;

pub const PAD_CFG_BASE: u16 = 0x600;

pub const GPIO_NUM_PAD_CFG_REGS: usize = 4;

pub const RST_MAP: [ResetMapping; 3] = [
    ResetMapping {
        logical: PAD_CFG0_LOGICAL_RESET_PWROK,
        chipset: 0 << 30,
    },
    ResetMapping {
        logical: PAD_CFG0_LOGICAL_RESET_DEEP,
        chipset: 1 << 30,
    },
    ResetMapping {
        logical: PAD_CFG0_LOGICAL_RESET_PLTRST,
        chipset: 2 << 30,
    },
];

pub const GLK_COMMUNITY_AUDIO_GROUPS: [PadGroup; 1] = [PadGroup::intel_gpp(
    AUDIO_OFFSET as i32,
    AUDIO_OFFSET as u32,
    GPIO_175 as u32,
)];

pub const GLK_COMMUNITY_NW_GROUPS: [PadGroup; 3] = [
    // NORTHWEST 0
    PadGroup::intel_gpp(NW_OFFSET as i32, NW_OFFSET as u32, GPIO_31 as u32),
    // NORTHWEST 1
    PadGroup::intel_gpp(NW_OFFSET as i32, GPIO_32 as u32, GPIO_63 as u32),
    // NORTHWEST 2
    PadGroup::intel_gpp(NW_OFFSET as i32, GPIO_64 as u32, GPIO_214 as u32),
];

pub const GLK_COMMUNITY_SCC_GROUPS: [PadGroup; 2] = [
    // SCC 0
    PadGroup::intel_gpp(SCC_OFFSET as i32, SCC_OFFSET as u32, GPIO_206 as u32),
    // SCC 1
    PadGroup::intel_gpp(SCC_OFFSET as i32, GPIO_207 as u32, GPIO_209 as u32),
];

pub const GLK_COMMUNITY_N_GROUPS: [PadGroup; 3] = [
    // NORTH 0
    PadGroup::intel_gpp(N_OFFSET as i32, N_OFFSET as u32, GPIO_107 as u32),
    // NORTH 1
    PadGroup::intel_gpp(N_OFFSET as i32, GPIO_108 as u32, GPIO_139 as u32),
    // NORTH 2
    PadGroup::intel_gpp(N_OFFSET as i32, GPIO_140 as u32, GPIO_155 as u32),
];

pub const GLK_GPIO_COMMUNITIES: [PadCommunity; GPIO_NUM_PAD_CFG_REGS] = [
    PadCommunity {
        port: PID_GPIO_NW as u8,
        first_pad: NW_OFFSET as u32,
        last_pad: GPIO_214 as u32,
        num_gpi_regs: NUM_NW_GPI_REGS as usize,
        gpi_status_offset: 0,
        pad_cfg_base: PAD_CFG_BASE,
        host_own_reg_0: HOSTSW_OWN_REG_0,
        gpi_int_sts_reg_0: GPI_INT_STS_0,
        gpi_int_en_reg_0: GPI_INT_EN_0,
        gpi_smi_sts_reg_0: GPI_SMI_STS_0,
        gpi_smi_en_reg_0: GPI_SMI_EN_0,
        max_pads_per_group: GPIO_MAX_NUM_PER_GROUP as usize,
        name: "GPIO_NORTHWEST",
        acpi_path: "\\_SB.GPO0",
        reset_map: &RST_MAP,
        groups: &GLK_COMMUNITY_NW_GROUPS,
        vw_entries: Vec::new(),
        vw_base: 0,
        cpu_port: 0,
        gpi_gpe_en_reg_0: 0,
        gpi_gpe_sts_reg_0: 0,
        gpi_nmi_en_reg_0: 0,
        gpi_nmi_sts_reg_0: 0,
        pad_cfg_lock_offset: 0,
    },
    PadCommunity {
        port: PID_GPIO_N as u8,
        first_pad: N_OFFSET as u32,
        last_pad: GPIO_155 as u32,
        num_gpi_regs: NUM_N_GPI_REGS as usize,
        gpi_status_offset: NUM_NW_GPI_REGS as u8,
        pad_cfg_base: PAD_CFG_BASE,
        host_own_reg_0: HOSTSW_OWN_REG_0,
        gpi_int_sts_reg_0: GPI_INT_STS_0,
        gpi_int_en_reg_0: GPI_INT_EN_0,
        gpi_smi_sts_reg_0: GPI_SMI_STS_0,
        gpi_smi_en_reg_0: GPI_SMI_EN_0,
        max_pads_per_group: GPIO_MAX_NUM_PER_GROUP as usize,
        name: "GPIO_NORTH",
        acpi_path: "\\_SB.GPO1",
        reset_map: &RST_MAP,
        groups: &GLK_COMMUNITY_N_GROUPS,
        vw_entries: Vec::new(),
        vw_base: 0,
        cpu_port: 0,
        gpi_gpe_en_reg_0: 0,
        gpi_gpe_sts_reg_0: 0,
        gpi_nmi_en_reg_0: 0,
        gpi_nmi_sts_reg_0: 0,
        pad_cfg_lock_offset: 0,
    },
    PadCommunity {
        port: PID_GPIO_AUDIO as u8,
        first_pad: AUDIO_OFFSET as u32,
        last_pad: GPIO_175 as u32,
        num_gpi_regs: NUM_AUDIO_GPI_REGS as usize,
        gpi_status_offset: (NUM_NW_GPI_REGS + NUM_N_GPI_REGS) as u8,
        pad_cfg_base: PAD_CFG_BASE,
        host_own_reg_0: HOSTSW_OWN_REG_0,
        gpi_int_sts_reg_0: GPI_INT_STS_0,
        gpi_int_en_reg_0: GPI_INT_EN_0,
        gpi_smi_sts_reg_0: GPI_SMI_STS_0,
        gpi_smi_en_reg_0: GPI_SMI_EN_0,
        max_pads_per_group: GPIO_MAX_NUM_PER_GROUP as usize,
        name: "GPIO_AUDIO",
        acpi_path: "\\_SB.GPO2",
        reset_map: &RST_MAP,
        groups: &GLK_COMMUNITY_AUDIO_GROUPS,
        vw_entries: Vec::new(),
        vw_base: 0,
        cpu_port: 0,
        gpi_gpe_en_reg_0: 0,
        gpi_gpe_sts_reg_0: 0,
        gpi_nmi_en_reg_0: 0,
        gpi_nmi_sts_reg_0: 0,
        pad_cfg_lock_offset: 0,
    },
    PadCommunity {
        port: PID_GPIO_SCC as u8,
        first_pad: SCC_OFFSET as u32,
        last_pad: GPIO_209 as u32,
        num_gpi_regs: NUM_SCC_GPI_REGS as usize,
        gpi_status_offset: (NUM_NW_GPI_REGS + NUM_N_GPI_REGS + NUM_AUDIO_GPI_REGS) as u8,
        pad_cfg_base: PAD_CFG_BASE,
        host_own_reg_0: HOSTSW_OWN_REG_0,
        gpi_int_sts_reg_0: GPI_INT_STS_0,
        gpi_int_en_reg_0: GPI_INT_EN_0,
        gpi_smi_sts_reg_0: GPI_SMI_STS_0,
        gpi_smi_en_reg_0: GPI_SMI_EN_0,
        max_pads_per_group: GPIO_MAX_NUM_PER_GROUP as usize,
        name: "GPIO_SCC",
        acpi_path: "\\_SB.GPO3",
        reset_map: &RST_MAP,
        groups: &GLK_COMMUNITY_SCC_GROUPS,
        vw_entries: Vec::new(),
        vw_base: 0,
        cpu_port: 0,
        gpi_gpe_en_reg_0: 0,
        gpi_gpe_sts_reg_0: 0,
        gpi_nmi_en_reg_0: 0,
        gpi_nmi_sts_reg_0: 0,
        pad_cfg_lock_offset: 0,
    },
];

pub fn soc_gpio_get_community(
) -> [PadCommunity<'static, 'static, 'static, 'static>; GPIO_NUM_PAD_CFG_REGS] {
    GLK_GPIO_COMMUNITIES
}
