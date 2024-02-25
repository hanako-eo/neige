use std::io::BufReader;
use std::mem::transmute;
use std::net::{Shutdown, SocketAddr, TcpStream};

use napi::bindgen_prelude::SharedReference;
use napi::{Env, JsObject};

use super::request::JsRequest;

#[napi(js_name = "Socket")]
pub struct JsSocket {
    pub(super) inner: SharedReference<JsRequest, &'static Socket>,
}

pub struct Socket {
    stream: TcpStream,
    remote_addr: SocketAddr,
    local_addr: SocketAddr,
    buffer: Option<BufReader<&'static TcpStream>>,
}

impl Socket {
    pub fn new(stream: TcpStream, remote_addr: SocketAddr, local_addr: SocketAddr) -> Self {
        Self {
            stream,
            remote_addr,
            local_addr,
            buffer: None,
        }
    }

    pub fn read_buf(&mut self) -> &mut BufReader<&'static TcpStream> {
        match &self.buffer {
            None => {
                self.buffer = Some(unsafe {
                    // FIXME: temporary
                    transmute::<BufReader<&TcpStream>, BufReader<&'static TcpStream>>(
                        BufReader::new(&self.stream),
                    )
                });
            }
            _ => (),
        }
        self.buffer.as_mut().unwrap()
    }

    pub fn close(&self) {
        let _ = self.stream.shutdown(Shutdown::Both);
    }
}

#[napi]
impl JsSocket {
    #[napi(constructor)]
    pub fn new(env: Env) -> napi::Result<Self> {
        Err(unsafe {
            env.throw(env.create_string("The socket cannot be built from 0.")?)
                .unwrap_err_unchecked()
        })
    }

    #[napi(getter)]
    pub fn remote_addr(&self, env: Env) -> napi::Result<JsObject> {
        let mut obj = env.create_object()?;
        obj.set("address", self.inner.remote_addr.ip().to_string())?;
        obj.set(
            "family",
            match self.inner.remote_addr {
                SocketAddr::V4(_) => "IPv4",
                SocketAddr::V6(_) => "IPv6",
            },
        )?;
        obj.set("port", self.inner.remote_addr.port())?;
        Ok(obj)
    }

    #[napi(getter)]
    pub fn local_addr(&self, env: Env) -> napi::Result<JsObject> {
        let mut obj = env.create_object()?;
        obj.set("address", self.inner.local_addr.ip().to_string())?;
        obj.set(
            "family",
            match self.inner.local_addr {
                SocketAddr::V4(_) => "IPv4",
                SocketAddr::V6(_) => "IPv6",
            },
        )?;
        obj.set("port", self.inner.local_addr.port())?;
        Ok(obj)
    }
}
