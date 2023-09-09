use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::{key_manager::key_types::*, ui_manager::app::App};

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
                // Constraint::Ratio(6, 20),
                Constraint::Ratio(18, 20),
                Constraint::Ratio(1, 20),
            ]
            .as_ref(),
        )
        .split(chunks[2]);

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
    draw_bar_graph(frame, bottom_chunks[1], app.clicked_keys(&layer), app, &layer);
    if app.help_on {
        let area = centered_rect(60, 60, size);
        frame.render_widget(Clear, area); //this clears out the background
        draw_show_info(frame, area)
    }
}
/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
