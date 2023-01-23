//! Simple error message routines for SDL.

use super::*;

extern "C" {
  /// Retrieve a message about the last error that occurred on the current
  /// thread.
  ///
  /// It is possible for multiple errors to occur before calling [SDL_GetError].
  /// Only the last error is returned.
  ///
  /// The message is only applicable when an SDL function has signaled an error.
  /// You must check the return values of SDL function calls to determine when
  /// to appropriately call [SDL_GetError]. You should *not* use the results of
  /// [SDL_GetError] to decide if an error has occurred! Sometimes SDL will set
  /// an error string even when reporting success.
  ///
  /// SDL will *not* clear the error string for successful API calls. You *must*
  /// check return values for failure cases before you can assume the error
  /// string applies.
  ///
  /// Error strings are set per-thread, so an error set in a different thread
  /// will not interfere with the current thread's operation.
  ///
  /// The returned string is internally allocated and must not be freed by the
  /// application.
  ///
  /// * **Returns:** a message with information about the specific error that
  ///   occurred, or an empty string if there hasn't been an error message set
  ///   since the last call to [SDL_ClearError]. The message is only applicable
  ///   when an SDL function has signaled an error. You must check the return
  ///   values of SDL function calls to determine when to appropriately call
  ///   [SDL_GetError].
  pub fn SDL_GetError() -> *const c_char;

  /// Get the last error message that was set for the current thread.
  ///
  /// This allows the caller to copy the error string into a provided buffer,
  /// but otherwise operates exactly the same as [SDL_GetError].
  ///
  /// * `errstr`: A buffer to fill with the last error message that was set for
  ///   the current thread
  /// * `maxlen`: The size of the buffer pointed to by the `errstr` parameter
  /// * **Returns:** the pointer passed in as the `errstr` parameter.
  pub fn SDL_GetErrorMsg(errstr: *mut c_char, maxlen: int) -> *mut c_char;

  /// Clear any previous error message for this thread.
  pub fn SDL_ClearError();
}
