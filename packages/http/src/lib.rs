#![feature(core_io_borrowed_buf)]
#![feature(maybe_uninit_slice)]
#![feature(new_uninit)]
#![feature(read_buf)]
#![feature(slice_internals)]

#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(internal_features)]

#[macro_use]
extern crate napi_derive;

mod server;
mod thread;
