use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Scrollbar, ScrollbarOrientation, Tabs},
    Frame,
};

use crate::{ui_manager::app::App, key_manager::key_types::{self, EvdevModMask, Layer}};

use super::{
    bar_graph_keys_widget::draw_bar_graph, keyboard_widget::draw_keyboard,
    layer_choice_widget::draw_layer_choice, show_info_widget::draw_show_info,
};

//TODO: Display info on current view (current mod, layer etc...)

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
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
                Constraint::Ratio(9, 20),
                Constraint::Ratio(9, 20),
                Constraint::Ratio(1, 20),
            ]
            .as_ref(),
        )
        .split(chunks[2]);

    let block = Block::default().style(Style::default());
    frame.render_widget(block, size);

    draw_layer_choice(frame, chunks[0], app.index);

    // frame.render_widget(tabs, chunks[1]);
    app.vertical_scroll_state = app
        .vertical_scroll_state
        .content_length(app.texts.len() as u16);
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
    match app.index {
        0 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::AllLayer);
        }
        1 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::Layer0);
        }
        2 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::Layer1);
        }
        3 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::Layer2);
        }
        4 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::Layer3);
        }
        5 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::Layer4);
        }
        _ => unreachable!(),
    };
    draw_show_info(frame, bottom_chunks[1]);
    draw_bar_graph(frame, bottom_chunks[2], app)
    // frame.render_stateful_widget(
    //     Scrollbar::default()
    //         .orientation(ScrollbarOrientation::VerticalRight)
    //         .begin_symbol(Some("↑"))
    //         .end_symbol(Some("↓")),
    //     chunks[1],
    //     &mut app.vertical_scroll_state,
    // );
}
