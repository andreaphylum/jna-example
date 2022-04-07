use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::{null_mut, null};

pub struct RustData {
    package_id: CString,
    oid: CString,
    var_names: Vec<CString>,
    var_names_idx: usize,
}


impl RustData {
    fn new() -> Self {
        Self {
            package_id: CString::new("react").unwrap(),
            oid: CString::new("12345").unwrap(),
            var_names: vec![
                CString::new("fooooo").unwrap(),
                CString::new("barrrr").unwrap(),
                CString::new("bazzzz").unwrap(),
            ],
            var_names_idx: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_data_new() -> *const RustData {
    println!("\x1b[32;1mReturning new instance\x1b[0m");
    let rust_data = Box::new(RustData::new());
    Box::into_raw(rust_data)
}

#[no_mangle]
pub extern "C" fn rust_data_package_id(this: &mut RustData) -> *const c_char {
    println!("\x1b[32;1mReturning package id\x1b[0m");
    this.package_id.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn rust_data_var_names_next(this: *mut RustData) -> *const c_char {
    println!("\x1b[32;1mReturning next string\x1b[0m");
    if let Some(this) = this.as_mut() {
        if this.var_names_idx < this.var_names.len() {
            let ptr = this.var_names[this.var_names_idx].as_ptr();
            this.var_names_idx += 1;
            ptr
        } else {
            null()
        }
    } else {
        null_mut()
    }
}

#[no_mangle]
pub extern "C" fn rust_data_drop(rust_data: *mut RustData) {
    println!("\x1b[32;1mDropping\x1b[0m");
    drop(unsafe { Box::from_raw(rust_data) });
}
