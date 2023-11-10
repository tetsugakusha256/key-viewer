use key_capture::ui_manager::app::{App, AppResult};
use key_capture::ui_manager::event::{Event, EventHandler};
use key_capture::ui_manager::handler::handle_key_events;
use key_capture::ui_manager::tui::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // TODO: Try to communicate with the deamon instead of reading directly the device
    // or find a way to make it so that I don't need the sudo command
    // let device_path = "/dev/input/event16"; // Replace X with the appropriate event number
    // let mut device = Device::open(device_path).expect("Failed to create device");
    
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Get input for custom process
        // if let Ok(events) = device.fetch_events() {
        //     for event in events {
        //             app.reader.send_key(&EvdevKeyCode(event.code()), &event.value());
        //     }
        // }
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
