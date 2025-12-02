use std::io;
use ratatui::{DefaultTerminal, Frame};

pub struct State {
    exit: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            exit: false,
        }
    }
    
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }

    pub fn restore() {
        ratatui::restore();
    }
}