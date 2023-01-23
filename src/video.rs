//! Header file for SDL video functions.

use super::*;

/// The type used to identify a window.
#[repr(transparent)]
pub struct SDL_Window(c_void);
