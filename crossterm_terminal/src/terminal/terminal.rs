//! A module that contains all the actions related to the terminal.
//! Like clearing and scrolling in the terminal or getting the window size from the terminal.

use super::{ClearType, ITerminal, AnsiTerminal};
use crossterm_utils::{write, Result, TerminalOutput};

#[cfg(windows)]
use crossterm_utils::get_module;

use std::fmt;
use std::sync::Arc;

/// Struct that stores a platform-specific platform implementation for terminal related actions.
///
/// Check `/examples/terminal` in the library for more specific examples.
///
/// ```rust
/// use crossterm::terminal;
///
/// let term = terminal();
///
/// term.scroll_down(5);
/// term.scroll_up(4);
/// let (with, height) = term.terminal_size();
/// ```
/// When you want to use 'terminal' related actions on 'alternate screen' use the `Screen` type instead, and pass it to the `terminal::from_screen()` function.
/// By doing that terminal actions will be performed on the alternate screen.
pub struct Terminal<'stdout> {
    terminal: Box<ITerminal + Sync + Send>,
    stdout: Option<&'stdout Arc<TerminalOutput>>,
}

impl<'stdout> Terminal<'stdout> {
    /// Create new terminal instance whereon terminal related actions can be performed.
    pub fn new() -> Terminal<'stdout> {
        #[cfg(target_os = "windows")]
        let terminal = get_module::<Box<ITerminal + Sync + Send>>(
            Box::new(WinApiTerminal::new()),
            Box::new(AnsiTerminal::new()),
        )
        .unwrap();

        #[cfg(not(target_os = "windows"))]
        let terminal = Box::from(AnsiTerminal::new()) as Box<ITerminal + Sync + Send>;

        Terminal {
            terminal,
            stdout: None,
        }
    }

    /// Create a new instance of `Terminal` whereon terminal related actions could be preformed on the given output.
    ///
    /// **Note**
    ///
    /// Use this function when you want your terminal to operate with a specific output.
    /// This could be useful when you have a screen which is in 'alternate mode'.
    /// And you want your actions from the `Terminal`, created by this function, to operate on the 'alternate screen'.
    ///
    /// # Example
    /// ```
    /// let screen = Screen::default();
    //
    /// if let Ok(alternate) = screen.enable_alternate_modes(false) {
    ///    let terminal = Terminal::from_output(&alternate.screen.stdout);
    /// }
    /// ```
    pub fn from_output(stdout: &'stdout Arc<TerminalOutput>) -> Terminal<'stdout> {
        #[cfg(target_os = "windows")]
        let terminal = get_module::<Box<ITerminal + Sync + Send>>(
            Box::new(WinApiTerminal::new()),
            Box::new(AnsiTerminal::new()),
        )
        .unwrap();

        #[cfg(not(target_os = "windows"))]
        let terminal = Box::from(AnsiTerminal::new()) as Box<ITerminal + Sync + Send>;

        Terminal {
            terminal,
            stdout: Some(stdout),
        }
    }

    /// Clear the current cursor by specifying the clear type.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// // clear all cells in terminal.
    /// term.clear(terminal::ClearType::All);
    /// // clear all cells from the cursor position downwards in terminal.
    /// term.clear(terminal::ClearType::FromCursorDown);
    /// // clear all cells from the cursor position upwards in terminal.
    /// term.clear(terminal::ClearType::FromCursorUp);
    /// // clear current line cells in terminal.
    /// term.clear(terminal::ClearType::CurrentLine);
    /// // clear all cells from cursor position until new line in terminal.
    /// term.clear(terminal::ClearType::UntilNewLine);
    /// ```
    pub fn clear(&self, clear_type: ClearType) -> Result<()> {
        self.terminal.clear(clear_type, &self.stdout)
    }

    /// Get the terminal size (x,y).
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// let size = term.terminal_size();
    /// println!("{:?}", size);
    /// ```
    pub fn terminal_size(&self) -> (u16, u16) {
        self.terminal.terminal_size(&self.stdout)
    }

    /// Scroll `n` lines up in the current terminal.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// // scroll up by 5 lines
    /// let size = term.scroll_up(5);
    /// ```
    pub fn scroll_up(&self, count: i16) -> Result<()> {
        self.terminal.scroll_up(count, &self.stdout)
    }

    /// Scroll `n` lines up in the current terminal.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// // scroll down by 5 lines
    /// let size = term.scroll_down(5);
    /// ```
    pub fn scroll_down(&self, count: i16) -> Result<()> {
        self.terminal.scroll_down(count, &self.stdout)
    }

    /// Set the terminal size. Note that not all terminals can be set to a very small scale.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// // Set of the size to X: 10 and Y: 10
    /// let size = term.set_size(10,10);
    /// ```
    pub fn set_size(&self, width: i16, height: i16) -> Result<()> {
        self.terminal.set_size(width, height, &self.stdout)
    }

    /// Exit the current process.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// let size = term.exit();
    /// ```
    pub fn exit(&self) {
        crate::sys::exit();
    }

    /// Write any displayable content to the current terminal screen.
    ///
    /// ```rust
    /// let mut term = terminal();
    ///
    /// let size = term.write("Some text \n Some text on new line");
    /// ```
    pub fn write<D: fmt::Display>(&self, value: D) -> Result<usize> {
        use std::fmt::Write;
        let mut string = String::new();
        write!(string, "{}", value)?;
        let size = write(&self.stdout, string)?;
        Ok(size)
    }
}

/// Get a `Terminal` instance whereon terminal related actions could performed.
pub fn terminal<'stdout>() -> Terminal<'stdout> {
    Terminal::new()
}
