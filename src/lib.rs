#![no_std]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![cfg_attr(feature = "const_trait_impl", feature(const_trait_impl))]

//! Rust bindings to version 3 of the [SDL library](https://github.com/libsdl-org/SDL).

type Uint32 = u32;
use core::ffi::{c_int as int, *};

#[macro_use]
mod macros;

pub mod error;
pub mod init;
pub mod messagebox;
pub mod prelude;
pub mod video;
