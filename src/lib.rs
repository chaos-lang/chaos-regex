use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::thread_local;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

thread_local! {
    static CACHE: RefCell<HashMap<String, Regex>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub unsafe extern "C" fn free_str(s: *mut c_char) {
    mem::drop(CString::from_raw(s))
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

#[no_mangle]
pub unsafe extern "C" fn replace(
    re: *mut c_char,
    text: *mut c_char,
    rep: *mut c_char,
    cache: bool,
) -> *mut c_char {
    let re = CStr::from_ptr(re).to_str().unwrap();
    let text = CStr::from_ptr(text).to_str().unwrap();
    let rep = CStr::from_ptr(rep).to_str().unwrap();
    let ret = if cache {
        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache
                .entry(re.to_owned())
                .or_insert(Regex::new(re).unwrap());
            reg.replace(text, rep)
        })
    } else {
        Regex::new(re).unwrap().replace(text, rep)
    };
    CString::new(ret.as_bytes()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn replace_all(
    re: *mut c_char,
    text: *mut c_char,
    rep: *mut c_char,
    cache: bool,
) -> *mut c_char {
    let re = CStr::from_ptr(re).to_str().unwrap();
    let text = CStr::from_ptr(text).to_str().unwrap();
    let rep = CStr::from_ptr(rep).to_str().unwrap();
    let ret = if cache {
        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache
                .entry(re.to_owned())
                .or_insert(Regex::new(re).unwrap());
            reg.replace_all(text, rep)
        })
    } else {
        Regex::new(re).unwrap().replace_all(text, rep)
    };
    CString::new(ret.as_bytes()).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn replacen(
    re: *mut c_char,
    text: *mut c_char,
    limit: i64,
    rep: *mut c_char,
    cache: bool,
) -> *mut c_char {
    let re = CStr::from_ptr(re).to_str().unwrap();
    let text = CStr::from_ptr(text).to_str().unwrap();
    let rep = CStr::from_ptr(rep).to_str().unwrap();
    let ret = if cache {
        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache
                .entry(re.to_owned())
                .or_insert(Regex::new(re).unwrap());
            reg.replacen(text, limit as usize, rep)
        })
    } else {
        Regex::new(re).unwrap().replacen(text, limit as usize, rep)
    };
    CString::new(ret.as_bytes()).unwrap().into_raw()
}
