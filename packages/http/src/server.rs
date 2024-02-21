use std::io::{self, BufReader};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream};

use crate::thread::life::{WorkerLife, WorkerLifeState};
use crate::thread::ThreadPool;

use self::request::Request;

pub mod ffi;
mod request;

pub type Callback = for<'a> extern "C" fn(*mut Request<'a>);

#[repr(C)]
pub struct Server {
    // Maximum number of Workers that can process requests at the same time
    pub(crate) pool_capacity: u32,
    // Boolean used to determine whether the server should block the main process or not
    pub(crate) obstruct: bool,
    callback: Callback,
    pub(crate) life: WorkerLife,
}

impl Server {
    pub(crate) fn new(callback: Callback) -> Self {
        Self {
            pool_capacity: 1,
            obstruct: false,
            life: WorkerLife::new(),
            callback,
        }
    }
    pub(crate) fn launch_on(&mut self, port: u16) {
        let listener = TcpListener::bind(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port,
        ))
        .unwrap();
        listener.set_nonblocking(true).unwrap();

        let pool_capacity = self.pool_capacity;
        let callback = self.callback;
        let life = self.life.clone();

        let serve = move || {
            let mut pool = ThreadPool::new(pool_capacity);

            loop {
                match listener.accept() {
                    Ok((stream, addr)) => {
                        pool.execute(connection(callback, stream, addr));
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
        };

        if self.obstruct {
            serve();
        } else {
            std::thread::spawn(serve);
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.life.die();
    }
}

fn connection(callback: Callback, stream: TcpStream, addr: SocketAddr) -> impl FnOnce() + 'static {
    move || {
        let buffer = BufReader::new(&stream);
        let mut request = Box::new(
            Request::parse(buffer, &stream)
                .expect("An error occured in the parsing of the request."),
        );
        callback(request.as_mut());
        let _ = stream.shutdown(Shutdown::Both);
    }
}
