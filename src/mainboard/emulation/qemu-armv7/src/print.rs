pub mod write_to {
    use core::fmt;

    pub struct WriteTo {}

    impl WriteTo {
        pub fn new() -> Self {
            WriteTo {}
        }
    }

    impl fmt::Write for WriteTo {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            for i in s.bytes() {
                crate::putc(i);
            }

            Ok(())
        }
    }

    pub fn show(args: fmt::Arguments) -> Result<(), fmt::Error> {
        let mut w = WriteTo::new();
        fmt::write(&mut w, args)
    }
}

pub fn print(_s: &str) {
    write_to::show(format_args!("write some stuff {:?}: {}", "foo", 42)).unwrap();
}
