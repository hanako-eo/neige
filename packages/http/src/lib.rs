#![allow(clippy::not_unsafe_ptr_arg_deref)]

pub use server::ffi::*;
pub use server::request_ffi::*;

mod server;
mod thread;
