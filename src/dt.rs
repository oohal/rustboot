use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use fdt::FDT;

use prlog;

struct DtProp {
    name : String,
    value : Box<Vec<u8>>,
}

struct DtNode {
    name :  String,
    children : Box<Vec<DtNode>>,
    properties : Box<Vec<DtProp>>,
//    parent : Option<&'a DtNode<'a>>,
    phandle : u32,
}



fn expand_node(fdt_node : fdt::node::Node) -> DtNode {
    let mut node = DtNode {
        name: String::from(fdt_node.name()),
        children: Box::new(Vec::new()),
        properties: Box::new(Vec::new()),
        phandle: 0
    };

//    let raw = node.name.as_ptr() as *const u8;
/*
    for p in fdt_node.properties() {
        let mut new = DtProp {
            name: String::from(p.name()),
            value: Box::new(p.raw().to_vec()),
        };

        node.properties.push(new);
    }
*/
    for c in fdt_node.children() {
        node.children.push(expand_node(c));
    }

    return node;
}

fn print_dt(n : &DtNode) {
    prlog!("n: {}", n.name);

/*
    for p in n.properties.iter() {
        prlog!("p: {}={:?}", p.name, p.value);
    }
*/
    for c in n.children.iter() {
        print_dt(c);
    }
}

pub fn expand_dt(fdt_ptr : u64) {
    let fdt;

    unsafe {
        fdt = FDT::from_raw(fdt_ptr as *const u8).unwrap();
    }

    let fdt_root = fdt.nodes().nth(0).unwrap();
    let root = expand_node(fdt_root);

    print_dt(&root);

    //return;

    for n in fdt.nodes() {
        let sn = n.size_cells();
        let an = n.address_cells();

        prlog!("node: {} [{},{}], depth: {}", n.name(), an, sn, n.depth());

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
