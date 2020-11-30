use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::thread_local;
use std::{ffi::CStr, os::raw::c_char};

thread_local! {
    static CACHE: RefCell<HashMap<String, Regex>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub unsafe extern "C" fn is_match(re: *mut c_char, text: *mut c_char, cache: bool) -> bool {
    let re = CStr::from_ptr(re).to_str().unwrap();
    let text = CStr::from_ptr(text).to_str().unwrap();
    if cache {
        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache
                .entry(re.to_owned())
                .or_insert(Regex::new(re).unwrap());
            reg.is_match(text)
        })
    } else {
        Regex::new(re).unwrap().is_match(text)
    }
}
