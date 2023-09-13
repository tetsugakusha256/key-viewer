use tui::widgets::ScrollbarState;

use crate::{
    key_manager::{
        evdev_x11_tools::EvdevX11Converter,
        key_types::{EvdevKeyCode, Layer},
    },
    key_reader::KeyReader,
};
use std::error;
/// List of Tab (View)
pub enum Tab {
    LayerTab,
    OneKeyTab,
}
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// TODO: remove pub when possible
/// Application.
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// tabs titles
    pub titles: Vec<&'a str>,
    pub heatmap_on: bool,
    pub help_on: bool,
    current_tab: Tab,
    pub index: usize,
    pub evdev_x11_tools: EvdevX11Converter,
    pub reader: KeyReader,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: u16,
    pub horizontal_scroll: u16,

    pub select_key_mode: bool,
    pub selected_key: EvdevKeyCode,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let reader = KeyReader::new_from_file(
            "/home/anon/Documents/Code/RustLearning/key_capture/output.txt".to_string(),
            ).unwrap();
        Self {
            titles: vec!["Keyboard View", "Tab0", "Tab1", "Tab2", "Tab3"],
            running: true,
            select_key_mode: false,
            selected_key: EvdevKeyCode(36),
            index: 0,
            current_tab: Tab::OneKeyTab,
            heatmap_on: false,
            help_on: false,
            reader,
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
        let reader = KeyReader::new_from_file(
            "/home/anon/Documents/Code/RustLearning/key_capture/output.txt".to_string(),
            ).unwrap();
        self.reader = reader;
    }
    pub fn keys_clicked_before_key(&self, second_key: &EvdevKeyCode) -> Vec<(EvdevKeyCode, u32)> {
        self.reader.keys_stats.keys_clicked_before_key(second_key)
    }
    pub fn keys_clicked_after_key(&self, first_key: &EvdevKeyCode) -> Vec<(EvdevKeyCode, u32)> {
        self.reader.keys_stats.keys_clicked_after_key(first_key)
    }
    /// Get number of clicks for a key -1 => all_clicks
    pub fn clicks(&self, key_code: &EvdevKeyCode, layer: &Layer) -> u32 {
        if layer == &Layer::AllLayer {
            self.reader.keys_stats.all_clicks(key_code)
        } else {
            self.reader.keys_stats.clicks(key_code, &layer.into())
        }
    }

    pub fn clicked_keys(&self, layer: &Layer) -> Vec<(EvdevKeyCode, u32)> {
        if layer == &Layer::AllLayer {
            self.reader.keys_stats.max_clicked_keys_all_layer()
        } else {
            self.reader.keys_stats.max_clicked_keys(&layer.into())
        }
    }
    pub fn toggle_heatmap(&mut self) {
        self.heatmap_on = !self.heatmap_on
    }
    pub fn toggle_help(&mut self) {
        self.help_on = !self.help_on
    }
    pub fn get_heatmap(&self) -> bool {
        self.heatmap_on
    }
    pub fn get_current_tab(&self) -> &Tab {
        &self.current_tab
    }
    pub fn one_key_tab_on(&mut self) {
        self.current_tab = Tab::OneKeyTab
    }
    pub fn layer_tab_on(&mut self) {
        self.current_tab = Tab::LayerTab
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
    pub fn toggle_select_key_mode(&mut self) {
        self.select_key_mode = !self.select_key_mode;
    }
}
