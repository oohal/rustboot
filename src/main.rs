#![feature(lang_items)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate fdt;

use core::intrinsics;
use core::panic::PanicInfo;
use core::cell::UnsafeCell;

mod my_alloc;
mod console;
mod lpc;
mod dt;

extern {
    fn ohshit();
}

//#[no_mangle]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    prlog!("picnicked!");

    if let Some(msg) = _info.location() {
        prlog!("Panic at: {}:{}", msg.file(), msg.line());
    } else {
        prlog!("recursive panic?");
    }

    prlog!("fin!");

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
pub fn memset(s : *mut u8, val : u8, size : isize) -> *mut u8
{
    unsafe {
        for i in 0..size {
            *s.offset(i) = val
        }
    }

    return s
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
pub fn bcmp(s1 : *mut u8, s2 : *const u8, size : isize) -> isize
{
    return memcmp(s1, s2, size);
}

#[no_mangle]
pub fn _start(base_addr : u64, fdt_ptr : u64) -> ! {
    lpc::init_sio();

    prlog!("FDT at: {:016x}", fdt_ptr);
    prlog!("Base at: {:016x}", base_addr);

    dt::expand_dt(fdt_ptr);

    panic!("shit!!!");
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
