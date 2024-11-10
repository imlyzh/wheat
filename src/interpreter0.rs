use crate::{
    make_object::make_bool,
    object_model::{assert_get_bool, get_tag, ObjectTag, Slot},
    object_operator::{pair::car, tageq::*},
};

pub unsafe fn is_atom(i: Slot) -> bool {
    !matches!((*i).tag, ObjectTag::Pair)
}

pub unsafe fn raw_or(i: &[Slot]) -> bool {
    for i in i {
        if assert_get_bool(eq(*i, make_bool(true))) {
            return true;
        }
    }
    return false;
}

pub unsafe fn evaluate(e: Slot, env: Slot) -> Slot {
    if is_atom(e) {
        if raw_is_symbol(e) {
            // lookup(e, env)
            todo!()
        } else if raw_or(&[
            is_number(e),
            is_string(e),
            is_char(e),
            is_boolean(e),
            is_vector(e),
        ]) {
            return e;
        } else {
            panic!("cannot evaluate")
        }
    } else {
        car(e);
        todo!()
    }
    todo!()
}
