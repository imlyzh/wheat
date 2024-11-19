use crate::make_object::*;
use crate::{object_model::*, vm::vm_state::VMState};

use super::tageq::{eq, eqv};

/// # pair

// TODO: cons
/// need heap alloc
pub unsafe fn cons(vms: &mut VMState, obj0: Slot, obj1: Slot) -> Slot {
    make_pair(vms, obj0, obj1)
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
        return make_bool(false);
    }
    if get_tag(list) == ObjectTag::Pair {
        if assert_get_bool(eq(car(list), obj)) {
            list
        } else {
            memq(obj, cdr(list))
        }
    } else {
        make_null()
    }
}

pub unsafe fn memv(obj: Slot, list: Slot) -> Slot {
    if get_tag(list) == ObjectTag::Null {
        return make_bool(false);
    }
    if get_tag(list) == ObjectTag::Pair {
        if assert_get_bool(eqv(car(list), obj)) {
            list
        } else {
            memv(obj, cdr(list))
        }
    } else {
        make_null()
    }
}

/* TODO: impl equal
pub unsafe fn member(obj: Slot, list: Slot) -> Slot {
    if get_tag(list) == ObjectTag::Null {
        return make_bool(false);
    }
    if get_tag(list) == ObjectTag::Pair {
        if assert_get_bool(equal(car(list), obj))  {
            list
        } else {
            member(obj, cdr(list))
        }
    } else {
        make_null()
    }
}
*/

pub unsafe fn assq(obj: Slot, list: Slot) -> Slot {
    if get_tag(list) == ObjectTag::Null {
        return make_bool(false);
    }
    if get_tag(list) == ObjectTag::Pair {
        if assert_get_bool(eq(car(car(list)), obj)) {
            list
        } else {
            assq(obj, cdr(list))
        }
    } else {
        make_null()
    }
}

pub unsafe fn assv(obj: Slot, list: Slot) -> Slot {
    if get_tag(list) == ObjectTag::Null {
        return make_bool(false);
    }
    if get_tag(list) == ObjectTag::Pair {
        if assert_get_bool(eqv(car(car(list)), obj)) {
            list
        } else {
            assq(obj, cdr(list))
        }
    } else {
        make_null()
    }
}

/* TODO: impl equal
pub unsafe fn assoc(obj: Slot, list: Slot) -> Slot {
    if list == NIL {
        return make_bool(false);
    }
    if get_tag(list) == ObjectTag::Pair as u64 {
        if assert_get_bool(equal(car(car(list)), obj))  {
            list
        } else {
            assoc(obj, cdr(list))
        }
    } else {
        make_null()
    }
}
*/
