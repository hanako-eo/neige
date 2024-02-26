use std::cell::RefCell;
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::rc::Rc;

use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::JsFunction;

use crate::thread::life::{WorkerLife, WorkerLifeState};
use crate::thread::ThreadPool;

use self::request::{JsRequest, Request};
use self::socket::Socket;

mod request;
mod socket;

// TODO: use ErrorStrategy::CalleeHandled
type ServerCallback = ThreadsafeFunction<Socket, ErrorStrategy::Fatal>;

#[napi(js_name = "Server")]
pub struct JsServer {
    server: Server,
}

struct Server {
    // Maximum number of Workers that can process requests at the same time
    pool_capacity: u32,
    callback: ServerCallback,
    life: WorkerLife,
}

impl Server {
    fn new(callback: ServerCallback) -> Self {
        Self {
            pool_capacity: 1,
            life: WorkerLife::new(),
            callback,
        }
    }
    fn launch_on(&mut self, port: u16) {
        let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let listener = TcpListener::bind(&local_addr).unwrap();
        listener.set_nonblocking(true).unwrap();

        let pool_capacity = self.pool_capacity;
        let callback = self.callback.clone();
        let life = self.life.clone();

        std::thread::spawn(move || {
            let mut pool = ThreadPool::new(pool_capacity);

            loop {
                match listener.accept() {
                    Ok((stream, addr)) => {
                        let socket = Socket::new(stream, addr, local_addr);
                        let callback = callback.clone();
                        pool.execute(move || {
                            callback.call(socket, ThreadsafeFunctionCallMode::Blocking);
                        });
                    }
                    Err(e) if e.kind() == ErrorKind::WouldBlock => {
                        match life.get() {
                            // stop the main loop and drop the rest as normal
                            WorkerLifeState::Die => break,
                            WorkerLifeState::Life => continue,
                        }
                    }
                    _ => (),
                }
            }
        });
    }
}

#[napi]
impl JsServer {
    #[napi(constructor)]
    pub fn new(callback: JsFunction) -> napi::Result<Self> {
        let server_callback: ServerCallback = callback.create_threadsafe_function(0, |c| {
            let socket = Rc::new(RefCell::new(c.value));
            let request: Request = Request::parse(socket.clone())
                .expect("An error occured in the parsing of the request.");

            Ok(vec![JsRequest::from(request)])
        })?;

        Ok(Self {
            server: Server::new(server_callback),
        })
    }

    #[napi]
    pub fn get_pool_capacity(&self) -> u32 {
        self.server.pool_capacity
    }

    #[napi]
    pub fn set_pool_capacity(&mut self, pool_capacity: u32) {
        self.server.pool_capacity = pool_capacity;
    }

    #[napi]
    pub fn listen(&mut self, port: u16) {
        self.server.launch_on(port);
    }

    #[napi]
    pub fn close(&mut self) {
        self.server.life.die();
    }
}
