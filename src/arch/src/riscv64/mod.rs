// For more information on what SBI is and what methods etc are defined, see
// https://github.com/riscv-non-isa/riscv-sbi-doc/blob/master/riscv-sbi.adoc
// For a test suite, have a look at https://github.com/rustsbi/sbi-testing
// A simple test can be found at https://github.com/orangecms/sbitest

pub mod sbi {
    pub mod csr;
    pub mod execute;
    pub mod feature;
    pub mod info;
    pub mod runtime;
}

mod util;
mod xuantie;
