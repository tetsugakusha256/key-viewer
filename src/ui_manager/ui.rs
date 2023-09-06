use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Tabs},
    Frame,
};

use crate::ui_manager::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
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
    let inner = match app.index {
        0 => paragraphs.get(0),
        1 => paragraphs.get(1),
        2 => paragraphs.get(2),
        3 => paragraphs.get(3),
        _ => unreachable!(),
    };
    frame.render_widget(inner.unwrap().to_owned(), chunks[1]);
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        chunks[1],
        &mut app.vertical_scroll_state,
    );
}
