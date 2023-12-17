use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};

use crate::thread::ThreadPool;

pub type Callback = extern "C" fn();

#[repr(C)]
pub struct ServerBuilder {
    // Maximum number of Workers that can process requests at the same time
    pub(crate) pool_capacity: u32,
    // Boolean used to determine whether the server should block the main process or not
    pub(crate) obstruct: bool,
}

impl ServerBuilder {
    pub(crate) fn new() -> Self {
        Self {
            pool_capacity: 1,
            obstruct: true,
        }
    }

    pub(crate) fn build(self, callback: Callback, port: u16) -> Server {
        Server {
            config: self,
            callback,
            port,
        }
    }
}

pub struct Server {
    callback: Callback,
    config: ServerBuilder,
    port: u16,
}

impl Server {
    pub(crate) fn run(&mut self) {
        let listener = TcpListener::bind(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            self.port,
        ))
        .unwrap();

        let pool_capacity = self.config.pool_capacity;
        let callback = self.callback;
        let main_thread = std::thread::spawn(move || {
            let mut pool = ThreadPool::new(pool_capacity);

            loop {
                let result = listener.accept();

                // Filter failed connection
                if let Ok((stream, addr)) = result {
                    pool.execute(connection(callback, stream, addr));
                }
            }
        });

        if self.config.obstruct {
            let _ = main_thread.join();
        }
    }
}

fn connection(callback: Callback, stream: TcpStream, addr: SocketAddr) -> impl FnOnce() {
    move || callback()
}
