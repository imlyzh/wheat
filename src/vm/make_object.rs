use super::object_model::*;
use super::vm_state::VMState;

pub static NULL: SingleData = SingleData {
    head: ObjectHead {
        __align32: 0,
        __align16: 0,
        tag: ObjectTag::Null,
        moved: true,
    },
    value: 0,
};

pub static FALSE: SingleData = SingleData {
    head: ObjectHead {
        __align32: 0,
        __align16: 0,
        tag: ObjectTag::Bool,
        moved: true,
    },
    value: 0,
};

pub static TRUE: SingleData = SingleData {
    head: ObjectHead {
        __align32: 0,
        __align16: 0,
        tag: ObjectTag::Bool,
        moved: true,
    },
    value: 1,
};

#[inline]
pub unsafe fn make_null() -> Slot {
    (&NULL) as *const SingleData as Slot
}

#[inline]
pub fn make_bool(b: bool) -> Slot {
    if b {
        (&TRUE) as *const SingleData as Slot
    } else {
        (&FALSE) as *const SingleData as Slot
    }
}

#[inline]
pub unsafe fn make_char(vms: &mut VMState, v: u8) -> Slot {
    let r: *mut ObjectHead = vms.alloc_with_gc(std::mem::size_of::<SingleData>());
    *(r as *mut SingleData) = SingleData {
        head: ObjectHead {
            __align32: 0,
            __align16: 0,
            tag: ObjectTag::Char,
            moved: false,
        },
        value: v as u64,
    };
    r
}

#[inline]
pub unsafe fn make_integer(vms: &mut VMState, v: i64) -> Slot {
    let r = vms.alloc_with_gc(std::mem::size_of::<Number>());
    *(r as *mut Number) = Number {
        head: ObjectHead {
            __align32: 0,
            __align16: 0,
            tag: ObjectTag::Char,
            moved: false,
        },
        value: v as i64,
    };
    r
}

#[inline]
pub unsafe fn make_pair(vms: &mut VMState, car: Slot, cdr: Slot) -> Slot {
    let r = vms.alloc_with_gc(std::mem::size_of::<Pair>());
    let r_ref = r as *mut Pair;
    (*r_ref).car = car;
    (*r_ref).cdr = cdr;
    r
}

/// # String

#[inline]
pub unsafe fn make_symbol(vms: &mut VMState, sym: &str) -> Slot {
    vms.symbol_register(sym)
}

pub unsafe fn make_string(vms: &mut VMState, k: usize) -> Slot {
    // assert_eq!(get_tag(k), ObjectTag::Number);
    let v = vms.alloc_with_gc(std::mem::size_of::<SingleByteString>() + k - 1);
    let strv = v as *mut SingleByteString;
    (*strv).head = ObjectHead {
        __align32: 0,
        __align16: 0,
        tag: ObjectTag::String,
        moved: false,
    };
    (*strv).length = k;
    v
}

// pub unsafe fn make_string_with_fill(k: Slot, char: Slot) -> Slot {
//     assert_eq!(get_tag(k), ObjectTag::Number);
//     assert_eq!(get_tag(char), ObjectTag::Char);
//     let number_ptr = k as *const Number;
//     let char_ptr = char as *const SingleData;
//     let number = (*number_ptr).value;
//     let char = (*char_ptr).value;
//     // todo: alloc
//     // let _v = String{ head: ObjectHead { tag: ObjectTag::Char, moved: false }, value };
//     todo!()
// }
