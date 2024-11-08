use crate::object_model::*;

macro_rules! gen_is {
    ($name: ident, $expr: expr) => {
        #[inline]
        pub fn $name(arg: Slot) -> Slot {
            if get_tag(arg) == $expr as u64 {
                TRUE
            } else {
                FALSE
            }
        }
    };
}

// bool

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
pub fn not(obj: Slot) -> Slot {
    if obj == NIL || obj == FALSE {
        TRUE
    } else {
        FALSE
    }
}

#[inline]
pub fn eq(obj0: Slot, obj1: Slot) -> Slot {
    if obj0 == obj1 {
        TRUE
    } else {
        FALSE
    }
}

#[inline]
pub fn eqv(obj0: Slot, obj1: Slot) -> Slot {
    /* fastpath
    if arg0 == arg1 {
        return TRUE;
    }
    // */
    let tag0 = get_tag(obj0);
    let tag1 = get_tag(obj1);
    if tag0 != tag1 {
        return FALSE;
    }
    if tag0 <= ObjectTag::Char as u64 {
        return eq(obj0, obj1);
    }
    match tag0 {
        // TODO: impl all type =?
        _ => FALSE,
    }
}

// pair

// TODO: cons
/// need heap alloc
pub unsafe fn cons(obj0: Slot, obj1: Slot) -> Slot {
    todo!()
}

#[inline]
pub unsafe fn car(pair: Slot) -> Slot {
    assert_eq!(get_tag(pair), ObjectTag::Pair as u64);
    let ptr = get_value(pair) as *const Pair;
    (*ptr).car
}

#[inline]
pub unsafe fn cdr(pair: Slot) -> Slot {
    assert_eq!(get_tag(pair), ObjectTag::Pair as u64);
    let ptr = get_value(pair) as *const Pair;
    (*ptr).cdr
}

#[inline]
pub unsafe fn set_car(pair: Slot, obj: Slot) {
    assert_eq!(get_tag(pair), ObjectTag::Pair as u64);
    let ptr = get_value(pair) as *mut Pair;
    (*ptr).car = obj;
}

#[inline]
pub unsafe fn set_cdr(pair: Slot, obj: Slot) {
    assert_eq!(get_tag(pair), ObjectTag::Pair as u64);
    let ptr = get_value(pair) as *mut Pair;
    (*ptr).cdr = obj;
}

/// need heap alloc
#[inline]
pub unsafe fn list(objs: &[Slot]) -> Slot {
    todo!()
}

#[inline]
pub unsafe fn raw_length(list: Slot) -> usize {
    if list == NIL {
        return 0;
    }
    assert_eq!(get_tag(list), ObjectTag::Pair as u64);
    let next = cdr(list);
    if next == NIL {
        return 1;
    }
    raw_length(next) + 1
}

/// need heap alloc number
#[inline]
pub unsafe fn length(list: Slot) -> Slot {
    let len = raw_length(list);
    todo!()
}

/// need heap alloc
pub unsafe fn append(list0: Slot, list1: Slot) -> Slot {
    todo!()
}

pub unsafe fn memq(obj: Slot, list: Slot) -> Slot {
    if list == NIL {
        return FALSE;
    }
    if get_tag(list) == ObjectTag::Pair as u64 {
        if assert_get_bool(eq(car(list), obj))  {
            list
        } else {
            memq(obj, cdr(list))
        }
    } else {
        NIL
    }
}

pub unsafe fn memv(obj: Slot, list: Slot) -> Slot {
    if list == NIL {
        return FALSE;
    }
    if get_tag(list) == ObjectTag::Pair as u64 {
        if assert_get_bool(eqv(car(list), obj))  {
            list
        } else {
            memq(obj, cdr(list))
        }
    } else {
        NIL
    }
}

/* TODO: impl equal
pub unsafe fn member(obj: Slot, list: Slot) -> Slot {
    if list == NIL {
        return FALSE;
    }
    if get_tag(list) == ObjectTag::Pair as u64 {
        if assert_get_bool(equal(car(list), obj))  {
            list
        } else {
            memq(obj, cdr(list))
        }
    } else {
        NIL
    }
}
*/