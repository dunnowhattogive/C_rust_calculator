use std::os::raw::c_int;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn add(a: i32, b: i32) -> i32 {
    unsafe { add(a as c_int, b as c_int) }
}

pub fn subtract(a: i32, b: i32) -> i32 {
    unsafe { subtract(a as c_int, b as c_int) }
}

pub fn multiply(a: i32, b: i32) -> i32 {
    unsafe { multiply(a as c_int, b as c_int) }
}

pub fn divide(a: i32, b: i32) -> Option<f64> {
    let result = unsafe { divide(a as c_int, b as c_int) };
    if b == 0 {
        None
    } else {
        Some(result)
    }
}
