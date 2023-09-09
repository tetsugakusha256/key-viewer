use crate::key_manager::key_types::{EvdevKeyCode, EvdevModMask};

use super::app::App;
use tui::{prelude::*, widgets::*};
pub fn draw_bar_graph<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    keys: Vec<(EvdevKeyCode, u32)>,
    app: &App,
) {
    let mut formated_keys: Vec<(String, u64)> = vec![];
    for (key_code, clicks) in keys {
        let key_code_string = app
            .evdev_x11_tools
            .get_x11_char(&key_code, &EvdevModMask(1));
        let clicks_u64 = u64::from(clicks);
        formated_keys.push((key_code_string, clicks_u64));
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
