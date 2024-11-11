use crate::make_object::*;
use crate::object_model::*;

/// # number

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
            let r = Number {
                head: ObjectHead {
                    tag: ObjectTag::Number,
                    moved: false,
                },
                value: r,
            };
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
