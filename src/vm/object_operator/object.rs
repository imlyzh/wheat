use crate::vm::{make_object::make_hidden_class, object_model::{HiddenKlass, Object, Slot, Symbol}, vm_state::VMState};

pub fn property_size(object: *const HiddenKlass) -> usize {
    if object.is_null() {
        0
    } else {
        1 + property_size(unsafe { object.as_ref().unwrap().prev })
    }
}

pub fn object_to_hashmap(vms: &mut VMState, object: &mut Object) {

}

pub fn add_property(vms: &mut VMState, object: &mut Object, name: *const Symbol, value: Slot) {
    let property_size = property_size(object.klass);
    if property_size < 7 {
        object.klass = unsafe { make_hidden_class(vms, object.klass, name) };
    } else {
        object_to_hashmap(vms, object);
    }
}
