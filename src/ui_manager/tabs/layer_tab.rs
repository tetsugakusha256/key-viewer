use tui::{
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::{
    key_manager::key_types::Layer,
    ui_manager::{
        app::App,
        widgets::{
            bar_graph_keys_widget::draw_bar_graph, keyboard_widget::draw_keyboard,
            layer_choice_widget::draw_text_choice,
        }, tab_manager::TabManager,
    },
};

pub struct LayerTab<'a> {
    pub tab: TabManager<'a>,
}
// pub fn set_titles(app: &mut App)

impl<'a> LayerTab<'a>{
    pub fn default() -> Self {
        let layout_str = vec!["All", "Base", "Shift", "AltGr", "AltGr + Shift"];
        Self {
            tab: TabManager::new(layout_str),
        }
    }
    pub fn draw_layer_tab<B: Backend>(&self, app: &App, size: Rect, frame: &mut Frame<B>) {
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

        draw_text_choice(frame, chunks[0], &self.tab.index, &self.tab.titles);

        //TODO: make this more idiomatic draw_all(index)
        let layer = match &self.tab.index {
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
}
