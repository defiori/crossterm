use crossterm_utils::TerminalOutput;

use std::sync::Arc;

/// This type could be used to access the `cursor, terminal, color, input, styling` module more easily.
/// You need to pass a reference to the screen whereon you want to perform the actions to the `Crossterm` type.
///
/// If you want to use the default screen you could do it like this:
///
/// ```rust
/// extern crate crossterm;
/// use crossterm::{Crossterm, Screen};
///
/// let crossterm = Crossterm::new(&Screen::default());
/// let cursor = crossterm.cursor();
/// ```
///
/// If you want to perform actions on the `AlternateScreen` make sure to pass a reference to the screen of the `AlternateScreen`.
///
/// ```
/// extern crate crossterm;
/// use crossterm::{Crossterm, Screen};
///
/// let main_screen = Screen::default();
///
/// if let Ok(alternate_srceen) = main_screen.enable_alternate_modes(false)
/// {
///    let crossterm = Crossterm::new(&alternate_screen.screen);
///    let cursor = crossterm.cursor();
/// }
/// ```
pub struct Crossterm {
    stdout: Option<Arc<TerminalOutput>>,
}

impl<'crossterm> Crossterm {
    /// Create a new instance of `Crossterm`
    pub fn new() -> Crossterm {
        Crossterm { stdout: None }
    }

    /// Create a new instance of `Crossterm`
    #[cfg(feature = "screen")]
    pub fn from_screen(screen: &crossterm_screen::Screen) -> Crossterm {
        Crossterm {
            stdout: Some(screen.stdout.clone()),
        }
    }
    /// Get a `TerminalCursor` implementation whereon cursor related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    ///
    /// let crossterm = Crossterm::new();
    /// let cursor = crossterm.cursor();
    /// ```
    #[cfg(feature = "cursor")]
    pub fn cursor(&self) -> crossterm_cursor::TerminalCursor {
        match &self.stdout {
            None => crossterm_cursor::TerminalCursor::new(),
            Some(stdout) => crossterm_cursor::TerminalCursor::from_output(&stdout),
        }
    }

    /// Get a `TerminalInput` implementation whereon terminal related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    ///
    /// let crossterm = Crossterm::new();
    /// let input = crossterm.input();
    /// ```
    #[cfg(feature = "input")]
    pub fn input(&self) -> crossterm_input::TerminalInput {
        match &self.stdout {
            None => crossterm_input::TerminalInput::new(),
            Some(stdout) => crossterm_input::TerminalInput::from_output(&stdout),
        }
    }

    /// Get a `Terminal` implementation whereon terminal related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    ///
    /// let crossterm = Crossterm::new();
    /// let mut terminal = crossterm.terminal();
    /// ```
    #[cfg(feature = "terminal")]
    pub fn terminal(&self) -> crossterm_terminal::Terminal {
        match &self.stdout {
            None => crossterm_terminal::Terminal::new(),
            Some(stdout) => crossterm_terminal::Terminal::from_output(&stdout),
        }
    }

    /// Get a `TerminalColor` implementation whereon color related actions can be performed.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    ///
    /// let crossterm = Crossterm::new();
    /// let mut terminal = crossterm.color();
    /// ```
    #[cfg(feature = "style")]
    pub fn color(&self) -> crossterm_style::TerminalColor {
        match &self.stdout {
            None => crossterm_style::TerminalColor::new(),
            Some(stdout) => crossterm_style::TerminalColor::from_output(&stdout),
        }
    }

    /// This could be used to style a `Displayable` type with colors and attributes.
    ///
    /// ```rust
    /// extern crate crossterm;
    /// use crossterm::Crossterm;
    ///
    /// let crossterm = Crossterm::new();
    ///
    /// // get an styled object which could be painted to the terminal.
    /// let styled_object = crossterm.style("Some Blue colored text on black background")
    ///     .with(Color::Blue)
    ///     .on(Color::Black);
    ///
    /// // print the styled font * times to the current screen.
    /// for i in 1..10
    /// {
    ///     println!("{}", styled_object);
    /// }
    /// ```
    #[cfg(feature = "style")]
    pub fn style<D>(&self, val: D) -> crossterm_style::StyledObject<D>
    where
        D: Display,
    {
        crossterm_style::ObjectStyle::new().apply_to(val)
    }

    /// This could be used to paint the styled object onto the given screen. You have to pass a reference to the screen whereon you want to perform the painting.
    ///
    /// ``` rust
    /// style("Some colored text")
    ///     .with(Color::Blue)
    ///     .on(Color::Black)
    ///     .paint(&screen);
    /// ```
    ///
    /// You should take not that `StyledObject` implements `Display`. You don't need to call paint unless you are on alternate screen.
    /// Checkout `into_displayable()` for more information about this.
    #[cfg(feature = "style")]
    #[cfg(feature = "screen")]
    pub fn paint<'a, D: Display + 'a>(
        &self,
        styled_object: crossterm_style::StyledObject<D>,
    ) -> super::crossterm_utils::Result<()> {
        let colored_terminal = match &self.stdout {
            Some(stdout) => super::TerminalColor::from_output(stdout),
            None => super::TerminalColor::new(),
        };

        let mut reset = false;

        if let Some(bg) = styled_object.object_style.bg_color {
            colored_terminal.set_bg(bg)?;
            reset = true;
        }

        if let Some(fg) = styled_object.object_style.fg_color {
            colored_terminal.set_fg(fg)?;
            reset = true;
        }

        match self.stdout {
            None => {
                let mut stdout = io::stdout();

                for attr in styled_object.object_style.attrs.iter() {
                    write!(stdout, "{}", format!(csi!("{}m"), *attr as i16))?;
                    reset = true;
                }

                write!(stdout, "{}", styled_object.content)?;
            }
            Some(ref stdout) => {
                for attr in styled_object.object_style.attrs.iter() {
                    stdout.write_string(format!(csi!("{}m"), *attr as i16))?;
                    reset = true;
                }

                use std::fmt::Write;
                let mut content = String::new();
                write!(content, "{}", styled_object.content)?;
                stdout.write_string(content)?;
                stdout.flush()?;
            }
        }

        if reset {
            colored_terminal.reset()?;
        }

        Ok(())
    }
}

impl From<Arc<TerminalOutput>> for Crossterm {
    fn from(stdout: Arc<TerminalOutput>) -> Self {
        Crossterm {
            stdout: Some(stdout),
        }
    }
}
