use crate::{
    make_object::{make_bool, make_char, make_null, make_symbol},
    object_model::Slot, vm_state::VMState,
};
use sexpr_ir::{gast::constant::Constant, syntax::sexpr::one_unit_parse};

pub unsafe fn read(vms: &mut VMState, i: &str) -> Slot {
    let r = one_unit_parse(i, "<read>").unwrap();
    if let Some(r) = r.get_const() {
        match r {
            Constant::Nil => return make_null(),
            Constant::Bool(b) => return make_bool(b),
            Constant::Char(c) => return make_char(vms, c as u8), // char
            Constant::Int(_) => todo!(),
            Constant::Uint(_) => todo!(),
            Constant::Str(arc) => todo!(),
            Constant::Sym(arc) => return make_symbol(vms, arc.0.as_str()),
            Constant::Float(_) => unimplemented!(),
        }
    }
    todo!()
}
