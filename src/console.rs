use core::fmt;
use core::fmt::Write;
use core::str;

use lpc;

pub struct Console;

pub fn cons_puts(s : &str) {
    for c in s.bytes() {
            lpc::lpc_outb(0x3f8, c);
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s : &str) -> Result<(), fmt::Error> {
        cons_puts(s);

        Ok(())
    }
}

pub fn cons_fmt(args : fmt::Arguments) {
	let mut cons = Console {};

	write!(&mut cons, "{}", args).unwrap();
}

// ripped from:
// https://docs.rs/cortex-m-semihosting/0.3.4/src/cortex_m_semihosting/macros.rs.html#35-42
#[macro_export]
macro_rules! prlog {
    () => {
        $crate::console::cons_puts("\n")
    };
    ($s:expr) => {
        $crate::console::cons_puts(concat!($s, "\n"))
    };
    ($s:expr, $($tt:tt)*) => {
        $crate::console::cons_fmt(format_args!(concat!($s, "\n"), $($tt)*))
    };
}
