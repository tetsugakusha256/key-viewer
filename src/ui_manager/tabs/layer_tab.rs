use tui::{prelude::{Rect, Layout, Direction, Constraint, Backend}, Frame, widgets::{Block, Borders, Paragraph}, style::{Style, Color, Modifier}, text::Span};

use crate::{ui_manager::{app::App, widgets::{layer_choice_widget::draw_text_choice, keyboard_widget::draw_keyboard, bar_graph_keys_widget::draw_bar_graph}}, key_manager::key_types::Layer};

pub fn draw_layer_tab<B: Backend>(app: &App, size: Rect, frame: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Ratio(4, 10),
                Constraint::Ratio(6, 10),
            ]
            .as_ref(),
        )
        .split(size);
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 20),
                Constraint::Ratio(18, 20),
                Constraint::Ratio(1, 20),
            ]
            .as_ref(),
        )
        .split(chunks[1]);
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Ratio(1, 20),
                // Constraint::Ratio(6, 20),
                Constraint::Ratio(18, 20),
                Constraint::Ratio(1, 20),
            ]
            .as_ref(),
        )
        .split(chunks[2]);

    let layout_str = vec!["All", "Base", "Shift", "AltGr", "AltGr + Shift"];
    draw_text_choice(frame, chunks[0], app.index, &layout_str);

    // frame.render_widget(tabs, chunks[1]);
    // app.vertical_scroll_state = app
    //     .vertical_scroll_state
    //     .content_length(app.texts.len() as u16);
    // app.horizontal_scroll_state = app.horizontal_scroll_state.content_length(long_line.len());
    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };
    let mut paragraphs = Vec::new();
    for text in app.texts.to_owned() {
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(Color::Gray))
            .block(create_block("Default alignment (Left), no wrap"))
            .scroll((app.vertical_scroll as u16, 0));
        paragraphs.push(paragraph);
    }
    //TODO: make this more idiomatic draw_all(index)
    let layer = match app.index {
        0 => Layer::AllLayer,
        1 => Layer::Layer0,
        2 => Layer::Layer1,
        3 => Layer::Layer2,
        4 => Layer::Layer3,
        5 => Layer::Layer4,
        _ => unreachable!(),
    };
    draw_keyboard(frame, middle_chunks[1], &app, &layer);
    draw_bar_graph(
        frame,
        bottom_chunks[1],
        app.clicked_keys(&layer),
        app,
        &layer,
    );
}
