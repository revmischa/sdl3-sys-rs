#![no_std]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![cfg_attr(feature = "const_trait_impl", feature(const_trait_impl))]
// suppress warnings about u128 (that we don't even use ourselves anyway)
#![allow(improper_ctypes)]

//! Rust bindings to version 3 of the [SDL library](https://github.com/libsdl-org/SDL).

use core::ffi::{c_int as int, *};

#[macro_use]
mod macros;

pub mod prelude;

include!(concat!(env!("OUT_DIR"), "/sdl_bindings.rs"));

#[cfg(feature = "mixer")]
pub mod mixer;

#[cfg(feature = "image")]
pub mod image;

#[cfg(feature = "ttf")]
pub mod ttf;

#[cfg(feature = "gfx")]
pub mod gfx;
