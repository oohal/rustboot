#![feature(lang_items)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate fdt;

use core::intrinsics;
use core::panic::PanicInfo;
use core::cell::UnsafeCell;

use core::fmt;
use core::fmt::Write;
use core::str;

use fdt::FDT;

mod my_alloc;
mod lpc;

extern {
    fn ohshit();
}

struct Console;

fn prlog(s : &str) {
    for c in s.bytes() {
            lpc::lpc_outb(0x3f8, c);
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s : &str) -> Result<(), fmt::Error> {
        prlog(s);

        Ok(())
    }
}

//#[no_mangle]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut cons = Console {};

        prlog("picnicked!\r\n");


        if let Some(msg) = _info.location(){
            write!(&mut cons, "Panic at: {}:{}\r\n", msg.file(), msg.line());
        } else {
            prlog("recursive panic?");
        }

        prlog("fin!\r\n");

        unsafe {
            ohshit();
            intrinsics::abort()
        }
}

#[no_mangle]
pub fn memcpy(dst : *mut u8, src : *const u8, size : isize)
{
    unsafe {
        for i in 0..size {
            *dst.offset(i) = *src.offset(i);
        }
    }
}

#[no_mangle]
pub fn memcmp(s1 : *mut u8, s2 : *const u8, size : isize) -> isize
{
    unsafe {
        for i in 0..size {
            if *s1.offset(i) != *s2.offset(i) {
                return (*s1.offset(i) - *s2.offset(i)) as isize
            }
        }
    }

    return 0
}

#[no_mangle]
pub fn _start(base_addr : u64, fdt_ptr : u64) -> ! {
    let mut cons = Console {};
    let fdt;

    lpc::init_sio();

    write!(&mut cons, "FDT at: {:016x}\r\n", fdt_ptr);
    write!(&mut cons, "Base at: {:016x}\r\n", base_addr);

    unsafe {
        fdt = FDT::from_raw(fdt_ptr as *const u8).unwrap();
    }

    for n in fdt.nodes() {
        let sn = n.size_cells();
        let an = n.address_cells();

        write!(&mut cons, "node: {} [{},{}]\r\n", n.name(), an, sn);

        for prop in n.properties() {
            if prop.name() == "reg" && sn > 0 && an > 0 {
                let mut size = 0_u64;
                let mut addr = 0_u64;

                for i in 0..sn {
                    size <<= 32;
                    size |= prop.cell(i as usize).unwrap_or(0u32) as u64;
                }

                for i in 0..an {
                    addr <<= 32;
                    addr |= prop.cell(i as usize).unwrap_or(0_u32) as u64;
                }

                write!(&mut cons, "\treg: [{},{}] = [{:016x}, {:016x}]\r\n", sn, an, addr, size);
            }
            write!(&mut cons, "\tprop: {}\r\n", prop.name());
        }
    }


    loop{};
}

// Declaration of the global memory allocator
// NOTE the user must ensure that the memory region `[0x2000_0100, 0x2000_0200]`
// is not used by other parts of the program
#[global_allocator]
static HEAP: my_alloc::BumpPointerAlloc = my_alloc::BumpPointerAlloc {
    head: UnsafeCell::new(0x2000_0000),
    end: 0x2001_0000,
};

#[lang = "eh_personality"] extern fn eh_personality() {}
//#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
