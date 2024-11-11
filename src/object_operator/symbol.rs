use crate::object_model::{assert_get_symbol, Slot};

/// # Symbol

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
