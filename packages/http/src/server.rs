use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream, Shutdown};
use std::io;

use crate::thread::life::WorkerLife;
use crate::thread::ThreadPool;

pub type Callback = extern "C" fn();

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
        )).unwrap();
        listener.set_nonblocking(true).unwrap();

        let pool_capacity = self.pool_capacity;
        let callback = self.callback;
        let life = self.life.clone();
        let main_thread = std::thread::spawn(move || {
            let mut pool = ThreadPool::new(pool_capacity);

            loop {
                match listener.accept() {
                    Ok((stream, addr)) => {
                        pool.execute(connection(callback, &stream, addr));
                        // force to close correctly the stream
                        let _ = stream.shutdown(Shutdown::Both);
                    },
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        if life.is_die() {
                            // stop the main loop and drop the rest as normal
                            break;
                        } else {
                            continue;
                        }
                    },
                    _ => ()
                }
            }
        });

        if self.obstruct {
            let _ = main_thread.join();
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.life.die();
    }
}

fn connection(callback: Callback, stream: &TcpStream, addr: SocketAddr) -> impl FnOnce() {
    move || callback()
}
