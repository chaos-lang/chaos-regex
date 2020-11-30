use regex::Regex;
use std::collections::HashMap;
use std::mem::{self, ManuallyDrop};
use std::thread_local;
use std::{cell::RefCell, ptr};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

thread_local! {
    static CACHE: RefCell<HashMap<String, Regex>> = RefCell::new(HashMap::new());
}

#[no_mangle]
pub unsafe extern "C" fn free_str(ptr: *mut c_char) {
    mem::drop(CString::from_raw(ptr))
}

#[no_mangle]
pub unsafe extern "C" fn free_str_list(ptr: *mut *mut c_char, len: usize) {
    let list = Vec::from_raw_parts(ptr, len, len);
    for item in list {
        mem::drop(CString::from_raw(item));
    }
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

#[no_mangle]
pub unsafe extern "C" fn split(
    re: *mut c_char,
    text: *mut c_char,
    cache: bool,
    list: *mut *mut *mut c_char,
) -> usize {
    let re = CStr::from_ptr(re).to_str().unwrap();
    let text = CStr::from_ptr(text).to_str().unwrap();
    let splits: Vec<_> = if cache {
        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache
                .entry(re.to_owned())
                .or_insert(Regex::new(re).unwrap());
            reg.split(text).collect()
        })
    } else {
        Regex::new(re).unwrap().split(text).collect()
    };
    let mut ret = ManuallyDrop::new(Vec::with_capacity(splits.len()));
    for item in splits {
        ret.push(CString::new(item).unwrap().into_raw());
    }
    *list = ret.as_ptr() as *mut _;
    ret.len()
}

#[no_mangle]
pub unsafe extern "C" fn splitn(
    re: *mut c_char,
    text: *mut c_char,
    limit: i64,
    cache: bool,
    list: *mut *mut *mut c_char,
) -> usize {
    let re = CStr::from_ptr(re).to_str().unwrap();
    let text = CStr::from_ptr(text).to_str().unwrap();
    let splits: Vec<_> = if cache {
        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache
                .entry(re.to_owned())
                .or_insert(Regex::new(re).unwrap());
            reg.splitn(text, limit as usize).collect()
        })
    } else {
        Regex::new(re)
            .unwrap()
            .splitn(text, limit as usize)
            .collect()
    };
    let mut ret = ManuallyDrop::new(Vec::with_capacity(splits.len()));
    for item in splits {
        ret.push(CString::new(item).unwrap().into_raw());
    }
    *list = ret.as_ptr() as *mut _;
    ret.len()
}

#[repr(C)]
pub struct Match {
    pub start: i64,
    pub end: i64,
    pub string: *mut c_char,
}

#[no_mangle]
pub unsafe extern "C" fn find(re: *mut c_char, text: *mut c_char, cache: bool) -> *mut Match {
    let re = CStr::from_ptr(re).to_str().unwrap();
    let text = CStr::from_ptr(text).to_str().unwrap();
    let matches = if cache {
        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache
                .entry(re.to_owned())
                .or_insert(Regex::new(re).unwrap());
            reg.find(text)
        })
    } else {
        Regex::new(re).unwrap().find(text)
    };
    if let Some(mat) = matches {
        let mat = Match {
            start: mat.start() as i64,
            end: mat.end() as i64,
            string: CString::new(mat.as_str()).unwrap().into_raw(),
        };
        Box::into_raw(Box::new(mat))
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn find_all(
    re: *mut c_char,
    text: *mut c_char,
    cache: bool,
    list: *mut *mut Match,
) -> usize {
    let re = CStr::from_ptr(re).to_str().unwrap();
    let text = CStr::from_ptr(text).to_str().unwrap();
    let matches: Vec<_> = if cache {
        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache
                .entry(re.to_owned())
                .or_insert(Regex::new(re).unwrap());
            reg.find_iter(text).collect()
        })
    } else {
        Regex::new(re).unwrap().find_iter(text).collect()
    };
    let mut ret = ManuallyDrop::new(Vec::with_capacity(matches.len()));
    for mat in matches {
        let mat = Match {
            start: mat.start() as i64,
            end: mat.end() as i64,
            string: CString::new(mat.as_str()).unwrap().into_raw(),
        };
        ret.push(mat);
    }
    *list = ret.as_ptr() as *mut _;
    ret.len()
}

#[no_mangle]
pub unsafe extern "C" fn free_match_prt(ptr: *mut Match) {
    mem::drop(Box::from_raw(ptr))
}

#[no_mangle]
pub unsafe extern "C" fn free_match_list(ptr: *mut Match, len: usize) {
    mem::drop(Vec::from_raw_parts(ptr, len, len));
}
