use crate::vm::{
    make_object::{make_hashmap, make_hidden_class},
    object_model::{HashMap, HiddenKlass, Object, Slot, Symbol},
    vm_state::VMState,
};

pub fn property_size(object: *const HiddenKlass) -> usize {
    if object.is_null() {
        0
    } else {
        1 + property_size(unsafe { object.as_ref().unwrap().prev })
    }
}

pub fn find_properry_offset(klass: *const HiddenKlass, name: *const Symbol) -> Option<usize> {
    if klass.is_null() {
        None
    } else {
        Some(property_size(unsafe { klass.as_ref().unwrap_unchecked() }))
    }
}

pub fn object_to_hashmap(vms: &mut VMState, object: &mut Object, name: *const Symbol, value: Slot) -> *mut HashMap {
    let hashmap = unsafe { make_hashmap(vms, 8) } as *mut HashMap;
    todo!();
    hashmap
}

pub fn store_property(vms: &mut VMState, object: &mut Object, name: *const Symbol, value: Slot) -> Option<*mut HashMap> {
    if let Some(offset) = find_properry_offset(object.klass, name) {
        object.instance[offset] = value;
        return None;
    }
    let property_size = property_size(object.klass);
    if property_size < 7 {
        object.klass = unsafe { make_hidden_class(vms, object.klass, name) };
        object.instance[property_size] = value;
        return None;
    } else {
        return Some(object_to_hashmap(vms, object, name, value));
    }
}
