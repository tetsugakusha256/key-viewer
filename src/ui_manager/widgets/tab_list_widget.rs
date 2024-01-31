use tui::{
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::{
    key_manager::key_types::{self, Layer, LAYER_0, LAYER_1, LAYER_2, LAYER_3},
    ui_manager::app::{App, TabMode},
};

pub fn draw_tab_list<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    let layout_str = TabMode::all_tabmode_str();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(5, 10), Constraint::Ratio(5, 10)].as_ref())
        .split(area);
    draw_text_choice(frame, area, &app.tab.current_index(), &layout_str);
}

fn draw_text_choice<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    indice: &usize,
    text_vec: &Vec<&str>,
) {
    // color the selected word
    let layout_colored: Vec<Line> = text_vec
        .iter()
        .enumerate()
        .map(|(i, str): (usize, &&str)| {
            let mut span = Span::from(*str);
            if &i == indice {
                span = span.red().underlined();
            } else {
                span = span.white();
            }
            let line = Line::from(span).alignment(tui::prelude::Alignment::Center);
            line
        })
        .collect();

    let paragraph = Paragraph::new(layout_colored)
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(paragraph, area);
}
