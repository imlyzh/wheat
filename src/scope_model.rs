use std::ptr::NonNull;

use crate::object_model::{ObjectHead, Slot, Symbol};



pub struct Scope {
    pub head: ObjectHead,
    pub name: Option<Symbol>,
    pub pointer: Slot,
    pub prev: Option<NonNull<Scope>>,
}
