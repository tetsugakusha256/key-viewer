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
    title: &str,
) {
    let mut bars: Vec<Bar> = vec![];
    for (key_code, clicks) in keys {
        let key_name = app
            .evdev_x11_tools
            .get_key_char(&key_code, &Layer::AllLayer.into());

        let bar = Bar::default()
            .text_value(key_name.clone() + " " + &clicks.to_string())
            .value(clicks.into())
            .value_style(Style::default().bg(Color::Green).fg(Color::Black));
        bars.push(bar);
    }

    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().fg(Color::Gray))
        .padding(Padding::new(1, 1, 1, 1))
        .title(Span::styled(
            title,
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
