use std::os::raw::{c_char, c_uint};
use std::ffi::CString;

extern {
    fn count_string(str: *const c_char) -> c_uint;
}

fn safe_count_string(str: &str) -> Option<u32> {
    let c_string = match CString::new(str) {
        Ok(c) => c,
        Err(_) => return None
    };

    unsafe {
        Some(count_string(c_string.as_ptr()))
    }
}

fn main() {
    let c_string = CString::new("A string that can be passed to C").expect("meep");
    let count = unsafe {
        count_string(c_string.as_ptr())
    };
    println!("c_string's length is {}", count);

    let count = safe_count_string("12345").expect("goot");
    println!("c_string's safe length is {}", count);
}
