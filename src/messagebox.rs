//! Support for displaying a basic message box to the user.

use super::*;
use crate::{
  prelude::{SDL_GetError, SDL_Init},
  video::SDL_Window,
};

/// If supported will display warning icon, etc.
#[derive(Debug, Clone, Copy, Default)]
#[repr(transparent)]
pub struct SDL_MessageBoxFlags(pub u32);
make_bit_ops_impls_for!(SDL_MessageBoxFlags);

/// error dialog
pub const SDL_MESSAGEBOX_ERROR: SDL_MessageBoxFlags = SDL_MessageBoxFlags(0x00000010);
/// warning dialog
pub const SDL_MESSAGEBOX_WARNING: SDL_MessageBoxFlags = SDL_MessageBoxFlags(0x00000020);
/// informational dialog
pub const SDL_MESSAGEBOX_INFORMATION: SDL_MessageBoxFlags = SDL_MessageBoxFlags(0x00000040);
/// buttons placed left to right
pub const SDL_MESSAGEBOX_BUTTONS_LEFT_TO_RIGHT: SDL_MessageBoxFlags =
  SDL_MessageBoxFlags(0x00000080);
/// buttons placed right to left
pub const SDL_MESSAGEBOX_BUTTONS_RIGHT_TO_LEFT: SDL_MessageBoxFlags =
  SDL_MessageBoxFlags(0x00000100);

extern "C" {
  /// Display a simple modal message box.
  ///
  /// If your needs aren't complex, this function is preferred over
  /// `SDL_ShowMessageBox`.
  ///
  /// `flags` may be any of the following:
  ///
  /// - [SDL_MESSAGEBOX_ERROR]: error dialog
  /// - [SDL_MESSAGEBOX_WARNING]: warning dialog
  /// - [SDL_MESSAGEBOX_INFORMATION]: informational dialog
  ///
  /// This function should be called on the thread that created the parent
  /// window, or on the main thread if the messagebox has no parent. It will
  /// block execution of that thread until the user clicks a button or closes
  /// the messagebox.
  ///
  /// This function may be called at any time, even before [SDL_Init]. This
  /// makes it useful for reporting errors like a failure to create a renderer
  /// or OpenGL context.
  ///
  /// On X11, SDL rolls its own dialog box with X11 primitives instead of a
  /// formal toolkit like GTK+ or Qt.
  ///
  /// Note that if [SDL_Init] would fail because there isn't any available video
  /// target, this function is likely to fail for the same reasons. If this is a
  /// concern, check the return value from this function and fall back to
  /// writing to stderr if you can.
  ///
  /// * `flags`: an [SDL_MessageBoxFlags] value
  /// * `title`: UTF-8 title text
  /// * `message`: UTF-8 message text
  /// * `window`: the parent window, or NULL for no parent
  /// * **Returns:** 0 on success or a negative error code on failure; call
  ///   [SDL_GetError] for more information.
  pub fn SDL_ShowSimpleMessageBox(
    flags: SDL_MessageBoxFlags, title: *const c_char, message: *const c_char,
    window: *mut SDL_Window,
  ) -> int;
}
