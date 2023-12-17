mod server;
mod thread;
mod owner;

#[no_mangle]
pub extern "C" fn server_builder() -> server::ServerBuilder {
    server::ServerBuilder::new()
}

#[no_mangle]
pub extern "C" fn launch_server(callback: server::Callback, builder: server::ServerBuilder, port: u16) {
    let mut server = builder.build(callback, port);
    server.run();
}
