use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    key_manager::key_types::*,
    ui_manager::app::App,
};

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
                Constraint::Ratio(6, 20),
                Constraint::Ratio(12, 20),
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
    //TODO: make this more idiomatic draw_all(index)
    match app.index {
        0 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::AllLayer);
            draw_bar_graph(
                frame,
                bottom_chunks[2],
                app.logger.max_clicked_keys_all_layer(),
                app,
            )
        }
        1 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::Layer0);
            draw_bar_graph(
                frame,
                bottom_chunks[2],
                app.logger.max_clicked_keys(&LAYER_0),
                app,
            )
        }
        2 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::Layer1);
            draw_bar_graph(
                frame,
                bottom_chunks[2],
                app.logger.max_clicked_keys(&LAYER_1),
                app,
            )
        }
        3 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::Layer2);
            draw_bar_graph(
                frame,
                bottom_chunks[2],
                app.logger.max_clicked_keys(&LAYER_2),
                app,
            )
        }
        4 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::Layer3);
            draw_bar_graph(
                frame,
                bottom_chunks[2],
                app.logger.max_clicked_keys(&LAYER_3),
                app,
            )
        }
        5 => {
            draw_keyboard(frame, middle_chunks[1], &app, Layer::Layer4);
            draw_bar_graph(
                frame,
                bottom_chunks[2],
                app.logger.max_clicked_keys(&LAYER_4),
                app,
            )
        }
        _ => unreachable!(),
    };
    draw_show_info(frame, bottom_chunks[1]);
    // frame.render_stateful_widget(
    //     Scrollbar::default()
    //         .orientation(ScrollbarOrientation::VerticalRight)
    //         .begin_symbol(Some("↑"))
    //         .end_symbol(Some("↓")),
    //     chunks[1],
    //     &mut app.vertical_scroll_state,
    // );
}
