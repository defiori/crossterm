//! A module that contains all the actions related to the styling of the terminal.
//! Like applying attributes to font and changing the foreground and background.

use std::io;

use super::*;
use crate::{Color, ITerminalColor};
use crossterm_utils::{Result, TerminalOutput};

#[cfg(windows)]
use crossterm_utils::get_module;

use std::sync::Arc;

/// Struct that stores a platform-specific implementation for color related actions.
///
/// For styling text use the `::crossterm::style()` function. `TerminalColor` will set the colors of the screen permanently and the `style()` will only style the text given.
///
/// Check `/examples/style` in the library for more specific examples.
///
/// # Remarks
/// 
/// When you want to use 'color' on 'alternate screen' use 'crossterm_screen' crate. Which allowes you to style the alternate screen.
pub struct TerminalColor<'stdout> {
    color: Box<ITerminalColor + Sync + Send>,
    stdout: Option<&'stdout Arc<TerminalOutput>>,
}

impl<'stdout> TerminalColor<'stdout> {
    /// Create new instance whereon color related actions can be performed.
    pub fn new() -> TerminalColor<'stdout> {
        #[cfg(target_os = "windows")]
        let color = get_module::<Box<ITerminalColor + Sync + Send>>(
            Box::from(WinApiColor::new()),
            Box::from(AnsiColor::new()),
        )
        .expect("could not extract module");

        #[cfg(not(target_os = "windows"))]
        let color = Box::from(AnsiColor::new()) as Box<ITerminalColor + Sync + Send>;

        TerminalColor {
            color,
            stdout: None,
        }
    }

    /// Create a new instance of `TerminalColor` whereon coloring could be preformed on the given output.
    ///
    /// **Note**
    ///
    /// Use this function when you want your terminal to operate with a specific output.
    /// This could be useful when you have a output which is in 'alternate mode', 
    /// and you want your actions from the `TerminalColor`, created by this function, to operate on the 'alternate screen'.
    ///
    /// - checkout `crossterm_screen` crate.
    /// 
    /// # Example
    /// ```
    /// let screen = Screen::default();
    //
    /// if let Ok(alternate) = screen.enable_alternate_modes(false) {
    ///    let terminal = TerminalColor::from_output(&alternate.screen.stdout);
    /// }
    /// ```
    pub fn from_output(stdout: &'stdout Arc<TerminalOutput>) -> TerminalColor<'stdout> {
        #[cfg(target_os = "windows")]
        let color = get_module::<Box<ITerminalColor + Sync + Send>>(
            Box::from(WinApiColor::new()),
            Box::from(AnsiColor::new()),
        )
        .unwrap();

        #[cfg(not(target_os = "windows"))]
        let color = Box::from(AnsiColor::new()) as Box<ITerminalColor + Sync + Send>;

        TerminalColor {
            color,
            stdout: Some(stdout),
        }
    }

    /// Set the foreground color to the given color.
    pub fn set_fg(&self, color: Color) -> Result<()> {
        self.color.set_fg(color, &self.stdout)
    }

    /// Set the background color to the given color.
    pub fn set_bg(&self, color: Color) -> Result<()> {
        self.color.set_bg(color, &self.stdout)
    }

    /// Reset the terminal colors and attributes to default.
    pub fn reset(&self) -> Result<()> {
        self.color.reset(&self.stdout)
    }

    /// Get available color count.
    pub fn get_available_color_count(&self) -> io::Result<u16> {
        use std::env;

        Ok(match env::var_os("TERM") {
            Some(val) => {
                if val.to_str().unwrap_or("").contains("256color") {
                    256
                } else {
                    8
                }
            }
            None => 8,
        })
    }
}

/// Get a `TerminalColor` implementation whereon color related actions can be performed.
pub fn color<'stdout>() -> TerminalColor<'stdout> {
    TerminalColor::new()
}
