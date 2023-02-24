pub struct MemCfg {
    pub reg_nr: u32,
    pub mask: u32,
    pub value: u32,
}

impl MemCfg {
    fn new(reg_nr: u32, mask: u32, value: u32) -> Self {
        MemCfg {
            reg_nr,
            mask,
            value,
        }
    }
}

/// Generate array of MemCfg from simple C-style struct array
///
/// Example:
/// ```rs
/// const MEM_CFG_0: [MemCfg; 3] = mem_cfg_arr![
///     {0x0,   0x0, 0x00000001},
///     {0xf00, 0x0, 0x40001030},
///     {0xf00, 0x0, 0x40001030},
/// ];
/// ```
///
/// Results in:
///
/// ```rs
/// const MEM_CFG_FOO: [MemCfg; 3] = [
///     MemCfg {
///         offset: 0x0,
///         mask: 0x0,
///         value: 0x00000001,
///     },
///     MemCfg {
///         offset: 0xf00,
///         mask: 0x0,
///         value: 0x40001030,
///     },
///     MemCfg {
///         offset: 0xf00,
///         mask: 0x0,
///         value: 0x40001030,
///     },
/// ];
/// ```
#[macro_export]
macro_rules! mem_cfg_arr {
    ($({ $o: expr, $m: expr, $v: expr }),* $(,)?) => {
        [
            $(MemCfg { reg_nr: $o, mask: $m, value: $v }),*
        ]
    };
}

pub(crate) use mem_cfg_arr;

pub struct MemSet {
    pub reg_nr: u32,
    pub value: u32,
}

#[macro_export]
macro_rules! mem_set_arr {
    ($({ $r: expr, $v: expr }),* $(,)?) => {
        [
            $(MemSet { reg_nr: $r, value: $v }),*
        ]
    };
}

pub(crate) use mem_set_arr;
