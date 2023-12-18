#![allow(clippy::not_unsafe_ptr_arg_deref)]

mod server;
mod thread;

#[no_mangle]
pub extern "C" fn create_server(callback: server::Callback) -> *const server::Server {
    // panic!("aaaaaaa0");
    let server = Box::new(server::Server::new(callback));
    Box::leak(server)
}

#[no_mangle]
pub extern "C" fn launch_server(server: *mut server::Server, port: u16) {
    // SAFETY: the pointer of server is define in the javascript code
    let server = unsafe { &mut *server };
    server.launch_on(port);
}

#[no_mangle]
pub extern "C" fn close_server(server: *mut server::Server) {
    // SAFETY: the pointer of server is define in the javascript code
    let _ = unsafe { Box::from_raw(server) };
}
