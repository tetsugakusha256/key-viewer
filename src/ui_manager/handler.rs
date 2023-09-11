use crate::ui_manager::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.select_key_mode {
        // app.selected_key = key_event.code;
    }
    match key_event.code {
        KeyCode::Char('m') => app.toggle_select_key_mode(),
        KeyCode::Char('i') => app.next(),
        KeyCode::Char('h') => app.previous(),
        KeyCode::Char('g') => app.toggle_heatmap(),
        KeyCode::Char('k') => app.one_key_tab_on(),
        KeyCode::Char('l') => app.layer_tab_on(),
        KeyCode::Char('?') => app.toggle_help(),
        KeyCode::Char('r') => app.refresh_data(),
        KeyCode::Insert => app.refresh_data(),
        KeyCode::Char('n') => {
            app.vertical_scroll = app.vertical_scroll.saturating_add(1);
            app.vertical_scroll_state = app.vertical_scroll_state.position(app.vertical_scroll);
        }
        KeyCode::Char('e') => {
            app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
            app.vertical_scroll_state = app.vertical_scroll_state.position(app.vertical_scroll);
        }
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),

        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
