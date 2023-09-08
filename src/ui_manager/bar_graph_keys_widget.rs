use super::app::App;
use tui::{layout::Constraint::*, prelude::*, widgets::*};
pub fn draw_bar_graph<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    let data = vec![
        ("B1", 9),
        ("B2", 12),
        ("B3", 5),
        ("B4", 8),
        ("B5", 2),
        ("B6", 4),
        ("B7", 5),
        ("B8", 9),
        ("B9", 14),
        ("B10", 15),
        ("B11", 1),
    ];
    let barchart = BarChart::default()
        .block(Block::default().borders(Borders::NONE))
        .data(&data)
        .bar_width(5)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(Style::default().fg(Color::Black).bg(Color::Green));
    frame.render_widget(barchart, area);
}
