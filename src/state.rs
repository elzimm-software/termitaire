use std::io;
use ratatui::{DefaultTerminal, Frame};

/// Application state and event handling
///
/// Provides a wrapper around [ratatui].
///
/// Example
/// ```rust
/// let mut terminal = ratatui::init();
/// let mut state = State::new();
/// state.run(&terminal)?;
/// state.restore();
/// # Ok::<(), io::Error>(())
/// ```
pub struct State {
    exit: bool,
}

impl State {
    /// Initialize state.
    ///
    /// Example
    /// ```rust
    /// let mut state = State::new();
    /// ```
    pub fn new() -> Self {
        Self {
            exit: false,
        }
    }

    /// Enter into rendering and event handling loop.
    ///
    /// Use [ratatui::init()] to get the [DefaultTerminal].
    ///

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        
        Ok(())
    }

    /// Frame rendering helper.
    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    /// Event handling helper.
    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }

    /// Restore the terminal to its initial state.
    pub fn restore() {
        ratatui::restore();
    }
}