use crate::object_model::*;

/// # bool

macro_rules! gen_is {
    ($name: ident, $expr: expr) => {
        #[inline]
        pub unsafe fn $name(arg: Slot) -> Slot {
            if get_tag(arg) == $expr {
                (&TRUE) as *const Bool as Slot
            } else {
                (&FALSE) as *const Bool as Slot
            }
        }
    };
}

gen_is!(is_null, ObjectTag::Null);
gen_is!(is_boolean, ObjectTag::Bool);
gen_is!(is_char, ObjectTag::Char);
gen_is!(is_number, ObjectTag::Number);
gen_is!(is_pair, ObjectTag::Pair);
gen_is!(is_vector, ObjectTag::Vector);
gen_is!(is_string, ObjectTag::String);
gen_is!(is_symbol, ObjectTag::Symbol);
gen_is!(is_closure, ObjectTag::Closure);
gen_is!(is_native, ObjectTag::NativeFunction);

#[inline]
pub unsafe fn not(obj: Slot) -> Slot {
    if *(obj as *const Null) == NULL || *(obj as *const Bool) == FALSE {
        (&TRUE) as *const Bool as Slot
    } else {
        (&FALSE) as *const Bool as Slot
    }
}

#[inline]
pub fn eq(obj0: Slot, obj1: Slot) -> Slot {
    if obj0 == obj1 {
        (&TRUE) as *const Bool as Slot
    } else {
        (&FALSE) as *const Bool as Slot
    }
}

#[inline]
pub unsafe fn eqv(obj0: Slot, obj1: Slot) -> Slot {
    /* fastpath
    if arg0 == arg1 {
        return TRUE;
    }
    // */
    let tag0 = get_tag(obj0);
    let tag1 = get_tag(obj1);
    if tag0 != tag1 {
        return (&FALSE) as *const Bool as Slot;
    }
    if tag0 as u8 <= ObjectTag::Char as u8 {
        return eq(obj0, obj1);
    }
    match tag0 {
        // TODO: impl all type =?
        _ => (&FALSE) as *const Bool as Slot,
    }
}