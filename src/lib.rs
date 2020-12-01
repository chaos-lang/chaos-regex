mod error;

use error_gen::c_error;
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
#[c_error(false)]
pub unsafe extern "C" fn is_match(re: *mut c_char, text: *mut c_char, cache: bool) -> bool {
    let re = CStr::from_ptr(re).to_str()?;
    let text = CStr::from_ptr(text).to_str()?;
    if cache {
        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache.entry(re.to_owned()).or_insert(Regex::new(re)?);
            Ok(reg.is_match(text))
        })
    } else {
        Ok(Regex::new(re)?.is_match(text))
    }
}

#[no_mangle]
#[c_error(ptr::null_mut())]
pub unsafe extern "C" fn replace(
    re: *mut c_char,
    text: *mut c_char,
    rep: *mut c_char,
    cache: bool,
) -> *mut c_char {
    let re = CStr::from_ptr(re).to_str()?;
    let text = CStr::from_ptr(text).to_str()?;
    let rep = CStr::from_ptr(rep).to_str()?;
    let ret = if cache {
        let ret: anyhow::Result<_> = CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache.entry(re.to_owned()).or_insert(Regex::new(re)?);
            Ok(reg.replace(text, rep))
        });
        ret?
    } else {
        Regex::new(re)?.replace(text, rep)
    };
    Ok(CString::new(ret.as_bytes())?.into_raw())
}

#[no_mangle]
#[c_error(ptr::null_mut())]
pub unsafe extern "C" fn replace_all(
    re: *mut c_char,
    text: *mut c_char,
    rep: *mut c_char,
    cache: bool,
) -> *mut c_char {
    let re = CStr::from_ptr(re).to_str()?;
    let text = CStr::from_ptr(text).to_str()?;
    let rep = CStr::from_ptr(rep).to_str()?;
    let ret = if cache {
        let ret: anyhow::Result<_> = CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache.entry(re.to_owned()).or_insert(Regex::new(re)?);
            Ok(reg.replace_all(text, rep))
        });
        ret?
    } else {
        Regex::new(re)?.replace_all(text, rep)
    };
    Ok(CString::new(ret.as_bytes())?.into_raw())
}

#[no_mangle]
#[c_error(ptr::null_mut())]
pub unsafe extern "C" fn replacen(
    re: *mut c_char,
    text: *mut c_char,
    limit: i64,
    rep: *mut c_char,
    cache: bool,
) -> *mut c_char {
    let re = CStr::from_ptr(re).to_str()?;
    let text = CStr::from_ptr(text).to_str()?;
    let rep = CStr::from_ptr(rep).to_str()?;
    let ret = if cache {
        let ret: anyhow::Result<_> = CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache.entry(re.to_owned()).or_insert(Regex::new(re)?);
            Ok(reg.replacen(text, limit as usize, rep))
        });
        ret?
    } else {
        Regex::new(re)?.replacen(text, limit as usize, rep)
    };
    Ok(CString::new(ret.as_bytes())?.into_raw())
}

#[no_mangle]
#[c_error(0)]
pub unsafe extern "C" fn split(
    re: *mut c_char,
    text: *mut c_char,
    cache: bool,
    list: *mut *mut *mut c_char,
) -> usize {
    let re = CStr::from_ptr(re).to_str()?;
    let text = CStr::from_ptr(text).to_str()?;
    let splits: Vec<_> = if cache {
        let ret: anyhow::Result<_> = CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache.entry(re.to_owned()).or_insert(Regex::new(re)?);
            Ok(reg.split(text).collect())
        });
        ret?
    } else {
        Regex::new(re)?.split(text).collect()
    };
    let mut ret = ManuallyDrop::new(Vec::with_capacity(splits.len()));
    for item in splits {
        ret.push(CString::new(item)?.into_raw());
    }
    *list = ret.as_ptr() as *mut _;
    Ok(ret.len())
}

#[no_mangle]
#[c_error(0)]
pub unsafe extern "C" fn splitn(
    re: *mut c_char,
    text: *mut c_char,
    limit: i64,
    cache: bool,
    list: *mut *mut *mut c_char,
) -> usize {
    let re = CStr::from_ptr(re).to_str()?;
    let text = CStr::from_ptr(text).to_str()?;
    let splits: Vec<_> = if cache {
        let ret: anyhow::Result<_> = CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache.entry(re.to_owned()).or_insert(Regex::new(re)?);
            Ok(reg.splitn(text, limit as usize).collect())
        });
        ret?
    } else {
        Regex::new(re)?.splitn(text, limit as usize).collect()
    };
    let mut ret = ManuallyDrop::new(Vec::with_capacity(splits.len()));
    for item in splits {
        ret.push(CString::new(item)?.into_raw());
    }
    *list = ret.as_ptr() as *mut _;
    Ok(ret.len())
}

#[repr(C)]
pub struct Match {
    pub start: i64,
    pub end: i64,
    pub string: *mut c_char,
}

#[no_mangle]
#[c_error(ptr::null_mut())]
pub unsafe extern "C" fn find(re: *mut c_char, text: *mut c_char, cache: bool) -> *mut Match {
    let re = CStr::from_ptr(re).to_str()?;
    let text = CStr::from_ptr(text).to_str()?;
    let matches = if cache {
        let ret: anyhow::Result<_> = CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache.entry(re.to_owned()).or_insert(Regex::new(re)?);
            Ok(reg.find(text))
        });
        ret?
    } else {
        Regex::new(re)?.find(text)
    };
    if let Some(mat) = matches {
        let mat = Match {
            start: mat.start() as i64,
            end: mat.end() as i64,
            string: CString::new(mat.as_str())?.into_raw(),
        };
        Ok(Box::into_raw(Box::new(mat)))
    } else {
        Ok(ptr::null_mut())
    }
}

#[no_mangle]
#[c_error(0)]
pub unsafe extern "C" fn find_all(
    re: *mut c_char,
    text: *mut c_char,
    cache: bool,
    list: *mut *mut Match,
) -> usize {
    let re = CStr::from_ptr(re).to_str()?;
    let text = CStr::from_ptr(text).to_str()?;
    let matches: Vec<_> = if cache {
        let ret: anyhow::Result<_> = CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache.entry(re.to_owned()).or_insert(Regex::new(re)?);
            Ok(reg.find_iter(text).collect())
        });
        ret?
    } else {
        Regex::new(re)?.find_iter(text).collect()
    };
    let mut ret = ManuallyDrop::new(Vec::with_capacity(matches.len()));
    for mat in matches {
        let mat = Match {
            start: mat.start() as i64,
            end: mat.end() as i64,
            string: CString::new(mat.as_str())?.into_raw(),
        };
        ret.push(mat);
    }
    *list = ret.as_ptr() as *mut _;
    Ok(ret.len())
}

#[no_mangle]
pub unsafe extern "C" fn free_match_prt(ptr: *mut Match) {
    mem::drop(Box::from_raw(ptr))
}

#[no_mangle]
pub unsafe extern "C" fn free_match_list(ptr: *mut Match, len: usize) {
    mem::drop(Vec::from_raw_parts(ptr, len, len));
}

#[repr(C)]
pub struct MatchIdx {
    pub idx: i64,
    pub mat: Match,
}

#[no_mangle]
#[c_error(0)]
pub unsafe extern "C" fn captures(
    re: *mut c_char,
    text: *mut c_char,
    cache: bool,
    list: *mut *mut MatchIdx,
) -> usize {
    let re = CStr::from_ptr(re).to_str()?;
    let text = CStr::from_ptr(text).to_str()?;
    let caps = if cache {
        let ret: anyhow::Result<_> = CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let reg = cache.entry(re.to_owned()).or_insert(Regex::new(re)?);
            Ok(reg.captures(text))
        });
        ret?
    } else {
        Regex::new(re)?.captures(text)
    };
    if let Some(caps) = caps {
        let mut ret = ManuallyDrop::new(Vec::with_capacity(caps.len()));
        for (idx, mat) in caps.iter().enumerate() {
            if let Some(mat) = mat {
                let mat = Match {
                    start: mat.start() as i64,
                    end: mat.end() as i64,
                    string: CString::new(mat.as_str())?.into_raw(),
                };
                ret.push(MatchIdx {
                    mat,
                    idx: idx as i64,
                });
            }
        }
        *list = ret.as_ptr() as *mut _;
        Ok(ret.len())
    } else {
        Ok(0)
    }
}
