use core::slice::memchr::memchr;
use std::io::{BorrowedBuf, Read, Result};
use std::mem::MaybeUninit;
use std::net::{SocketAddr, TcpStream};
use std::ops::Deref;

use napi::{bindgen_prelude::SharedReference, Env, JsObject};

use super::request::JsRequest;

const BUFFER_SIZE: usize = 8 * 1024;

#[napi(js_name = "Socket")]
pub struct JsSocket {
    pub(super) inner: SharedReference<JsRequest, &'static Socket>,
}

pub struct Socket {
    stream: TcpStream,
    remote_addr: SocketAddr,
    local_addr: SocketAddr,

    buffer: Box<[MaybeUninit<u8>]>,
    filled: usize,
    pos: usize,
    initialized: usize,
}

pub struct SocketLines<'b> {
    buf: &'b mut Socket,
}

impl Socket {
    pub fn new(stream: TcpStream, remote_addr: SocketAddr, local_addr: SocketAddr) -> Self {
        Self::with_capacity(BUFFER_SIZE, stream, remote_addr, local_addr)
    }

    pub fn with_capacity(
        capacity: usize,
        stream: TcpStream,
        remote_addr: SocketAddr,
        local_addr: SocketAddr,
    ) -> Self {
        Self {
            stream,
            remote_addr,
            local_addr,
            buffer: Box::new_uninit_slice(capacity),
            filled: 0,
            pos: 0,
            initialized: 0,
        }
    }

    #[inline]
    pub fn buffer(&self) -> &[u8] {
        // SAFETY: self.pos and self.cap are valid, and self.cap => self.pos, and
        // that region is initialized because those are all invariants of this type.
        unsafe {
            MaybeUninit::slice_assume_init_ref(self.buffer.get_unchecked(self.pos..self.filled))
        }
    }

    pub fn fill_buf(&mut self) -> Result<&[u8]> {
        if self.pos >= self.filled {
            let mut buf = BorrowedBuf::from(self.buffer.as_mut());
            // SAFETY: `self.filled` bytes will always have been initialized.
            unsafe {
                buf.set_init(self.initialized);
            }

            self.stream.read_buf(buf.unfilled())?;

            self.pos = 0;
            self.filled = buf.len();
            self.initialized = buf.init_len();
        }
        Ok(self.buffer())
    }

    pub fn clear(&mut self) {
        self.filled = 0;
    }

    pub fn read_line(&mut self, line: &mut String) -> Result<usize> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut len = 0;

        loop {
            let buf = self.fill_buf()?;
            let (done, i) = match memchr(b'\n', buf) {
                Some(i) => (true, i + 1),
                None => (false, buf.len()),
            };
            bytes.extend_from_slice(&buf[..i]);
            self.clear();
            len += i;
            if done {
                line.push_str(String::from_utf8_lossy(bytes.as_slice()).deref());
                return Ok(len);
            }
        }
    }

    pub fn lines(&mut self) -> SocketLines<'_> {
        SocketLines { buf: self }
    }
}

impl Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl<'b> Iterator for SocketLines<'b> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with('\n') {
                    buf.pop();
                    if buf.ends_with('\r') {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

#[napi]
impl JsSocket {
    #[napi]
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

    #[napi]
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
