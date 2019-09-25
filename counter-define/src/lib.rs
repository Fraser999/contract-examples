#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

extern crate contract_ffi;
use contract_ffi::contract_api::pointers::TURef;
use contract_ffi::contract_api::*;
use contract_ffi::key::Key;

#[no_mangle]
pub extern "C" fn counter_ext() {
    let i_key: TURef<i32> = get_uref("count").and_then(Key::to_turef).unwrap();
    let method_name: String = match get_arg(0) {
        Some(Ok(name)) => name,
        Some(Err(_)) => revert(Error::InvalidArgument.into()),
        None => revert(Error::MissingArgument.into()),
    };
    match method_name.as_str() {
        "inc" => add(i_key, 1),
        "get" => {
            let result = read(i_key)
                .unwrap_or_else(|_| revert(Error::Read.into()))
                .unwrap_or_else(|| revert(Error::ValueNotFound.into()));
            ret(&result, &Vec::new());
        }
        _ => panic!("Unknown method name!"),
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let counter_local_key = new_turef(0); //initialize counter

    //create map of references for stored contract
    let mut counter_urefs: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from("count");
    counter_urefs.insert(key_name, counter_local_key.into());

    let pointer = store_function("counter_ext", counter_urefs);
    add_uref("counter", &pointer.into());
}
