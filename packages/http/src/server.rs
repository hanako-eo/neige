use std::io::{self, BufReader};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream};

use napi::threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi::JsFunction;

use crate::thread::life::{WorkerLife, WorkerLifeState};
use crate::thread::ThreadPool;

use self::request::Request;

mod request;

type ServerCallback = ThreadsafeFunction<(), ErrorStrategy::Fatal>;

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
        let listener = TcpListener::bind(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port,
        ))
        .unwrap();
        listener.set_nonblocking(true).unwrap();

        let pool_capacity = self.pool_capacity;
        let callback = self.callback.clone();
        let life = self.life.clone();

        std::thread::spawn(move || {
            let mut pool = ThreadPool::new(pool_capacity);

            loop {
                match listener.accept() {
                    Ok((stream, addr)) => {
                        pool.execute(connection(callback.clone(), stream, addr));
                        // force to close correctly the stream
                        // let _ = stream.shutdown(Shutdown::Both);
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
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
    pub fn new(callback: JsFunction) -> Self {
        // TODO: use ErrorStrategy::CalleeHandled
        let server_callback: ServerCallback = callback
            .create_threadsafe_function(0, |c| {
                // c.
                Ok(vec![()])
            })
            .unwrap();

        Self {
            server: Server::new(server_callback),
        }
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

fn connection(
    callback: ServerCallback,
    stream: TcpStream,
    addr: SocketAddr,
) -> impl FnOnce() + 'static {
    move || {
        let buffer = BufReader::new(&stream);
        let mut request = Request::parse(buffer, &stream)
            .expect("An error occured in the parsing of the request.");
        callback.call((), ThreadsafeFunctionCallMode::Blocking);
        let _ = stream.shutdown(Shutdown::Both);
    }
}
