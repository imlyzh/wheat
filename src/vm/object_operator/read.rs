use crate::vm::{
    make_object::*,
    object_model::{SingleByteString, Slot},
    vm_state::VMState,
};
use sexpr_ir::{
    gast::{constant::Constant, GAst},
    syntax::sexpr::one_unit_parse,
};

pub unsafe fn read_(vms: &mut VMState, i: &str) -> Slot {
    let r = one_unit_parse(i, "<read>").unwrap();
    println!("{:?}", r);
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
            Constant::Str(s) => {
                return make_string(vms, &s);
            }
            Constant::Sym(arc) => return make_symbol(vms, arc.0.as_str()),
            Constant::Float(_) => unimplemented!(),
        }
    }
    if let Some(r) = r.get_list() {
        let mut slot = make_null();
        for i in 0..(r.0.len()) {
            let i: &GAst = r.0.get(r.0.len() - 1 - i).unwrap();
            let car = gast2slot(vms, i);
            slot = make_pair(vms, car, slot);
        }
        return slot;
    }
    unreachable!()
}
