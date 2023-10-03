use cty::{c_char, c_int};

use std::ffi::CStr;

#[link(name = "rawsock", kind = "static")]
extern "C" {
    fn get_rs(ttl: c_int, rcv_timeout_sec: c_int) -> c_int;
    fn get_err_dscr() -> *const c_char;
    fn close_rs(sock: c_int) -> c_int;
}

fn main() {
    let mut sock: i32 = 0;
    unsafe {
        sock = get_rs(10, 2);
    }
    if sock < 0 {
        let err_msg = unsafe { get_err_dscr() };
        let err_msg: &CStr = unsafe { CStr::from_ptr(err_msg) };
        println!("FAIL: {}", err_msg.to_str().unwrap());
        return;
    }
    println!("My sock = {}", sock);
    unsafe {
        close_rs(sock);
    };
    println!("Hello, from RUST!");
}
