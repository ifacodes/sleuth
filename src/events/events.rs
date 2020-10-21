use crate::events::Actions;
use std::{sync::mpsc, thread, time::Duration};
use crossterm::event::{self, Event as CEvent};
/// an event
pub enum Event<I> {
    /// input event
    Input(I),
    /// tick event
    Tick,
}

pub struct EventHandler {
    rx: mpsc::Receiver<Event<Actions>>,
    _tx: mpsc::Sender<Event<Actions>>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> EventHandler {
        let (tx, rx) = mpsc::channel();
        let e_tx = tx.clone();

        // poll for tick rate duration, if no events, send tick event
        let tick_rate = Duration::from_millis(tick_rate);
        thread::Builder::new().name("slueth: event handler".into()).spawn(move ||{
            loop {
                if event::poll(tick_rate).unwrap() {
                    if let CEvent::Key(key) = event::read().unwrap() {
                        let action = Actions::from(key);
                        e_tx.send(Event::Input(action)).unwrap();
                    }
                }

                e_tx.send(Event::Tick).unwrap();
            }
        }).unwrap();

        EventHandler { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event<Actions>, mpsc::RecvError> {
        self.rx.recv()
    }
}