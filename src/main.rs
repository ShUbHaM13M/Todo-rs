pub mod app;
pub mod db;
pub mod event;
pub mod todo;
pub mod tui;
pub mod ui;
pub mod update;

use std::io::stderr;

use app::App;
use event::EventHandler;
use ratatui::{prelude::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            event::Event::Tick => {}
            event::Event::Key(key_event) => update(&mut app, key_event),
            event::Event::Mouse(_) => {}
            event::Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
