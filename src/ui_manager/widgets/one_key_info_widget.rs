use tui::{
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::Paragraph,
    Frame,
};

use crate::{
    key_manager::key_types::{self, Layer, LAYER_0, LAYER_1, LAYER_2, LAYER_3},
    ui_manager::app::App,
};

pub fn draw_one_key_info<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Ratio(10, 10)].as_ref())
        .split(area);
    let chunks_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(7, 10), Constraint::Ratio(3, 10)].as_ref())
        .split(chunks[1]);
    let key = &app.selected_key;
    let key_code = key.to_string();
    let key_name = key_types::evdev_keycode_to_name(key);
    let key_char_l0 = &app.evdev_x11_tools.get_x11_char(key, &LAYER_0);
    let key_char_l1 = &app.evdev_x11_tools.get_x11_char(key, &LAYER_1);
    let key_char_l2 = &app.evdev_x11_tools.get_x11_char(key, &LAYER_2);
    let key_char_l3 = &app.evdev_x11_tools.get_x11_char(key, &LAYER_3);
    let key_clicks_l0 = &app.clicks(key, &Layer::Layer0).to_string();
    let key_clicks_l1 = &app.clicks(key, &Layer::Layer1).to_string();
    let key_clicks_l2 = &app.clicks(key, &Layer::Layer2).to_string();
    let key_clicks_l3 = &app.clicks(key, &Layer::Layer3).to_string();
    let key_clicks_all = &app.clicks(key, &Layer::AllLayer).to_string();
    let text_left = vec![
        Line::from(" "),
        Line::from(vec!["X11 name ".red().into(), key_name.into()]),
        Line::from(vec!["Code ".red().into(), key_code.into()]),
        Line::from(" "),
        Line::from(vec!["Layer ".red().into(), ":".into()]),
        Line::from(" "),
        Line::from(vec![" Base ".red().into(), key_clicks_l0.into()]),
        Line::from(vec![" Shift ".red().into(), key_clicks_l1.into()]),
        Line::from(vec![" AltGr ".red().into(), key_clicks_l2.into()]),
        Line::from(vec![" Al+Sh ".red().into(), key_clicks_l3.into()]),
        Line::from(vec![" All ".red().into(), key_clicks_all.into()]),
    ];
    let text_right = vec![
        Line::from(" "),
        Line::from(" "),
        Line::from(" "),
        Line::from(" "),
        Line::from(" "),
        Line::from(" "),
        Line::from(vec![key_char_l0.into()]),
        Line::from(vec![key_char_l1.into()]),
        Line::from(vec![key_char_l2.into()]),
        Line::from(vec![key_char_l3.into()]),
        Line::from(" "),
    ];
    let paragraph_left = Paragraph::new(text_left.clone()).style(Style::default().fg(Color::Gray));
    let paragraph_right =
        Paragraph::new(text_right.clone()).style(Style::default().fg(Color::Gray));

    let title = Paragraph::new(Line::from(" Information")).style(Style::default().fg(Color::Gray));

    frame.render_widget(title, chunks[0]);
    frame.render_widget(paragraph_left, chunks_horizontal[0]);
    frame.render_widget(paragraph_right, chunks_horizontal[1]);
}
