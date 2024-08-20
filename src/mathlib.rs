use std::os::raw::c_int;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn add_safe(a: i32, b: i32) -> i32 {
    unsafe { add(a as c_int, b as c_int) }
}

pub fn subtract_safe(a: i32, b: i32) -> i32 {
    unsafe { subtract(a as c_int, b as c_int) }
}

pub fn multiply_safe(a: i32, b: i32) -> i32 {
    unsafe { multiply(a as c_int, b as c_int) }
}

pub fn divide_safe(a: i32, b: i32) -> Option<f64> {
    if b == 0 {
        None
    } else {
        Some(unsafe { divide(a as c_int, b as c_int) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add_safe(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract_safe(5, 3), 2);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply_safe(2, 3), 6);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide_safe(6, 3), Some(2.0));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide_safe(1, 0), None);
    }
}
