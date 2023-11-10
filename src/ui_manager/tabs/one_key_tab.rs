use tui::{
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Paragraph,
    Frame,
};

use crate::{
    key_manager::key_types::{self, Layer},
    ui_manager::{
        app::App,
        tab_manager::TabManager,
        widgets::{
            bar_keys_horiz_widget::draw_bar_graph_horiz,
            keyboard_widget::draw_keyboard, layer_choice_widget::draw_text_choice,
            one_key_info_widget::draw_one_key_info,
        },
    },
};

pub struct OneKeyTab<'a> {
    pub tab: TabManager<'a>,
}
// pub fn set_titles(app: &mut App)

impl<'a> OneKeyTab<'a> {
    pub fn default() -> Self {
        let layout_str = vec!["Key pressed after", "Key pressed before"];
        Self {
            tab: TabManager::new(layout_str),
        }
    }
    pub fn draw_one_key_tab<B: Backend>(&self, app: &App, size: Rect, frame: &mut Frame<B>) {
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
                    Constraint::Ratio(10, 32),
                    Constraint::Ratio(1, 32),
                    Constraint::Ratio(7, 32),
                    Constraint::Ratio(1, 32),
                    Constraint::Ratio(10, 32),
                    Constraint::Ratio(1, 32),
                    Constraint::Ratio(1, 32),
                ]
                .as_ref(),
            )
            .split(chunks[2]);

        draw_text_choice(frame, chunks[0], &self.tab.index, &self.tab.titles);
        draw_one_key_info(frame, bottom_chunks[3], app);

        let name = key_types::evdev_keycode_to_name(&app.selected_key);
        let layout_line = Line::from("  Selected key : ".to_string() + &name)
            .alignment(tui::prelude::Alignment::Left);
        let paragraph = Paragraph::new(vec![Line::from(""), layout_line.clone()])
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(paragraph, chunks[0]);
        //TODO: make this more idiomatic draw_all(index)
        let layer = match &self.tab.index {
            0 => Layer::AllLayer,
            1 => Layer::Layer0,
            _ => unreachable!(),
        };
        draw_keyboard(frame, middle_chunks[1], &app, &layer);
        draw_bar_graph_horiz(
            frame,
            bottom_chunks[1],
            app.keys_clicked_before_key(&app.selected_key),
            app,
            "Key Before",
        );
        draw_bar_graph_horiz(
            frame,
            bottom_chunks[5],
            app.keys_clicked_after_key(&app.selected_key),
            app,
            "Key After",
        );
        // draw_bar_graph_horiz(
        //     frame,
        //     bottom_chunks[7],
        //     app.clicked_keys(&layer),
        //     app,
        //     "Total",
        // );
    }
}
