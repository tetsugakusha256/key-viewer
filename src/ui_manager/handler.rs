use crate::{
    key_manager::key_types::EvdevKeyCode,
    ui_manager::app::{App, AppResult},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::event::ModifierKeyCode::*;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    let evdev_key_code = match key_event.code {
        KeyCode::Backspace => EvdevKeyCode(14),
        KeyCode::Enter => EvdevKeyCode(28),
        KeyCode::Left => EvdevKeyCode(105),
        KeyCode::Right => EvdevKeyCode(106),
        KeyCode::Up => EvdevKeyCode(103),
        KeyCode::Down => EvdevKeyCode(108),
        KeyCode::Home => EvdevKeyCode(102),
        KeyCode::End => EvdevKeyCode(107),
        KeyCode::PageUp => EvdevKeyCode(104),
        KeyCode::PageDown => EvdevKeyCode(109),
        KeyCode::Tab => EvdevKeyCode(15),
        KeyCode::Delete => EvdevKeyCode(111),
        KeyCode::Insert => EvdevKeyCode(110),
        KeyCode::Esc => EvdevKeyCode(1),
        KeyCode::CapsLock => EvdevKeyCode(1),
        KeyCode::PrintScreen => EvdevKeyCode(99),
        //TODO: Use layout to match the keys
        KeyCode::Char(char) => match char {
            'q' => EvdevKeyCode(16),
            'w' => EvdevKeyCode(17),
            'f' => EvdevKeyCode(18),
            'p' => EvdevKeyCode(19),
            'g' => EvdevKeyCode(20),
            'j' => EvdevKeyCode(21),
            'l' => EvdevKeyCode(22),
            'u' => EvdevKeyCode(23),
            'y' => EvdevKeyCode(24),
            '=' => EvdevKeyCode(25),
            ']' => EvdevKeyCode(26),
            'a' => EvdevKeyCode(30),
            'r' => EvdevKeyCode(31),
            's' => EvdevKeyCode(32),
            't' => EvdevKeyCode(33),
            'd' => EvdevKeyCode(34),
            'h' => EvdevKeyCode(35),
            'n' => EvdevKeyCode(36),
            'e' => EvdevKeyCode(37),
            'i' => EvdevKeyCode(38),
            'o' => EvdevKeyCode(39),
            '\'' => EvdevKeyCode(40),
            '¦' => EvdevKeyCode(43),
            '<' => EvdevKeyCode(86),
            'z' => EvdevKeyCode(44),
            'x' => EvdevKeyCode(45),
            'c' => EvdevKeyCode(46),
            'v' => EvdevKeyCode(47),
            'b' => EvdevKeyCode(48),
            'k' => EvdevKeyCode(49),
            'm' => EvdevKeyCode(50),
            ',' => EvdevKeyCode(51),
            '.' => EvdevKeyCode(52),
            '/' => EvdevKeyCode(53),
            ' ' => EvdevKeyCode(57),
            '1' => EvdevKeyCode(2),
            '2' => EvdevKeyCode(3),
            '3' => EvdevKeyCode(4),
            '4' => EvdevKeyCode(5),
            '5' => EvdevKeyCode(6),
            '6' => EvdevKeyCode(7),
            '7' => EvdevKeyCode(8),
            '8' => EvdevKeyCode(9),
            '9' => EvdevKeyCode(10),
            '0' => EvdevKeyCode(11),
            '-' => EvdevKeyCode(12),
            // '=' => EvdevKeyCode(13),
            '`' => EvdevKeyCode(41),
            _ => EvdevKeyCode(1),
        },
        KeyCode::Modifier(modifier) => match modifier {
            LeftShift => EvdevKeyCode(42),
            LeftControl => EvdevKeyCode(29),
            LeftAlt => EvdevKeyCode(56),
            LeftSuper => EvdevKeyCode(125),
            LeftHyper => EvdevKeyCode(125),
            LeftMeta => EvdevKeyCode(125),
            RightShift => EvdevKeyCode(54),
            RightControl => EvdevKeyCode(97),
            RightAlt => EvdevKeyCode(100),
            IsoLevel3Shift => EvdevKeyCode(100),
            IsoLevel5Shift => EvdevKeyCode(99),
            _ => EvdevKeyCode(1),
        },
        KeyCode::F(number) => match number {
            1 => EvdevKeyCode(59),
            2 => EvdevKeyCode(60),
            3 => EvdevKeyCode(61),
            4 => EvdevKeyCode(62),
            5 => EvdevKeyCode(63),
            6 => EvdevKeyCode(64),
            7 => EvdevKeyCode(65),
            8 => EvdevKeyCode(66),
            9 => EvdevKeyCode(67),
            10 => EvdevKeyCode(68),
            11 => EvdevKeyCode(87),
            12 => EvdevKeyCode(88),
            _ => EvdevKeyCode(1),
        },
        _ => EvdevKeyCode(1),
    };
    // FIX: don't get release event
    //
    // if key_event.kind == KeyEventKind::Release {
    //     app.current_keys.retain_mut(|e| e != &evdev_key_code);
    // }
    // else if key_event.kind == KeyEventKind::Press {
    //     if !app.current_keys.contains(&evdev_key_code) {
    //         app.current_keys.push(evdev_key_code);
    //     }
    // }
    if app.select_key_mode {
        app.selected_key = evdev_key_code;
        app.last_key = Some(evdev_key_code);
        app.select_key_mode = false;
    } else {
        match key_event.code {
            KeyCode::Char('m') => app.toggle_select_key_mode(),
            KeyCode::Char('i') => app.inner_next(),
            KeyCode::Char('h') => app.inner_previous(),
            KeyCode::Char('g') => app.toggle_heatmap(),
            KeyCode::Char('k') => app.set_one_key_mode(),
            KeyCode::Char('l') => app.set_layer_mode(),
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
    }
    Ok(())
}
