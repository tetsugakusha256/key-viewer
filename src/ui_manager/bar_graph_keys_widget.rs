use crate::key_manager::key_types::{self, EvdevKeyCode, EvdevModMask, Layer};

use super::app::App;
use tui::{prelude::*, widgets::*};
pub fn draw_bar_graph<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    keys: Vec<(EvdevKeyCode, u32)>,
    app: &App,
    layer: &Layer,
) {
    let mut formated_keys: Vec<(String, u64)> = vec![];
    for (key_code, clicks) in keys {
        let key_code_string = app.evdev_x11_tools.get_x11_char(&key_code, &layer.into());
        let clicks_u64 = u64::from(clicks);
        let name = key_types::evdev_keycode_to_name(key_code);
        // Renaming some keys that x11 don't return correctly
        let _name = if key_code_string.contains("keysym") {
            name
        } else if key_code_string.trim().len() == 0 {
            name
        } else if key_code == EvdevKeyCode(1) {
            name
        } else if key_code == EvdevKeyCode(14) {
            name
        } else if key_code_string.len() == 0 {
            name
        } else {
            key_code_string
        };
        formated_keys.push((_name, clicks_u64));
    }

    let formated_keys_3: Vec<(&str, u64)> = formated_keys.iter().map(|(k, c)| (&**k, *c)).collect();
    let barchart = BarChart::default()
        .block(Block::default().borders(Borders::NONE))
        .data(&formated_keys_3)
        .bar_width(5)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(Style::default().fg(Color::Black).bg(Color::Green));
    frame.render_widget(barchart, area);
}
