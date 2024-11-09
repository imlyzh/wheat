use crate::object_model::*;

/// # String

pub unsafe fn make_string(k: Slot) -> Slot {
    assert_eq!(get_tag(k), ObjectTag::Number);
    let ptr = k as *const Number;
    let value = (*ptr).value as u8;
    let _v = SingleData {
        head: ObjectHead {
            tag: ObjectTag::Char,
            moved: false,
        },
        value: value as u64,
    };
    // todo: alloc
    todo!()
}

pub unsafe fn make_string_with_fill(k: Slot, char: Slot) -> Slot {
    assert_eq!(get_tag(k), ObjectTag::Number);
    assert_eq!(get_tag(char), ObjectTag::Char);
    let number_ptr = k as *const Number;
    let char_ptr = char as *const SingleData;
    let number = (*number_ptr).value;
    let char = (*char_ptr).value;
    // todo: alloc
    // let _v = String{ head: ObjectHead { tag: ObjectTag::Char, moved: false }, value };
    todo!()
}
