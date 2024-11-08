use std::ptr::NonNull;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectHead {
    pub tag: ObjectTag,
    pub moved: bool,
    // _align: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Null {
    pub head: ObjectHead,
    // pub is_signed: bool,
    pub value: ()
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bool {
    pub head: ObjectHead,
    // pub is_signed: bool,
    pub value: bool
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char {
    pub head: ObjectHead,
    // pub is_signed: bool,
    pub value: u8
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number {
    pub head: ObjectHead,
    // pub is_signed: bool,
    pub value: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pair {
    pub head: ObjectHead,
    pub car: Slot,
    pub cdr: Slot,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector {
    pub head: ObjectHead,
    pub length: usize,
    pub instance: [Slot; 1],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct String {
    pub head: ObjectHead,
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

pub static NULL: Null = Null { head: ObjectHead { tag: ObjectTag::Null, moved: false }, value: () };
pub static FALSE: Bool = Bool { head: ObjectHead { tag: ObjectTag::Null, moved: false }, value: false };
pub static TRUE:  Bool = Bool { head: ObjectHead { tag: ObjectTag::Null, moved: false }, value: true };

pub type Slot = *mut ObjectHead;

#[inline(always)]
pub unsafe fn get_tag(arg: Slot) -> ObjectTag {
    (*arg).tag
}

#[inline(always)]
pub unsafe fn get_is_moved(arg: Slot) -> bool {
    (*arg).moved
}

#[inline(always)]
pub unsafe fn assert_null(obj: Slot) {
    assert_eq!(get_tag(obj), ObjectTag::Null);
}

#[inline(always)]
pub unsafe fn assert_get_bool(obj: Slot) -> bool {
    assert_eq!(get_tag(obj), ObjectTag::Bool);
    (*(obj as *mut Bool)).value
}

#[inline(always)]
pub unsafe fn assert_get_char(obj: Slot) -> u8 {
    assert_eq!(get_tag(obj), ObjectTag::Char);
    (*(obj as *mut Char)).value
}

#[inline(always)]
pub unsafe fn assert_get_number(obj: Slot) -> i64 {
    assert_eq!(get_tag(obj), ObjectTag::Number);
    let r = obj as *mut Number;
    (*r).value
}

#[inline(always)]
pub unsafe fn assert_get_pair(obj: Slot) -> Pair {
    assert_eq!(get_tag(obj), ObjectTag::Pair);
    let r = obj as *mut Pair;
    *r
}

#[inline(always)]
pub unsafe fn assert_get_vector(obj: Slot) -> Vector {
    assert_eq!(get_tag(obj), ObjectTag::Pair);
    let r = obj as *mut Vector;
    *r
}

#[inline(always)]
pub unsafe fn assert_get_string(obj: Slot) -> String {
    assert_eq!(get_tag(obj), ObjectTag::Pair);
    let r = obj as *mut String;
    *r
}


#[inline(always)]
pub unsafe fn assert_get_symbol(obj: Slot) -> Symbol {
    assert_eq!(get_tag(obj), ObjectTag::Pair);
    let r = obj as *mut Symbol;
    *r
}