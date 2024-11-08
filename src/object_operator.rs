use crate::object_model::*;

/// ## bool

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

/// ## pair

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
            memv(obj, cdr(list))
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
            member(obj, cdr(list))
        }
    } else {
        NIL
    }
}
*/


pub unsafe fn assq(obj: Slot, list: Slot) -> Slot {
    if list == NIL {
        return FALSE;
    }
    if get_tag(list) == ObjectTag::Pair as u64 {
        if assert_get_bool(eq(car(car(list)), obj))  {
            list
        } else {
            assq(obj, cdr(list))
        }
    } else {
        NIL
    }
}

pub unsafe fn assv(obj: Slot, list: Slot) -> Slot {
    if list == NIL {
        return FALSE;
    }
    if get_tag(list) == ObjectTag::Pair as u64 {
        if assert_get_bool(eqv(car(car(list)), obj))  {
            list
        } else {
            assq(obj, cdr(list))
        }
    } else {
        NIL
    }
}

/* TODO: impl equal
pub unsafe fn assoc(obj: Slot, list: Slot) -> Slot {
    if list == NIL {
        return FALSE;
    }
    if get_tag(list) == ObjectTag::Pair as u64 {
        if assert_get_bool(equal(car(car(list)), obj))  {
            list
        } else {
            assoc(obj, cdr(list))
        }
    } else {
        NIL
    }
}
*/

/// ## Symbol

/// need heap alloc
pub unsafe fn symbol2string(obj: Slot) -> Slot {
    let sym = assert_get_symbol(obj);
    todo!()
}

/*
///  need heap alloc
pub unsafe fn string2symbol(obj: Slot) -> Slot {
    let str = assert_get_string(obj);
    let ptr = (&str.instance) as *const u8;
    let slice = std::slice::from_raw_parts(ptr, str.length);
    let r = str::from_utf8(slice).unwrap();
    // TODO: let r = r.global_intern();
    // r as Slot
    todo!()
}
// */

/// ## number

#[inline]
pub unsafe fn raw_is_zero(i: Slot) -> bool {
    debug_assert_eq!(get_tag(i), ObjectTag::Number as u64);
    let ptr = get_value(i) as *const Number;
    (*ptr).value == 0
}

#[inline]
pub unsafe fn raw_is_positive(i: Slot) -> bool {
    debug_assert_eq!(get_tag(i), ObjectTag::Number as u64);
    let ptr = get_value(i) as *const Number;
    ((*ptr).value >> 63) == 0
}

#[inline]
pub unsafe fn raw_is_negative(i: Slot) -> bool {
    debug_assert_eq!(get_tag(i), ObjectTag::Number as u64);
    let ptr = get_value(i) as *const Number;
    ((*ptr).value >> 63) == 0
}

#[inline]
pub unsafe fn raw_is_odd(i: Slot) -> bool {
    debug_assert_eq!(get_tag(i), ObjectTag::Number as u64);
    let ptr = get_value(i) as *const Number;
    ((*ptr).value % 2) != 0
}

#[inline]
pub unsafe fn raw_is_even(i: Slot) -> bool {
    debug_assert_eq!(get_tag(i), ObjectTag::Number as u64);
    let ptr = get_value(i) as *const Number;
    ((*ptr).value % 2) == 0
}

#[inline]
pub unsafe fn raw_math_eq(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number as u64);
    assert_eq!(get_tag(x1), ObjectTag::Number as u64);
    let ptr0 = get_value(x0) as *const Number;
    let ptr1 = get_value(x1) as *const Number;
    (*ptr0).value == (*ptr1).value
}

#[inline]
pub unsafe fn raw_math_less(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number as u64);
    assert_eq!(get_tag(x1), ObjectTag::Number as u64);
    let ptr0 = get_value(x0) as *const Number;
    let ptr1 = get_value(x1) as *const Number;
    (*ptr0).value < (*ptr1).value
}

#[inline]
pub unsafe fn raw_math_greater(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number as u64);
    assert_eq!(get_tag(x1), ObjectTag::Number as u64);
    let ptr0 = get_value(x0) as *const Number;
    let ptr1 = get_value(x1) as *const Number;
    (*ptr0).value > (*ptr1).value
}

#[inline]
pub unsafe fn raw_math_less_eq(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number as u64);
    assert_eq!(get_tag(x1), ObjectTag::Number as u64);
    let ptr0 = get_value(x0) as *const Number;
    let ptr1 = get_value(x1) as *const Number;
    (*ptr0).value <= (*ptr1).value
}


#[inline]
pub unsafe fn raw_math_greater_eq(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number as u64);
    assert_eq!(get_tag(x1), ObjectTag::Number as u64);
    let ptr0 = get_value(x0) as *const Number;
    let ptr1 = get_value(x1) as *const Number;
    (*ptr0).value >= (*ptr1).value
}


macro_rules! unwary_op_to_wheat {
    ($name:ident, $raw_name:ident) => {
        #[inline]
        pub unsafe fn $name(x0: Slot) -> Slot {
            if $raw_name(x0) {
                TRUE
            } else {
                FALSE
            }
        }
    };
}

macro_rules! binary_op_to_wheat {
    ($name:ident, $raw_name:ident) => {
        #[inline]
        pub unsafe fn $name(x0: Slot, x1: Slot) -> Slot {
            if $raw_name(x0, x1) {
                TRUE
            } else {
                FALSE
            }
        }
    };
}

unwary_op_to_wheat!(is_zero, raw_is_zero);
unwary_op_to_wheat!(is_positive, raw_is_positive);
unwary_op_to_wheat!(is_negative, raw_is_negative);
unwary_op_to_wheat!(is_odd, raw_is_odd);
unwary_op_to_wheat!(is_even, raw_is_even);
binary_op_to_wheat!(math_eq, raw_math_eq);
binary_op_to_wheat!(math_less, raw_math_less);
binary_op_to_wheat!(math_less_eq, raw_math_less_eq);
binary_op_to_wheat!(math_greater, raw_math_greater);
binary_op_to_wheat!(math_greater_eq, raw_math_greater_eq);