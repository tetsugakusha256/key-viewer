use tui::{
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    key_manager::key_types::Layer,
    ui_manager::{
        app::App,
        widgets::{
            bar_graph_keys_widget::draw_bar_graph, keyboard_widget::draw_keyboard,
            layer_choice_widget::draw_text_choice, one_key_info_widget::draw_one_key_info, bar_keys_horiz_widget::draw_bar_graph_horiz,
        },
    },
};

pub fn draw_one_key_tab<B: Backend>(app: &App, size: Rect, frame: &mut Frame<B>) {
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
                Constraint::Ratio(1, 32),
                Constraint::Ratio(5, 32),
                Constraint::Ratio(1, 32),
                Constraint::Ratio(7, 32),
                Constraint::Ratio(1, 32),
                Constraint::Ratio(7, 32),
                Constraint::Ratio(1, 32),
                Constraint::Ratio(8, 32),
                Constraint::Ratio(1, 32),
            ]
            .as_ref(),
        )
        .split(chunks[2]);

    let layout_str = vec!["Key pressed after", "Key pressed before"];
    draw_text_choice(frame, chunks[0], app.index, &layout_str);
    draw_one_key_info(frame, bottom_chunks[1], app);

    let layout_line = Line::from("  Selected key : a").alignment(tui::prelude::Alignment::Left);
    let paragraph = Paragraph::new(vec![Line::from(""), layout_line.clone()])
        .style(Style::default().fg(Color::Gray));
    frame.render_widget(paragraph, chunks[0]);
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
    draw_bar_graph_horiz(
        frame,
        bottom_chunks[3],
        app.clicked_keys(&layer),
        app,
    );
    draw_bar_graph_horiz(
        frame,
        bottom_chunks[5],
        app.clicked_keys(&layer),
        app,
    );
    draw_bar_graph_horiz(
        frame,
        bottom_chunks[7],
        app.clicked_keys(&layer),
        app,
    );
}
