use crate::vm::make_object::*;
use crate::vm::object_model::*;

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
                make_bool(true)
            } else {
                make_bool(false)
            }
        }
    };
}

macro_rules! binary_bool_op_to_wheat {
    ($name:ident, $raw_name:ident) => {
        #[inline]
        pub unsafe fn $name(x0: Slot, x1: Slot) -> Slot {
            if $raw_name(x0, x1) {
                make_bool(true)
            } else {
                make_bool(false)
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
    let ptr0 = x0 as *const SingleData;
    let ptr1 = x1 as *const SingleData;
    ((*ptr0).value as u8) + ((*ptr1).value as u8)
}

#[inline]
pub unsafe fn raw_char_sub(x0: Slot, x1: Slot) -> u8 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const SingleData;
    let ptr1 = x1 as *const SingleData;
    ((*ptr0).value as u8) - ((*ptr1).value as u8)
}

#[inline]
pub unsafe fn raw_char_mul(x0: Slot, x1: Slot) -> u8 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const SingleData;
    let ptr1 = x1 as *const SingleData;
    ((*ptr0).value as u8) * ((*ptr1).value as u8)
}

#[inline]
pub unsafe fn raw_char_div(x0: Slot, x1: Slot) -> u8 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const SingleData;
    let ptr1 = x1 as *const SingleData;
    ((*ptr0).value as u8) / ((*ptr1).value as u8)
}

#[inline]
pub unsafe fn raw_char_mod(x0: Slot, x1: Slot) -> u8 {
    assert_eq!(get_tag(x0), ObjectTag::Number);
    assert_eq!(get_tag(x1), ObjectTag::Number);
    let ptr0 = x0 as *const SingleData;
    let ptr1 = x1 as *const SingleData;
    ((*ptr0).value as u8) % ((*ptr1).value as u8)
}

macro_rules! binary_number_op_to_wheat {
    ($name:ident, $raw_name:ident) => {
        #[inline]
        pub unsafe fn $name(x0: Slot, x1: Slot) -> Slot {
            let r = $raw_name(x0, x1);
            let _r = SingleData {
                head: ObjectHead {
                    __align32: 0,
                    __align16: 0,
                    tag: ObjectTag::Char,
                    moved: false,
                },
                value: r as u64,
            };
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
    let ptr = obj as *const SingleData;
    let value = (*ptr).value as i64;
    let _v = Number {
        head: ObjectHead {
            __align32: 0,
            __align16: 0,
            tag: ObjectTag::Number,
            moved: false,
        },
        value,
    };
    // todo: alloc
    todo!()
}

pub unsafe fn int2char(obj: Slot) -> Slot {
    assert_eq!(get_tag(obj), ObjectTag::Char);
    let ptr = obj as *const Number;
    let value = (*ptr).value as u8;
    let _v = SingleData {
        head: ObjectHead {
            __align32: 0,
            __align16: 0,
            tag: ObjectTag::Char,
            moved: false,
        },
        value: value as u64,
    };
    // todo: alloc
    todo!()
}
