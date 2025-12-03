use std::cell::RefCell;
use std::io;
use ratatui::{DefaultTerminal, Frame};

/// Application state and event handling
///
/// Provides a wrapper around [ratatui].
// `terminal` is in a RefCell to avoid some nasty borrow checker gunk in run().
// I don't fully know why it works, but it does and that's good enough for me.
pub struct State {
    terminal: RefCell<DefaultTerminal>,
    exit: bool,
}

impl State {
    /// Initialize state.
    pub fn init() -> Self {
        Self {
            terminal: RefCell::new(ratatui::init()),
            exit: false,
        }
    }

    /// Enter into rendering and event handling loop.
    pub fn run(&mut self) -> io::Result<()> {
        // core game loop
        while !self.exit {
            self.terminal.borrow_mut().draw(|frame| self.draw(frame))?;
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
}

/// Automate calling [ratatui::restore()] when State is dropped.
impl Drop for State {
    fn drop(&mut self) {
        ratatui::restore();
    }
}