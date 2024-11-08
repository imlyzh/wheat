
// 4bit
#[repr(u8)]
pub enum ObjectTag {
    Nil = 0,
    Bool,
    Char,
    Number,
    Pair,
    Vector,
    String,
    Symbol,
    Closure,
    NativeFunction,
}

pub type Slot = u64;

pub static VALUE_MASK: Slot = u64::MAX >> 8;
pub static NIL: Slot = 0;
pub static FALSE: Slot = (ObjectTag::Bool as u64) << 56;
pub static TRUE: Slot = FALSE | 1;

#[inline(always)]
pub fn get_tag(arg: Slot) -> u64 {
    arg >> 56
}

#[inline(always)]
pub fn get_value(arg: Slot) -> u64 {
    arg & VALUE_MASK
}

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

gen_is!(is_nil, ObjectTag::Nil);
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
pub fn not(arg: Slot) -> Slot {
    if arg == NIL || arg == FALSE {
        TRUE
    } else {
        FALSE
    }
}

#[inline]
pub fn eq(arg0: Slot, arg1: Slot) -> Slot {
    if arg0 == arg1 {
        TRUE
    } else {
        FALSE
    }
}

#[inline]
pub fn eqv(arg0: Slot, arg1: Slot) -> Slot {
    let tag0 = get_tag(arg0);
    let tag1 = get_tag(arg1);
    if tag0 != tag1 {
        return FALSE;
    }
    if tag0 <= ObjectTag::Char as u64 {
        return eq(arg0, arg1);
    }
    match tag0 {
        _ => FALSE,
    }
}
