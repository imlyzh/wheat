use std::{
    collections::HashSet,
    io::{stdin, stdout, Write},
};

use wheat::{
    memory_manage::{page_size::{_4K, _64K}, SemiSpaceMemory},
    object_operator::read::read_,
    vm_state::VMState,
};

fn main() {
    println!("wheat programming language. Copyright 2024 lyzh");
    unsafe {
        let mut vms = VMState {
            heap: SemiSpaceMemory::init(_4K, false),
            current: None,
            symbol_cache: HashSet::new(),
        };
        loop {
            print!("> ");
            stdout().flush().unwrap();
            let mut input_buf = "".to_string();
            stdin().read_line(&mut input_buf).unwrap();
            let slot = read_(&mut vms, &input_buf);
            println!("slot pointer: {:?}", slot);
            println!("alloc counter: {:?}", vms.heap.alloc_count);
        }
    }
}
