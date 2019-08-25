use fdt::FDT;

use prlog;

pub fn expand_dt(fdt_ptr : u64) {
    let fdt;

    unsafe {
        fdt = FDT::from_raw(fdt_ptr as *const u8).unwrap();
    }

    for n in fdt.nodes() {
        let sn = n.size_cells();
        let an = n.address_cells();

        prlog!("node: {} [{},{}]", n.name(), an, sn);

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

                prlog!("\treg: [{},{}] = [{:016x}, {:016x}]", sn, an, addr, size);
            }
            prlog!("\tprop: {}", prop.name());
        }
    }

    prlog!("dt parsed!");
}
