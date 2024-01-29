use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    widgets::Clear,
    Frame,
};

use crate::ui_manager::app::App;
use crate::ui_manager::app::Mode;

use super::widgets::show_info_widget::draw as draw_info;

//TODO: Display info on current view (current mod, layer etc...)

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let size = frame.size();
    match app.get_current_mode(){
        Mode::LayerMode => app.layer_tab.draw(app, size, frame),
        Mode::OneKeyMode => app.one_key_tab.draw(app, size, frame),
    }
    if app.help_on {
        let area = centered_rect(60, 60, size);
        frame.render_widget(Clear, area); //this clears out the background
        draw_info(frame, area)
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
