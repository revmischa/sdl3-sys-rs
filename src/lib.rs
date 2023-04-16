#![no_std]
#![allow(clippy::approx_constant)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(deref_nullptr)] // bindgen is goofballs
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

#[cfg(feature = "sdl3_mixer")]
pub mod mixer;

#[cfg(feature = "sdl3_image")]
pub mod image;

#[cfg(feature = "sdl3_ttf")]
pub mod ttf;

#[cfg(feature = "sdl3_gfx")]
pub mod gfx;
