use std::ffi::c_char;

use super::request::Request;

#[no_mangle]
pub extern "C" fn get_method<'a>(req: *const Request<'a>) -> *const c_char {
    let req = unsafe { &*req };
    req.method().as_ptr()
}

#[no_mangle]
pub extern "C" fn get_url<'a>(req: *const Request<'a>) -> *const c_char {
    let req = unsafe { &*req };
    req.url().as_ptr()
}

#[no_mangle]
pub extern "C" fn get_http_version<'a>(req: *const Request<'a>) -> u8 {
    let req = unsafe { &*req };
    *req.version() as u8
}
