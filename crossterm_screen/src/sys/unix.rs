use crossterm_utils::Result;
use libc;
use std::fs;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::os::unix::io::AsRawFd;
use termios::{tcsetattr, Termios};

/// This command is used for enabling and disabling raw mode for the terminal.
pub struct RawModeCommand;

impl RawModeCommand {
    pub fn new() -> Self {
        RawModeCommand
    }

    /// Enables raw mode.
    pub fn enable(&mut self) -> Result<()> {
        let tty_f;

        let fd = unsafe {
            if libc::isatty(libc::STDIN_FILENO) == 1 {
                libc::STDIN_FILENO
            } else {
                tty_f = fs::File::open("/dev/tty")?;
                tty_f.as_raw_fd()
            }
        };

        let mut termios = Termios::from_fd(fd)?;
        let original = termios.clone();

        unsafe {
            if ORIGINAL_TERMINAL_MODE.is_none() {
                ORIGINAL_TERMINAL_MODE = Some(original.clone())
            }
        }

        make_raw(&mut termios);
        tcsetattr(fd, TCSADRAIN, &termios)?;

        unsafe { terminal::RAW_MODE_ENABLED_BY_USER = true }
        Ok(())
    }

    /// Disables raw mode.
    pub fn disable(&mut self) -> Result<()> {
        let tty_f;

        let fd = unsafe {
            if libc::isatty(libc::STDIN_FILENO) == 1 {
                libc::STDIN_FILENO
            } else {
                tty_f = fs::File::open("/dev/tty")?;
                tty_f.as_raw_fd()
            }
        };

        if let Some(original) = unsafe { ORIGINAL_TERMINAL_MODE } {
            tcsetattr(fd, TCSADRAIN, &original)?;
        }

        unsafe { terminal::RAW_MODE_ENABLED_BY_USER = false }
        Ok(())
    }
}

static mut ORIGINAL_TERMINAL_MODE: Option<Termios> = None;
static mut RAW_MODE_ENABLED_BY_SYSTEM: bool = false;
pub static mut RAW_MODE_ENABLED_BY_USER: bool = false;

/// Transform the given mode into an raw mode (non-canonical) mode.
pub fn make_raw(termios: &mut Termios) {
    extern "C" {
        pub fn cfmakeraw(termptr: *mut Termios);
    }
    unsafe { cfmakeraw(termios) }
}
