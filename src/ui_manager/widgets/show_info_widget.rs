
use tui::{prelude::*, widgets::*};
pub fn draw<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let text = vec![
        Line::from(" "),
        Line::from(vec![
            "? ".red().into(),
            ": toggle this help".into(),
        ]),
        Line::from(vec![
            "h ".red().into(),
            ": to change layer left".into(),
        ]),
        Line::from(vec![
            "i ".red().into(),
            ": to change layer right".into(),
        ]),
        Line::from(vec![
            "INS ".red().into(),
            ": to refresh datas".into(),
        ]),
        Line::from(vec![
            "r ".red().into(),
            ": to refresh".into(),
        ]),
        Line::from(vec![
            "g ".red().into(),
            ": toggle gradient view".into(),
        ]),
        Line::from(vec![
            "k ".red().into(),
            ": toggle key label".into(),
        ]),
    ];
    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Gray))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Help :"));
    frame.render_widget(paragraph, area);
}
