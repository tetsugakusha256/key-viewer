use tui::{layout::Constraint::*, prelude::*, widgets::*};

use super::app::App;
use crate::key_manager::key_types::Layer;
use crate::{
    key_manager::evdev_x11_tools,
    key_manager::{
        evdev_x11_tools::EvdevX11Converter,
        key_types::{self, EvdevKeyCode, EvdevModMask},
    },
    ui_manager::app,
};
//, keys: Vec<(u16,u32)>
//TODO: Change color based on clicked amount, get min, max clicks and map a gradient on it
//TODO: Change the key label using the X11 layout info
pub fn draw_keyboard<B: Backend>(frame: &mut Frame<B>, area: Rect, app: &App, layer: Layer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Length(2), // First keyboard row
            Length(2),
            Length(2),
            Length(2),
            Length(2),
            Length(2),
            Min(0), // fills remaining space
        ])
        .split(area);
    //TODO: keyboard layout
    // nbr keys per row: 1->17, 2->14, 3->14, 4->14, 5->13, 6->11 , 6½->3
    // idea: try creating the keyboard with the tuples, (evdev keycode, constraints)
    let keyboard_rows_aa = vec![
        (EvdevKeyCode(143), "Fn", Ratio(1, 15)),
        (EvdevKeyCode(29), "Ctrl", Ratio(1, 15)),
        (EvdevKeyCode(125), "Mod4", Ratio(1, 15)),
        (EvdevKeyCode(56), "Alt", Ratio(1, 15)),
        (EvdevKeyCode(57), "Space", Ratio(5, 15)),
        (EvdevKeyCode(100), "AltGr", Ratio(1, 15)),
        (EvdevKeyCode(99), "PrScn", Ratio(1, 15)),
        (EvdevKeyCode(97), "Ctrl", Ratio(1, 15)),
        (EvdevKeyCode(104), "PgUp", Ratio(1, 15)),
        (EvdevKeyCode(103), "Up", Ratio(1, 15)),
        (EvdevKeyCode(109), "PgDn", Ratio(1, 15)),
    ];
    let keyboard_rows_ab = vec![
        (EvdevKeyCode(42), "shift", Ratio(3, 30)),
        (EvdevKeyCode(86), "<", Ratio(2, 30)),
        (EvdevKeyCode(44), "z", Ratio(2, 30)),
        (EvdevKeyCode(45), "x", Ratio(2, 30)),
        (EvdevKeyCode(46), "c", Ratio(2, 30)),
        (EvdevKeyCode(47), "v", Ratio(2, 30)),
        (EvdevKeyCode(48), "b", Ratio(2, 30)),
        (EvdevKeyCode(49), "k", Ratio(2, 30)),
        (EvdevKeyCode(50), "m", Ratio(2, 30)),
        (EvdevKeyCode(51), ",", Ratio(2, 30)),
        (EvdevKeyCode(52), ".", Ratio(2, 30)),
        (EvdevKeyCode(53), "/", Ratio(2, 30)),
        (EvdevKeyCode(54), "shift", Ratio(5, 30)),
    ];
    let keyboard_rows_ac = vec![
        (EvdevKeyCode(1), "caps", Ratio(4, 30)),
        (EvdevKeyCode(30), "a", Ratio(2, 30)),
        (EvdevKeyCode(31), "r", Ratio(2, 30)),
        (EvdevKeyCode(32), "s", Ratio(2, 30)),
        (EvdevKeyCode(33), "t", Ratio(2, 30)),
        (EvdevKeyCode(34), "d", Ratio(2, 30)),
        (EvdevKeyCode(35), "h", Ratio(2, 30)),
        (EvdevKeyCode(36), "n", Ratio(2, 30)),
        (EvdevKeyCode(37), "e", Ratio(2, 30)),
        (EvdevKeyCode(38), "i", Ratio(2, 30)),
        (EvdevKeyCode(39), "o", Ratio(2, 30)),
        (EvdevKeyCode(40), "'", Ratio(2, 30)),
        (EvdevKeyCode(43), "¦", Ratio(2, 30)),
        (EvdevKeyCode(28), "", Ratio(2, 30)),
    ];
    let keyboard_rows_ad = vec![
        (EvdevKeyCode(15), "tab", Ratio(3, 30)),
        (EvdevKeyCode(16), "q", Ratio(2, 30)),
        (EvdevKeyCode(17), "w", Ratio(2, 30)),
        (EvdevKeyCode(18), "f", Ratio(2, 30)),
        (EvdevKeyCode(19), "p", Ratio(2, 30)),
        (EvdevKeyCode(20), "g", Ratio(2, 30)),
        (EvdevKeyCode(21), "j", Ratio(2, 30)),
        (EvdevKeyCode(22), "l", Ratio(2, 30)),
        (EvdevKeyCode(23), "u", Ratio(2, 30)),
        (EvdevKeyCode(24), "y", Ratio(2, 30)),
        (EvdevKeyCode(25), "=", Ratio(2, 30)),
        (EvdevKeyCode(26), "ç", Ratio(2, 30)),
        (EvdevKeyCode(27), "]", Ratio(2, 30)),
        (EvdevKeyCode(28), "cr", Ratio(3, 30)),
    ];
    let keyboard_rows_numbers = vec![
        (EvdevKeyCode(41), "`", Ratio(1, 16)),
        (EvdevKeyCode(2), "1", Ratio(1, 16)),
        (EvdevKeyCode(3), "2", Ratio(1, 16)),
        (EvdevKeyCode(4), "3", Ratio(1, 16)),
        (EvdevKeyCode(5), "4", Ratio(1, 16)),
        (EvdevKeyCode(6), "5", Ratio(1, 16)),
        (EvdevKeyCode(7), "6", Ratio(1, 16)),
        (EvdevKeyCode(8), "7", Ratio(1, 16)),
        (EvdevKeyCode(9), "8", Ratio(1, 16)),
        (EvdevKeyCode(10), "9", Ratio(1, 16)),
        (EvdevKeyCode(11), "0", Ratio(1, 16)),
        (EvdevKeyCode(12), "-", Ratio(1, 16)),
        (EvdevKeyCode(13), "=", Ratio(1, 16)),
        (EvdevKeyCode(14), "Backspace", Ratio(3, 16)),
    ];
    let keyboard_rows_fn = vec![
        (EvdevKeyCode(1), "Esc", Ratio(3, 36)),
        (EvdevKeyCode(59), "F1", Ratio(2, 36)),
        (EvdevKeyCode(60), "F2", Ratio(2, 36)),
        (EvdevKeyCode(61), "F3", Ratio(2, 36)),
        (EvdevKeyCode(62), "F4", Ratio(2, 36)),
        (EvdevKeyCode(63), "F5", Ratio(2, 36)),
        (EvdevKeyCode(64), "F6", Ratio(2, 36)),
        (EvdevKeyCode(65), "F7", Ratio(2, 36)),
        (EvdevKeyCode(66), "F8", Ratio(2, 36)),
        (EvdevKeyCode(67), "F9", Ratio(2, 36)),
        (EvdevKeyCode(68), "F10", Ratio(2, 36)),
        (EvdevKeyCode(87), "F11", Ratio(2, 36)),
        (EvdevKeyCode(88), "F12", Ratio(2, 36)),
        (EvdevKeyCode(102), "Home", Ratio(2, 36)),
        (EvdevKeyCode(107), "End", Ratio(2, 36)),
        (EvdevKeyCode(110), "Ins", Ratio(2, 36)),
        (EvdevKeyCode(111), "Del", Ratio(3, 36)),
    ];
    render_single_row(frame, chunks[0], keyboard_rows_fn, app, &layer);
    render_single_row(frame, chunks[1], keyboard_rows_numbers, app, &layer);
    render_single_row(frame, chunks[2], keyboard_rows_ad, app, &layer);
    render_single_row(frame, chunks[3], keyboard_rows_ac, app, &layer);
    render_single_row(frame, chunks[4], keyboard_rows_ab, app, &layer);
    render_single_row(frame, chunks[5], keyboard_rows_aa, app, &layer);
}
/// Renders a single example line
fn render_single_row<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    keys: Vec<(EvdevKeyCode, &str, Constraint)>,
    app: &App,
    layer: &Layer,
) {
    fn paragraph(key_name: &str, rnd: usize, clicks: u32) -> Paragraph {
        let text = vec![Line::from(key_name.clone()), Line::from(clicks.to_string())];
        if rnd % 2 == 0 {
            Paragraph::new(text.clone())
                .alignment(Alignment::Center)
                .on_black()
        } else {
            Paragraph::new(text.clone())
                .alignment(Alignment::Center)
                .on_dark_gray()
        }
    }
    let row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints::<Vec<Constraint>>(keys.iter().map(|t| t.2).collect())
        .split(area);
    //TODO: Manage key_name in a more coherant way
    for (i, (key_code, name, constr)) in keys.iter().enumerate() {
        let x11_name = app.evdev_x11_tools.get_x11_char(key_code, &layer.into());
        let _name = if x11_name.contains("keysym") {
            name
        } else if x11_name.trim().len() == 0 {
            name
        } else if key_code == &EvdevKeyCode(1) {
            name
        } else if key_code == &EvdevKeyCode(14) {
            name
        } else if x11_name.len() == 0 {
            name
        } else {
            x11_name.as_str()
        };
        match layer {
            Layer::AllLayer => {
                frame.render_widget(paragraph(_name, i, app.all_clicks(key_code)), row[i])
            }
            _ => frame.render_widget(
                paragraph(_name, i, app.clicks(key_code, &layer.into())),
                row[i],
            ),
        }
    }
}
