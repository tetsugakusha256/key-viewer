use tui::{
    prelude::{Backend, Rect, Layout, Direction, Constraint},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{key_manager::key_types::Layer, ui_manager::app::App};

pub fn draw_one_key_info<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Ratio(10, 10),
            ]
            .as_ref(),
        )
        .split(area);
    let chunks_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(7, 10),
                Constraint::Ratio(3, 10),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let text_left = vec![
        Line::from(" "),
        Line::from(vec!["X11 name ".red().into(), "AD01".into()]),
        Line::from(vec!["Code ".red().into(), "234".into()]),
        Line::from(" "),
        Line::from(vec!["Layer ".red().into(), ":".into()]),
        Line::from(" "),
        Line::from(vec![" Base ".red().into(), "124".into()]),
        Line::from(vec![" Shift ".red().into(), "48".into()]),
        Line::from(vec![" AltGr ".red().into(), "456".into()]),
        Line::from(vec![" Al+Sh ".red().into(), "246".into()]),
        Line::from(vec![" All ".red().into(), "124".into()]),
    ];
    let text_right = vec![
        Line::from(" "),
        Line::from(" "),
        Line::from(" "),
        Line::from(" "),
        Line::from(" "),
        Line::from(" "),
        Line::from(vec![" a".into()]),
        Line::from(vec![" A".into()]),
        Line::from(vec![" _".into()]),
        Line::from(vec![" ¤".into()]),
        Line::from(" "),
    ];
    let paragraph_left = Paragraph::new(text_left.clone())
        .style(Style::default().fg(Color::Gray));
    let paragraph_right = Paragraph::new(text_right.clone())
        .style(Style::default().fg(Color::Gray));

    let title = Paragraph::new(Line::from(" Information"))
        .style(Style::default().fg(Color::Gray));

    frame.render_widget(title, chunks[0]);
    frame.render_widget(paragraph_left, chunks_horizontal[0]);
    frame.render_widget(paragraph_right, chunks_horizontal[1]);
}
