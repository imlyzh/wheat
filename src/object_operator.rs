use crate::object_model::*;

/// ## bool

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

/// ## pair

// TODO: cons
/// need heap alloc
pub unsafe fn cons(obj0: Slot, obj1: Slot) -> Slot {
    todo!()
}

#[inline]
pub unsafe fn car(pair: Slot) -> Slot {
    assert_eq!(get_tag(pair), ObjectTag::Pair);
    let ptr = pair as *const Pair;
    (*ptr).car as Slot
}

#[inline]
pub unsafe fn cdr(pair: Slot) -> Slot {
    assert_eq!(get_tag(pair), ObjectTag::Pair);
    let ptr = pair as *const Pair;
    (*ptr).cdr as Slot
}

#[inline]
pub unsafe fn set_car(pair: Slot, obj: Slot) {
    assert_eq!(get_tag(pair), ObjectTag::Pair);
    let ptr = pair as *mut Pair;
    (*ptr).car = obj;
}

#[inline]
pub unsafe fn set_cdr(pair: Slot, obj: Slot) {
    assert_eq!(get_tag(pair), ObjectTag::Pair);
    let ptr = pair as *mut Pair;
    (*ptr).cdr = obj;
}

/// need heap alloc
#[inline]
pub unsafe fn list(objs: &[Slot]) -> Slot {
    todo!()
}

#[inline]
pub unsafe fn raw_length(list: Slot) -> usize {
    if get_tag(list) == ObjectTag::Null {
        return 0;
    }
    assert_eq!(get_tag(list), ObjectTag::Pair);
    let next = cdr(list);
    if get_tag(list) == ObjectTag::Null {
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
    if get_tag(list) == ObjectTag::Null {
        return (&FALSE) as *const Bool as Slot;
    }
    if get_tag(list) == ObjectTag::Pair {
        if assert_get_bool(eq(car(list), obj)) {
            list
        } else {
            memq(obj, cdr(list))
        }
    } else {
        (&NULL) as *const Null as Slot
    }
}

pub unsafe fn memv(obj: Slot, list: Slot) -> Slot {
    if get_tag(list) == ObjectTag::Null {
        return (&FALSE) as *const Bool as Slot;
    }
    if get_tag(list) == ObjectTag::Pair {
        if assert_get_bool(eqv(car(list), obj)) {
            list
        } else {
            memv(obj, cdr(list))
        }
    } else {
        return (&NULL) as *const Null as Slot;
    }
}

/* TODO: impl equal
pub unsafe fn member(obj: Slot, list: Slot) -> Slot {
    if get_tag(list) == ObjectTag::Null {
        return (&FALSE) as *const Bool as Slot;
    }
    if get_tag(list) == ObjectTag::Pair {
        if assert_get_bool(equal(car(list), obj))  {
            list
        } else {
            member(obj, cdr(list))
        }
    } else {
        return (&NULL) as *const Null as Slot;
    }
}
*/

pub unsafe fn assq(obj: Slot, list: Slot) -> Slot {
    if get_tag(list) == ObjectTag::Null {
        return (&FALSE) as *const Bool as Slot;
    }
    if get_tag(list) == ObjectTag::Pair {
        if assert_get_bool(eq(car(car(list)), obj)) {
            list
        } else {
            assq(obj, cdr(list))
        }
    } else {
        (&NULL) as *const Null as Slot
    }
}

pub unsafe fn assv(obj: Slot, list: Slot) -> Slot {
    if get_tag(list) == ObjectTag::Null {
        return (&FALSE) as *const Bool as Slot;
    }
    if get_tag(list) == ObjectTag::Pair {
        if assert_get_bool(eqv(car(car(list)), obj)) {
            list
        } else {
            assq(obj, cdr(list))
        }
    } else {
        (&NULL) as *const Null as Slot
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
    debug_assert_eq!(get_tag(i), ObjectTag::Number);
    let ptr = i as *const Number;
    (*ptr).value == 0
}

#[inline]
pub unsafe fn raw_is_positive(i: Slot) -> bool {
    debug_assert_eq!(get_tag(i), ObjectTag::Number);
    let ptr = i as *const Number;
    ((*ptr).value >> 63) == 0
}

#[inline]
pub unsafe fn raw_is_negative(i: Slot) -> bool {
    debug_assert_eq!(get_tag(i), ObjectTag::Number);
    let ptr = i as *const Number;
    ((*ptr).value >> 63) == 0
}

#[inline]
pub unsafe fn raw_is_odd(i: Slot) -> bool {
    debug_assert_eq!(get_tag(i), ObjectTag::Number);
    let ptr = i as *const Number;
    ((*ptr).value % 2) != 0
}

#[inline]
pub unsafe fn raw_is_even(i: Slot) -> bool {
    debug_assert_eq!(get_tag(i), ObjectTag::Number);
    let ptr = i as *const Number;
    ((*ptr).value % 2) == 0
}

#[inline]
pub unsafe fn raw_math_eq(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value == (*ptr1).value
}

#[inline]
pub unsafe fn raw_math_less(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value < (*ptr1).value
}

#[inline]
pub unsafe fn raw_math_greater(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value > (*ptr1).value
}

#[inline]
pub unsafe fn raw_math_less_eq(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value <= (*ptr1).value
}

#[inline]
pub unsafe fn raw_math_greater_eq(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value >= (*ptr1).value
}



macro_rules! unwary_bool_op_to_wheat {
    ($name:ident, $raw_name:ident) => {
        #[inline]
        pub unsafe fn $name(x0: Slot) -> Slot {
            if $raw_name(x0) {
                (&TRUE) as *const Bool as Slot
            } else {
                (&FALSE) as *const Bool as Slot
            }
        }
    };
}

macro_rules! binary_bool_op_to_wheat {
    ($name:ident, $raw_name:ident) => {
        #[inline]
        pub unsafe fn $name(x0: Slot, x1: Slot) -> Slot {
            if $raw_name(x0, x1) {
                (&TRUE) as *const Bool as Slot
            } else {
                (&FALSE) as *const Bool as Slot
            }
        }
    };
}

unwary_bool_op_to_wheat!(is_zero, raw_is_zero);
unwary_bool_op_to_wheat!(is_positive, raw_is_positive);
unwary_bool_op_to_wheat!(is_negative, raw_is_negative);
unwary_bool_op_to_wheat!(is_odd, raw_is_odd);
unwary_bool_op_to_wheat!(is_even, raw_is_even);
binary_bool_op_to_wheat!(math_eq, raw_math_eq);
binary_bool_op_to_wheat!(math_less, raw_math_less);
binary_bool_op_to_wheat!(math_less_eq, raw_math_less_eq);
binary_bool_op_to_wheat!(math_greater, raw_math_greater);
binary_bool_op_to_wheat!(math_greater_eq, raw_math_greater_eq);

#[inline]
pub unsafe fn raw_math_add(x0: Slot, x1: Slot) -> i64 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value + (*ptr1).value
}

#[inline]
pub unsafe fn raw_math_sub(x0: Slot, x1: Slot) -> i64 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value - (*ptr1).value
}


#[inline]
pub unsafe fn raw_math_mul(x0: Slot, x1: Slot) -> i64 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value * (*ptr1).value
}

#[inline]
pub unsafe fn raw_math_div(x0: Slot, x1: Slot) -> i64 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value / (*ptr1).value
}

#[inline]
pub unsafe fn raw_math_mod(x0: Slot, x1: Slot) -> i64 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value % (*ptr1).value
}

macro_rules! binary_number_op_to_wheat {
    ($name:ident, $raw_name:ident) => {
        #[inline]
        pub unsafe fn $name(x0: Slot, x1: Slot) -> Slot {
            let r = $raw_name(x0, x1);
            let r = Number { head: ObjectHead { tag: ObjectTag::Number, moved: false }, value: r };
            // alloc
            todo!()
        }
    };
}

binary_number_op_to_wheat!(math_add, raw_math_add);
binary_number_op_to_wheat!(math_sub, raw_math_sub);
binary_number_op_to_wheat!(math_mul, raw_math_mul);
binary_number_op_to_wheat!(math_div, raw_math_div);
binary_number_op_to_wheat!(math_mod, raw_math_mod);


/// Char

#[inline]
pub unsafe fn raw_char_eq(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value == (*ptr1).value
}

#[inline]
pub unsafe fn raw_char_less(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value < (*ptr1).value
}

#[inline]
pub unsafe fn raw_char_greater(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value > (*ptr1).value
}

#[inline]
pub unsafe fn raw_char_less_eq(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value <= (*ptr1).value
}

#[inline]
pub unsafe fn raw_char_greater_eq(x0: Slot, x1: Slot) -> bool {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Number;
    let ptr1 = x1 as *const Number;
    (*ptr0).value >= (*ptr1).value
}



macro_rules! unwary_bool_op_to_wheat {
    ($name:ident, $raw_name:ident) => {
        #[inline]
        pub unsafe fn $name(x0: Slot) -> Slot {
            if $raw_name(x0) {
                (&TRUE) as *const Bool as Slot
            } else {
                (&FALSE) as *const Bool as Slot
            }
        }
    };
}

macro_rules! binary_bool_op_to_wheat {
    ($name:ident, $raw_name:ident) => {
        #[inline]
        pub unsafe fn $name(x0: Slot, x1: Slot) -> Slot {
            if $raw_name(x0, x1) {
                (&TRUE) as *const Bool as Slot
            } else {
                (&FALSE) as *const Bool as Slot
            }
        }
    };
}

// unwary_bool_op_to_wheat!(is_zero, raw_is_zero);
binary_bool_op_to_wheat!(char_eq, raw_char_eq);
binary_bool_op_to_wheat!(char_less, raw_char_less);
binary_bool_op_to_wheat!(char_less_eq, raw_char_less_eq);
binary_bool_op_to_wheat!(char_greater, raw_char_greater);
binary_bool_op_to_wheat!(char_greater_eq, raw_char_greater_eq);


#[inline]
pub unsafe fn raw_char_add(x0: Slot, x1: Slot) -> u8 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Char;
    let ptr1 = x1 as *const Char;
    (*ptr0).value + (*ptr1).value
}

#[inline]
pub unsafe fn raw_char_sub(x0: Slot, x1: Slot) -> u8 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Char;
    let ptr1 = x1 as *const Char;
    (*ptr0).value - (*ptr1).value
}


#[inline]
pub unsafe fn raw_char_mul(x0: Slot, x1: Slot) -> u8 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Char;
    let ptr1 = x1 as *const Char;
    (*ptr0).value * (*ptr1).value
}

#[inline]
pub unsafe fn raw_char_div(x0: Slot, x1: Slot) -> u8 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Char;
    let ptr1 = x1 as *const Char;
    (*ptr0).value / (*ptr1).value
}

#[inline]
pub unsafe fn raw_char_mod(x0: Slot, x1: Slot) -> u8 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const Char;
    let ptr1 = x1 as *const Char;
    (*ptr0).value % (*ptr1).value
}

macro_rules! binary_number_op_to_wheat {
    ($name:ident, $raw_name:ident) => {
        #[inline]
        pub unsafe fn $name(x0: Slot, x1: Slot) -> Slot {
            let r = $raw_name(x0, x1);
            let _r = Char { head: ObjectHead { tag: ObjectTag::Char, moved: false }, value: r };
            // alloc
            todo!()
        }
    };
}

binary_number_op_to_wheat!(char_add, raw_char_add);
binary_number_op_to_wheat!(char_sub, raw_char_sub);
binary_number_op_to_wheat!(char_mul, raw_char_mul);
binary_number_op_to_wheat!(char_div, raw_char_div);
binary_number_op_to_wheat!(char_mod, raw_char_mod);

pub unsafe fn char2int(obj: Slot) -> Slot {
    assert_eq!(get_tag(obj), ObjectTag::Char);
    let ptr = obj as *const Char;
    let value = (*ptr).value as i64;
    let _v = Number { head: ObjectHead { tag: ObjectTag::Number, moved: false }, value };
    // todo: alloc
    todo!()
}

pub unsafe fn int2char(obj: Slot) -> Slot {
    assert_eq!(get_tag(obj), ObjectTag::Char);
    let ptr = obj as *const Number;
    let value = (*ptr).value as u8;
    let _v = Char { head: ObjectHead { tag: ObjectTag::Char, moved: false }, value };
    // todo: alloc
    todo!()
}

/// String

pub unsafe fn make_string(k: Slot) -> Slot {
    assert_eq!(get_tag(k), ObjectTag::Number);
    let ptr = k as *const Number;
    let value = (*ptr).value as u8;
    let _v = Char { head: ObjectHead { tag: ObjectTag::Char, moved: false }, value };
    // todo: alloc
    todo!()
}

pub unsafe fn make_string_with_fill(k: Slot, char: Slot) -> Slot {
    assert_eq!(get_tag(k), ObjectTag::Number);
    assert_eq!(get_tag(char), ObjectTag::Char);
    let number_ptr = k as *const Number;
    let char_ptr = char as *const Char;
    let number= (*number_ptr).value;
    let char = (*char_ptr).value;
    // todo: alloc
    // let _v = String{ head: ObjectHead { tag: ObjectTag::Char, moved: false }, value };
    todo!()
}