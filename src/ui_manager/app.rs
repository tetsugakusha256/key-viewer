use tui::widgets::ScrollbarState;

use crate::logger::Logger;
use std::error;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// tabs titles
    pub titles: Vec<&'a str>,
    /// current tab
    pub index: usize,
    logger: Logger,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: u16,
    pub horizontal_scroll: u16,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let logger = Logger::new_from_file(
            "/home/anon/Documents/Code/RustLearning/key_capture/output.txt".to_string(),
        )
        .unwrap();
        Self {
            titles: vec!["Tab0", "Tab1", "Tab2", "Tab3"],
            running: true,
            index: 0,
            logger,
            vertical_scroll:0,
            horizontal_scroll:0,
            vertical_scroll_state: ScrollbarState::default(),
            horizontal_scroll_state: ScrollbarState::default(),
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    pub fn logger_string(&self) -> String {
        self.logger.nice_string()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
