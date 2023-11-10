use tui::widgets::ScrollbarState;

use crate::{
    key_manager::{
        evdev_x11_tools::EvdevX11Converter,
        key_types::{EvdevKeyCode, Layer},
    },
    key_reader::KeyReader,
};
use std::error;

use super::{
    tab_manager::TabManager,
    tabs::{layer_tab::LayerTab, one_key_tab::OneKeyTab},
};
/// List of Tab (View)
#[derive(PartialEq)]
pub enum Mode {
    LayerMode,
    OneKeyMode,
}
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// TODO: remove pub when possible
/// Application.
// TODO: extract data for tabs into there own class and just keep
// an object representing the tab
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// tabs titles
    tab: TabManager<'a>,
    pub one_key_tab: OneKeyTab<'a>,
    pub layer_tab: LayerTab<'a>,
    pub heatmap_on: bool,
    pub help_on: bool,
    pub evdev_x11_tools: EvdevX11Converter,
    pub reader: KeyReader,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: u16,
    pub horizontal_scroll: u16,
    pub last_key: Option<EvdevKeyCode>,
    pub current_keys: Vec<EvdevKeyCode>,
    pub select_key_mode: bool,
    pub selected_key: EvdevKeyCode,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        let reader = KeyReader::new_from_file(
            "/home/anon/Documents/Code/RustLearning/key_capture/output.txt".to_string(),
        )
        .unwrap();
        Self {
            tab: TabManager::new(vec!["Layer view", "One key view"]),
            one_key_tab: OneKeyTab::default(),
            layer_tab: LayerTab::default(),
            current_keys: vec![],
            running: true,
            last_key: None,
            select_key_mode: false,
            selected_key: EvdevKeyCode(36),
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
        )
        .unwrap();
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
    pub fn get_current_mode(&self) -> &Mode {
        match self.tab.index {
            0 => &Mode::LayerMode,
            1 => &Mode::OneKeyMode,
            _ => unreachable!(),
        }
    }
    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn inner_next(&mut self) {
        match self.get_current_mode() {
            Mode::LayerMode => self.layer_tab.tab.next(),
            Mode::OneKeyMode => self.one_key_tab.tab.next(),
        }
    }
    pub fn inner_previous(&mut self) {
        match self.get_current_mode() {
            Mode::LayerMode => self.layer_tab.tab.previous(),
            Mode::OneKeyMode => self.one_key_tab.tab.previous(),
        }
    }
    pub fn set_layer_mode(&mut self) {
        self.tab.index = 0;
    }
    pub fn set_one_key_mode(&mut self) {
        self.tab.index = 1;
    }
    pub fn toggle_select_key_mode(&mut self) {
        self.select_key_mode = !self.select_key_mode;
    }
}
