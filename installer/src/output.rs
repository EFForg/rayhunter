//! Output handling for the installer
//!
//! This module provides custom print macros that can be intercepted by setting
//! a callback function. This is essential for FFI usage where stdout/stderr
//! redirection doesn't work reliably (especially on Android).

use std::io::Write;
use std::sync::Mutex;

/// Type for the output callback function
type OutputCallbackFn = Box<dyn Fn(&str) + Send + Sync>;

/// Global output callback storage
static OUTPUT_CALLBACK: Mutex<Option<OutputCallbackFn>> = Mutex::new(None);

/// Set the global output callback
///
/// All output from `println!` and `eprintln!` will be sent to this callback.
/// If no callback is set, output goes to stdout/stderr as normal.
///
/// Returns a guard that when dropped, resets the callback.
pub(crate) fn set_output_callback<F>(callback: F) -> OutputCallbackGuard
where
    F: Fn(&str) + Send + Sync + 'static,
{
    *OUTPUT_CALLBACK.lock().unwrap() = Some(Box::new(callback));
    OutputCallbackGuard
}

pub struct OutputCallbackGuard;

impl Drop for OutputCallbackGuard {
    fn drop(&mut self) {
        clear_output_callback();
    }
}

/// Clear the global output callback
pub(crate) fn clear_output_callback() {
    *OUTPUT_CALLBACK.lock().unwrap() = None;
}

/// Write a line to the output (either callback or stdout)
pub(crate) fn write_output_line(s: &str) {
    if let Ok(guard) = OUTPUT_CALLBACK.lock()
        && let Some(ref callback) = *guard
    {
        callback(s);
        callback("\n");
        return;
    }
    // Fallback to stdout if no callback or lock failed
    std::println!("{}", s);
    let _ = std::io::stdout().flush();
}

/// Write an error line to the output (either callback or stderr)
pub(crate) fn write_error_line(s: &str) {
    if let Ok(guard) = OUTPUT_CALLBACK.lock()
        && let Some(ref callback) = *guard
    {
        callback(s);
        callback("\n");
        return;
    }
    // Fallback to stderr if no callback or lock failed
    std::eprintln!("{}", s);
    let _ = std::io::stderr().flush();
}

/// Write raw output without newline (either callback or stdout)
pub(crate) fn write_output_raw(s: &str) {
    if let Ok(guard) = OUTPUT_CALLBACK.lock()
        && let Some(ref callback) = *guard
    {
        callback(s);
        return;
    }
    // Fallback to stdout if no callback or lock failed
    std::print!("{}", s);
    let _ = std::io::stdout().flush();
}

/// Shadow println! macro to respect the output callback
macro_rules! println {
    () => {
        $crate::output::write_output_line("")
    };
    ($($arg:tt)*) => {{
        $crate::output::write_output_line(&format!($($arg)*))
    }};
}
pub(crate) use println;

/// Shadow eprintln! macro to respect the output callback
macro_rules! eprintln {
    () => {
        $crate::output::write_error_line("")
    };
    ($($arg:tt)*) => {{
        $crate::output::write_error_line(&format!($($arg)*))
    }};
}
pub(crate) use eprintln;

/// Shadow print! macro to respect the output callback
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::output::write_output_raw(&format!($($arg)*))
    }};
}
pub(crate) use print;
