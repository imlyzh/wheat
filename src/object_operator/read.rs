use crate::{
    make_object::*,
    object_model::{SingleByteString, Slot},
    vm_state::VMState,
};
use sexpr_ir::{
    gast::{constant::Constant, GAst},
    syntax::sexpr::one_unit_parse,
};

pub unsafe fn read(vms: &mut VMState, i: &str) -> Slot {
    let r = one_unit_parse(i, "<read>").unwrap();
    gast2slot(vms, &r)
}

pub unsafe fn gast2slot(vms: &mut VMState, r: &GAst) -> Slot {
    if let Some(r) = r.get_const() {
        match r {
            Constant::Nil => return make_null(),
            Constant::Bool(b) => return make_bool(b),
            Constant::Char(c) => return make_char(vms, c as u8), // char
            Constant::Int(i) => return make_integer(vms, i),
            Constant::Uint(_) => unimplemented!(),
            Constant::Str(arc) => {
                let len = arc.bytes().len();
                let r = make_string(vms, len);
                let dst = &(*(r as *mut SingleByteString)).instance[0];
                let dst = dst as *const u8 as *mut u8;
                std::ptr::copy_nonoverlapping(arc.as_ptr(), dst, len);
                return r;
            }
            Constant::Sym(arc) => return make_symbol(vms, arc.0.as_str()),
            Constant::Float(_) => unimplemented!(),
        }
    }
    if let Some(r) = r.get_list() {
        let mut slot = make_null();
        for i in 0..(r.0.len()) {
            let i = r.0.get(r.0.len() - 1 - i).unwrap_unchecked();
            let car = gast2slot(vms, i);
            slot = make_pair(vms, car, slot);
        }
        return slot;
    }
    unreachable!()
}
