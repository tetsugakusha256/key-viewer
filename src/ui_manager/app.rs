use tui::widgets::ScrollbarState;

use crate::{
    key_manager::{key_types::{EvdevKeyCode, EvdevModMask}, evdev_x11_tools::EvdevX11Converter},
    logger::Logger,
};
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
    pub evdev_x11_tools:EvdevX11Converter,
    pub logger: Logger,
    pub texts: Vec<String>,
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
        let mut texts = Vec::new();
        // layer 0=0, 1= 2 or 64, 2 = 16, 3=18 or 80, 4=32, 5=34 or 96
        texts.push(logger.nice_string_mask(&crate::key_manager::key_types::EvdevModMask(0)));
        texts.push(logger.nice_string_mask(&crate::key_manager::key_types::EvdevModMask(2)));
        texts.push(logger.nice_string_mask(&crate::key_manager::key_types::EvdevModMask(16)));
        texts.push(logger.nice_string_mask(&crate::key_manager::key_types::EvdevModMask(18)));
        Self {
            titles: vec!["Keyboard View", "Tab0", "Tab1", "Tab2", "Tab3"],
            running: true,
            index: 0,
            logger,
            texts,
            evdev_x11_tools: EvdevX11Converter::new("cuco"),
            vertical_scroll: 0,
            horizontal_scroll: 0,
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
    pub fn refresh_data(&mut self) {
        let logger = Logger::new_from_file(
            "/home/anon/Documents/Code/RustLearning/key_capture/output.txt".to_string(),
        )
        .unwrap();
        let mut texts = Vec::new();
        // layer 0=0, 1= 2 or 64, 2 = 16, 3=18 or 80, 4=32, 5=34 or 96
        texts.push(logger.nice_string_mask(&crate::key_manager::key_types::EvdevModMask(0)));
        texts.push(logger.nice_string_mask(&crate::key_manager::key_types::EvdevModMask(2)));
        texts.push(logger.nice_string_mask(&crate::key_manager::key_types::EvdevModMask(16)));
        texts.push(logger.nice_string_mask(&crate::key_manager::key_types::EvdevModMask(18)));
        self.logger = logger;
        self.texts = texts;
    }
    /// Get number of clicks for a key -1 => all_clicks
    pub fn clicks(&self, key_code: &EvdevKeyCode, layer: &EvdevModMask) -> u32 {
            self.logger.clicks(key_code, layer)
    }
    /// Get all clicks for a key
    pub fn all_clicks(&self, key_code: &EvdevKeyCode) -> u32 {
        self.logger.all_clicks(key_code)
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
