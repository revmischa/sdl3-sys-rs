//! Init and quit header for the SDL library.

use super::*;
use crate::prelude::{SDL_GetError, SDL_ShowSimpleMessageBox};

pub const SDL_INIT_TIMER: u32 = 0x00000001;

pub const SDL_INIT_AUDIO: u32 = 0x00000010;

/// [SDL_INIT_VIDEO] implies [SDL_INIT_EVENTS]
pub const SDL_INIT_VIDEO: u32 = 0x00000020;

/// [SDL_INIT_JOYSTICK] implies [SDL_INIT_EVENTS]
pub const SDL_INIT_JOYSTICK: u32 = 0x00000200;

pub const SDL_INIT_HAPTIC: u32 = 0x00001000;

/// [SDL_INIT_GAMEPAD] implies [SDL_INIT_JOYSTICK]
pub const SDL_INIT_GAMEPAD: u32 = 0x00002000;

pub const SDL_INIT_EVENTS: u32 = 0x00004000;

pub const SDL_INIT_SENSOR: u32 = 0x00008000;

pub const SDL_INIT_EVERYTHING: u32 = SDL_INIT_TIMER
  | SDL_INIT_AUDIO
  | SDL_INIT_VIDEO
  | SDL_INIT_EVENTS
  | SDL_INIT_JOYSTICK
  | SDL_INIT_HAPTIC
  | SDL_INIT_GAMEPAD
  | SDL_INIT_SENSOR;

extern "C" {
  /// Initialize the SDL library.
  ///
  /// [SDL_Init] simply forwards to calling [SDL_InitSubSystem]. Therefore, the
  /// two may be used interchangeably. Though for readability of your code
  /// [SDL_InitSubSystem] might be preferred.
  ///
  /// The file I/O (for example: SDL_RWFromFile) and threading
  /// (SDL_CreateThread) subsystems are initialized by default. Message boxes
  /// [SDL_ShowSimpleMessageBox] also attempt to work without initializing the
  /// video subsystem, in hopes of being useful in showing an error dialog when
  /// [SDL_Init] fails. You must specifically initialize other subsystems if you
  /// use them in your application.
  ///
  /// Logging (such as SDL_Log) works without initialization, too.
  ///
  /// `flags` may be any of the following OR'd together:
  ///
  /// * [SDL_INIT_TIMER]: timer subsystem
  /// * [SDL_INIT_AUDIO]: audio subsystem
  /// * [SDL_INIT_VIDEO]: video subsystem; automatically initializes the events
  ///   subsystem
  /// * [SDL_INIT_JOYSTICK]: joystick subsystem; automatically initializes the
  ///   events subsystem
  /// * [SDL_INIT_HAPTIC]: haptic (force feedback) subsystem
  /// * [SDL_INIT_GAMEPAD]: gamepad subsystem; automatically initializes the
  ///   joystick subsystem
  /// * [SDL_INIT_EVENTS]: events subsystem
  /// * [SDL_INIT_EVERYTHING]: all of the above subsystems
  ///
  /// Subsystem initialization is ref-counted, you must call [SDL_QuitSubSystem]
  /// for each [SDL_InitSubSystem] to correctly shutdown a subsystem manually
  /// (or call [SDL_Quit] to force shutdown). If a subsystem is already loaded
  /// then this call will increase the ref-count and return.
  ///
  /// * `flags`: subsystem initialization flags
  /// * **Returns:** 0 on success or a negative error code on failure; call
  ///   [SDL_GetError] for more information.
  pub fn SDL_Init(flags: Uint32) -> int;

  /// Compatibility function to initialize the SDL library.
  ///
  /// This function and [SDL_Init] are interchangeable.
  ///
  /// * `flags`: any of the flags used by SDL_Init(); see SDL_Init for details.
  /// * **Returns:** 0 on success or a negative error code on failure; call
  ///   [SDL_GetError] for more information.
  pub fn SDL_InitSubSystem(flags: Uint32) -> int;

  /// Shut down specific SDL subsystems.
  ///
  /// You still need to call [SDL_Quit] even if you close all open subsystems
  /// with [SDL_QuitSubSystem].
  ///
  /// * `flags`: any of the flags used by SDL_Init(); see SDL_Init for details.
  pub fn SDL_QuitSubSystem(flags: Uint32);

  /// Get a mask of the specified subsystems which are currently initialized.
  ///
  /// * `flags`: any of the flags used by SDL_Init(); see SDL_Init for details.
  /// * **Returns:** a mask of all initialized subsystems if `flags` is 0,
  ///   otherwise it returns the initialization status of the specified
  ///   subsystems.
  pub fn SDL_WasInit(flags: Uint32) -> Uint32;

  /// Clean up all initialized subsystems.
  ///
  /// You should call this function even if you have already shutdown each
  /// initialized subsystem with [SDL_QuitSubSystem]. It is safe to call this
  /// function even in the case of errors in initialization.
  pub fn SDL_Quit();
}
