mod config;
mod events;
mod app;
mod ui;

use crate::{app::App, events::Actions};

use anyhow::Result;
use std::{
    io::{stdout, Write},
    thread,
    time::{Duration, Instant},

};
use crossterm::{
    cursor::MoveTo,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{
    Terminal,
    backend::{Backend, CrosstermBackend}
};


// define exit function
fn exit() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout,LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

/* load config from config directory, or prompt to enter configuration process.
fn load_config () {

}
*/

fn main() -> Result<()> {
    let config: config::Config = argh::from_env();

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend= CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // initialise app here
    let mut app = App::new("Sueth", config.unicode, config.display8);

    // initialise event handler here
    let event_handler = events::EventHandler::new(config.tick_rate);

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        match event_handler.next()? {
            events::Event::Input(action) => {
                if action == Actions::Quit {
                    break;
                }
                // maybe I should use a handler for these two updates and have it run during the on_tick() event
                if action == Actions::ScrollTrack(',') && app.display4[0] != 0 {
                    app.display4 = app.display4.iter().map(|x| x - 1).collect();
                }
                if action == Actions::ScrollTrack('.') && app.display4[3] != 8 {
                    app.display4 = app.display4.iter().map(|x| x + 1).collect();
                }
            }
            events::Event::Tick => {
                app.on_tick();
            }
        }
        // event handling
        // match event_handler.next()?
        
   }
    // when the loop exits, exit the app
    terminal.show_cursor()?;
    exit()?;

    Ok(())
}
