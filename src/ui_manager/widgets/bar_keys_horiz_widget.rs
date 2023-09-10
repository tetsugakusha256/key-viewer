use crate::{
    key_manager::key_types::{self, EvdevKeyCode, Layer},
    ui_manager::app::App,
};

use tui::{prelude::*, widgets::*};
pub fn draw_bar_graph_horiz<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    keys: Vec<(EvdevKeyCode, u32)>,
    app: &App,
) {
    let mut bars: Vec<Bar> = vec![];
    for (key_code, clicks) in keys {
        let key_code_string = app
            .evdev_x11_tools
            .get_x11_char(&key_code, &Layer::AllLayer.into());

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
        let bar = Bar::default()
            .text_value(_name.clone() + " " + &clicks.to_string())
            .value(clicks.into())
            .value_style(Style::default().bg(Color::Green).fg(Color::Black));
        bars.push(bar);
    }

    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().fg(Color::Gray))
        .padding(Padding::new(1,1,1,1))
        .title(Span::styled(
            "Key after :",
            Style::default().add_modifier(Modifier::BOLD),
        ));
    let bar_group = BarGroup::default().bars(&bars);
    let barchart = BarChart::default()
        .block(block)
        .data(bar_group)
        .bar_width(1)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(Style::default().fg(Color::Black).bg(Color::Green))
        .direction(Direction::Horizontal);

    frame.render_widget(barchart, area);
}
