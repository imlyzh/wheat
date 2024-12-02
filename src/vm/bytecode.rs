#[repr(u8)]
pub enum VMByteCode {
    panic,
    unreachable,

    get_type,

    get_local,
    set_local,

    // atomic
    load_nil,
    load_true,
    load_false,

    load_const,

    make_char,
    make_const_char,

    // integer

    add,
    sub,
    mul,
    div,
    r#mod,
    lshift,
    rshift,

    eqz,
    eq,
    ne,
    lt,
    gt,
    le,
    ge,

    // pair
    car,
    cdr,
    set_car,
    set_cdr,
    cons,

    // vector
    get_length,
    get_element,
    set_element,
    make_vector,

    // struct
    get_struct_tag_length,
    get_property,
    set_property,
    make_struct,

    if_true_jump,
    if_false_jump,

    call_local,
    call,
    r#return,
}