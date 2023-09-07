use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Tabs},
    Frame,
};

use crate::ui_manager::app::App;

use super::keyboard_widget::draw_keyboard;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().style(Style::default());
    frame.render_widget(block, size);
    let titles = app
        .titles
        .iter()
        .map(|t| Line::from(t.to_string()))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    frame.render_widget(tabs, chunks[0]);
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
            draw_keyboard(frame, chunks[1], &app);
        }
        1 => {
            frame.render_widget(paragraphs.get(0).unwrap().to_owned(), chunks[1]);
        }
        2 => {
            frame.render_widget(paragraphs.get(1).unwrap().to_owned(), chunks[1]);
        }
        3 => {
            frame.render_widget(paragraphs.get(2).unwrap().to_owned(), chunks[1]);
        }
        4 => {
            frame.render_widget(paragraphs.get(3).unwrap().to_owned(), chunks[1]);
        }
        _ => unreachable!(),
    };
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        chunks[1],
        &mut app.vertical_scroll_state,
    );
}
