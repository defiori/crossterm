extern crate crossterm_utils;
#[cfg(unix)]
extern crate libc;

mod input;
mod sys;

pub use self::input::{input, AsyncReader, KeyEvent, TerminalInput};
