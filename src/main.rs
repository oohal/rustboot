//extern crate fdt;

#![feature(lang_items)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics;
use core::panic::PanicInfo;

use core::fmt;
use core::str;

//#[no_mangle]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
        unsafe { intrinsics::abort() }
}

extern {
    fn mambo_write(buf : *const u8, length : usize);
    fn ohshit();
    fn inb(addr : u64) -> u8;
    fn outb(addr : u64, val : u8);
}

// LPC io space base addres
const lpc_io_base : u64 = 0x60300d0010000;

fn lpc_inb(reg : u16) -> u8 {
    unsafe {
        inb(lpc_io_base + reg as u64)
    }
}

fn lpc_outb(reg : u16, val : u8) {
    unsafe {
        outb(lpc_io_base + reg as u64, val);
    }
}

fn sio_inb(reg : u8) -> u8{

    /* superio is accessed indirectly via an PIO interface at 0x2e, 0x2f */
    lpc_outb(0x2e, reg);
    lpc_inb(0x2f)
}

fn sio_outb(reg : u8, val : u8) {
    lpc_outb(0x2e, reg);
    lpc_outb(0x2f, val);
}

fn init_sio() {

    lpc_outb(0x2e, 0xa5); // unlock superio
    lpc_outb(0x2e, 0xa5);

    sio_outb(0x7, 2); // select uart1

    sio_outb(0x30, 0); // disable uart

    sio_outb(0x60, 0x03); // put uart at 0x3f8
    sio_outb(0x61, 0xf8);

    sio_outb(0x30, 0x1); // enable uart

    // lock
    lpc_outb(0x2e, 0xaa);
}

fn init_uart() {
}

struct Console {} // i forget what this is for?

impl fmt::Write for Console {
    fn write_str(&mut self, s : &str) -> Result<(), fmt::Error> {
        let uart_fifo : *mut u8 = 0x60300d00103f8 as *mut u8;

        for c in "asdf\n".bytes() {
                lpc_outb(0x3f8, c);
        }

        Ok(())
    }
}

#[no_mangle]
pub fn _start(fdt_ptr : u64) -> ! {
    let mut cons = Console {};

    init_sio();

    loop {
        fmt::write(&mut cons, format_args!("hello {}\r\n", "world")).unwrap();
//        fmt::write(&mut cons, format_args!("Hello {}!", "world")).unwrap();
//            .expect("Error occurred while trying to write in String");
     }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
//#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
