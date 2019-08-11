extern {
    fn inb(addr : u64) -> u8;
    fn outb(addr : u64, val : u8);
}

// LPC io space base addres
const LPC_IO_BASE : u64 = 0x60300d0010000;

pub fn lpc_inb(reg : u16) -> u8 {
    unsafe {
        inb(LPC_IO_BASE + reg as u64)
    }
}

pub fn lpc_outb(reg : u16, val : u8) {
    unsafe {
        outb(LPC_IO_BASE + reg as u64, val);
    }
}

#[allow(dead_code)]
pub fn sio_inb(reg : u8) -> u8{
    /* superio is accessed indirectly via an PIO interface at 0x2e, 0x2f */
    lpc_outb(0x2e, reg);
    lpc_inb(0x2f)
}

pub fn sio_outb(reg : u8, val : u8) {
    lpc_outb(0x2e, reg);
    lpc_outb(0x2f, val);
}

pub fn init_sio() {
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
