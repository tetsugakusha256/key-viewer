

use tui::{prelude::*, widgets::*};

pub fn draw_layer_choice<B: Backend>(frame: &mut Frame<B>, area: Rect, indice: usize) {
    let layout_str = vec!["All", "Base", "Shift", "AltGr", "AltGr + Shift"];
    // color the selected word
    let layout_colored: Vec<Span> = layout_str
        .iter()
        .enumerate()
        .map(|(i, str): (usize, &&str)| {
            if i == indice {
                str.red().underlined()
            } else {
                Span::from(*str)
            }
        })
        .collect();
    // add separator
    let mut modified_vec: Vec<Span> = layout_colored
        .iter()
        .flat_map(|item| vec![item.clone(), Span::from(" | ").clone()])
        .collect();
    // pop the last added separator
    modified_vec.pop();
    let layout_line = Line::from(modified_vec).alignment(tui::prelude::Alignment::Center);
    let paragraph = Paragraph::new(vec![Line::from(""), layout_line.clone()])
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(paragraph, area);
}
