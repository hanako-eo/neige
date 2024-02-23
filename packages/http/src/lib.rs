#![allow(clippy::not_unsafe_ptr_arg_deref)]

#[macro_use]
extern crate napi_derive;

pub use server::request_ffi::*;

mod server;
mod thread;
