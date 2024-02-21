use super::{Callback, Server};

#[no_mangle]
pub extern "C" fn create_server(callback: Callback) -> *const Server {
    let server = Box::new(Server::new(callback));
    Box::leak(server)
}

#[no_mangle]
pub extern "C" fn get_pool_capacity(server: *const Server) -> u32 {
    // SAFETY: the pointer of server is define in the javascript code
    let server = unsafe { &*server };
    server.pool_capacity
}

#[no_mangle]
pub extern "C" fn set_pool_capacity(server: *mut Server, pool_capacity: u32) {
    // SAFETY: the pointer of server is define in the javascript code
    let server = unsafe { &mut *server };
    server.pool_capacity = pool_capacity;
}

#[no_mangle]
pub extern "C" fn get_obstruction(server: *const Server) -> bool {
    // SAFETY: the pointer of server is define in the javascript code
    let server = unsafe { &*server };
    server.obstruct
}

#[no_mangle]
pub extern "C" fn set_obstruction(server: *mut Server, obstruct: bool) {
    // SAFETY: the pointer of server is define in the javascript code
    let server = unsafe { &mut *server };
    server.obstruct = obstruct;
}

#[no_mangle]
pub extern "C" fn launch_server(server: *mut Server, port: u16) {
    // SAFETY: the pointer of server is define in the javascript code
    let server = unsafe { &mut *server };
    server.launch_on(port);
}

#[no_mangle]
pub extern "C" fn close_server(server: *mut Server) {
    // SAFETY: the pointer of server is define in the javascript code
    let _ = unsafe { Box::from_raw(server) };
}
