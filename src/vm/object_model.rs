use std::ptr::NonNull;

use crate::vm::vm_state::VMState;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObjectTag {
    Null = 0,
    Bool,
    Char,
    Number,
    Vector,
    String,
    Symbol,
    Object,
    Closure,
    NativeFunction,
    Opaque,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectHead {
    pub __align32: u32,
    pub __align16: u16,
    pub tag: ObjectTag,
    pub moved: bool,
}

pub trait Length {
    fn length(&self) -> usize;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SingleData {
    pub head: ObjectHead,
    pub value: u64,
}

impl SingleData {
    pub unsafe fn alloc(vms: &mut VMState) -> NonNull<SingleData> {
        let r = vms.alloc_with_gc(std::mem::size_of::<Self>());
        let r = r as *mut SingleData;
        NonNull::new(r).unwrap()
    }
}

impl Length for SingleData {
    fn length(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number {
    pub head: ObjectHead,
    pub value: i64,
}

impl Length for Number {
    fn length(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl Number {
    pub unsafe fn alloc(vms: &mut VMState) -> NonNull<Number> {
        let r = vms.alloc_with_gc(std::mem::size_of::<Self>());
        let r = r as *mut Number;
        NonNull::new(r).unwrap()
    }
}

// #[repr(C)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Pair {
//     pub head: ObjectHead,
//     pub car: Slot,
//     pub cdr: Slot,
// }

// impl Length for Pair {
//     fn length(&self) -> usize {
//         std::mem::size_of::<Self>()
//     }
// }

// impl Pair {
//     pub unsafe fn alloc(vms: &mut VMState) -> NonNull<Pair> {
//         let r = vms.alloc_with_gc(std::mem::size_of::<Self>());
//         let r = r as *mut Pair;
//         NonNull::new(r).unwrap()
//     }
// }

pub struct HiddenKlass {
    pub prev: *const HiddenKlass,
    pub name: *const Symbol,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Object {
    pub head: ObjectHead,
    pub klass: *const HiddenKlass,
    pub descriptor: Slot,
    pub element: Slot,
    pub instance: [Slot; 7],
}

impl Length for Object {
    fn length(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector {
    pub head: ObjectHead,
    pub length: usize,
    pub instance: [Slot; 1],
}

impl Length for Vector {
    fn length(&self) -> usize {
        std::mem::size_of::<Self>() + (self.length as usize - 1) * std::mem::size_of::<Slot>()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SingleByteString {
    pub head: ObjectHead,
    pub length: usize,
    pub instance: [u8; 1],
}

impl Length for SingleByteString {
    fn length(&self) -> usize {
        std::mem::size_of::<Self>() + (self.length as usize - 1) * std::mem::size_of::<u8>()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Hash, Copy, Eq, PartialOrd, Ord)]
pub struct Symbol {
    pub head: ObjectHead,
    pub value: NonNull<String>,
}

impl Length for Symbol {
    fn length(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.value.as_ptr() == other.value.as_ptr()
    }
}

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
    (*(obj as *mut SingleData)).value == 1
}

#[inline(always)]
pub unsafe fn assert_get_char(obj: Slot) -> u8 {
    assert_eq!(get_tag(obj), ObjectTag::Char);
    (*(obj as *mut SingleData)).value as u8
}

#[inline(always)]
pub unsafe fn assert_get_number(obj: Slot) -> i64 {
    assert_eq!(get_tag(obj), ObjectTag::Number);
    let r = obj as *mut Number;
    (*r).value
}

#[inline(always)]
pub unsafe fn assert_get_vector(obj: Slot) -> Vector {
    assert_eq!(get_tag(obj), ObjectTag::Vector);
    let r = obj as *mut Vector;
    *r
}

#[inline(always)]
pub unsafe fn assert_get_string(obj: Slot) -> SingleByteString {
    assert_eq!(get_tag(obj), ObjectTag::String);
    let r = obj as *mut SingleByteString;
    *r
}

#[inline(always)]
pub unsafe fn assert_get_symbol(obj: Slot) -> Symbol {
    assert_eq!(get_tag(obj), ObjectTag::Symbol);
    let r = obj as *mut Symbol;
    *r
}
