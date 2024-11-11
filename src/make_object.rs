use crate::{object_model::*, vm_state::VMState};

pub static NULL: SingleData = SingleData {
    head: ObjectHead {
        tag: ObjectTag::Null,
        moved: true,
    },
    value: 0,
};

pub static FALSE: SingleData = SingleData {
    head: ObjectHead {
        tag: ObjectTag::Bool,
        moved: true,
    },
    value: 0,
};

pub static TRUE: SingleData = SingleData {
    head: ObjectHead {
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
    let r = vms.alloc(std::mem::size_of::<SingleData>());
    *(r as *mut SingleData) = SingleData {
        head: ObjectHead {
            tag: ObjectTag::Char,
            moved: false,
        },
        value: v as u64,
    };
    r
}

#[inline]
pub unsafe fn make_pair(vms: &mut VMState, car: Slot, cdr: Slot) -> Slot {
    let r = vms.alloc(std::mem::size_of::<Pair>());
    let r_ref = r as *mut Pair;
    (*r_ref).car = car;
    (*r_ref).cdr = cdr;
    r
}

#[inline]
pub unsafe fn make_symbol(vms: &mut VMState, sym: &str) -> Slot {
    vms.symbol_register(sym)
}
