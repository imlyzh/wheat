use std::ptr::NonNull;

// 4bit
#[repr(u8)]
pub enum ObjectTag {
    Null = 0,
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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number {
    pub moved: bool,
    // pub is_signed: bool,
    pub value: Slot,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pair {
    pub car: Slot,
    pub cdr: Slot,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector {
    pub moved: bool,
    pub length: usize,
    pub instance: [Slot; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct String {
    pub moved: bool,
    pub length: usize,
    pub instance: [u8; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Symbol {
    pub value: NonNull<String>,
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.value.as_ptr() == other.value.as_ptr()
    }
}

pub static VALUE_MASK: Slot = u64::MAX >> 12;

pub static NIL: Slot = 0;
pub static FALSE: Slot = (ObjectTag::Bool as u64) << 60;
pub static TRUE: Slot = FALSE | 1;

#[inline(always)]
pub fn get_tag(arg: Slot) -> u64 {
    arg >> 60
}

#[inline(always)]
pub fn get_is_moved(arg: Slot) -> u64 {
    (arg << 4) >> 63
}

#[inline(always)]
pub fn get_value(arg: Slot) -> u64 {
    arg & VALUE_MASK
}

/// Host (Rust) and guest (Wheat) language interaction operator

#[inline(always)]
pub unsafe fn assert_null(obj: Slot) {
    assert_eq!(get_tag(obj), ObjectTag::Null as u64);
}

#[inline(always)]
pub unsafe fn assert_get_bool(obj: Slot) -> bool {
    assert_eq!(get_tag(obj), ObjectTag::Bool as u64);
    get_value(obj) == 1
}

#[inline(always)]
pub unsafe fn assert_get_char(obj: Slot) -> u8 {
    assert_eq!(get_tag(obj), ObjectTag::Bool as u64);
    get_value(obj) as u8
}

#[inline(always)]
pub unsafe fn assert_get_number(obj: Slot) -> u64 {
    assert_eq!(get_tag(obj), ObjectTag::Number as u64);
    let r = get_value(obj) as *mut Number;
    (*r).value
}

#[inline(always)]
pub unsafe fn assert_get_pair(obj: Slot) -> Pair {
    assert_eq!(get_tag(obj), ObjectTag::Pair as u64);
    let r = get_value(obj) as *mut Pair;
    (*r).clone()
}