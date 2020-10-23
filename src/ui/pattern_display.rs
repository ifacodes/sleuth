use crate::{app::App};

use std::iter;

use tui::{
    backend::Backend,
    layout::{Layout, Constraint, Direction, Rect},
    widgets::{Widget, Block, Borders, Table, Row, TableState},
    style::{Style, Color},
    Frame
};
/// a struct to store step data
#[derive(Clone)]
struct Step {

    /// current step note
    note: String,
    
    /// current step instrument
    instrument: String,
}

impl Default for Step {
    fn default() -> Self {
        Step {
            note: String::from(""),
            instrument: String::from(""),
        }
    }
}

/// a struct to store an vector of steps of length size
struct Track {
    /// vector of steps
    steps: Vec<Step>,

    /// highlighted step index
    current_step: usize,

}

impl Default for Track {
    fn default() -> Self {
        Track {
            steps: vec![Step::default(); 128],
            current_step: 0,
        }
    }
}

impl Track {
    fn new(length: usize) -> Track {
        Track {
            steps: vec![Step::default(); length],
            ..Track::default()
        }
    }
}
/// a struct to store pattern data
pub struct Pattern {

    /// tracks in pattern
    tracks: [Track; 8],

    /// length of the pattern (0 to 128)
    length: usize,

}

impl Default for Pattern {
    // creates a new Pattern with the default length of 128
    fn default() -> Self {
        Pattern {
            tracks: [Track::default(), Track::default(), Track::default(), Track::default(), Track::default(), Track::default(), Track::default(), Track::default(), ],
            length: 128,
        }
    }
}

impl Pattern {
    /// creates a new Pattern with a custom length
    fn new(length: usize) -> Pattern {
        Pattern {
            tracks: [Track::new(length), Track::new(length), Track::new(length), Track::new(length), Track::new(length), Track::new(length), Track::new(length), Track::new(length), ],
            length: length,
        }  
    }
}



/// draws the tracker widget? UI Element? 
pub fn draw_display<B> (f: &mut Frame<B>, app: &mut App, area: Rect, pattern: &mut Pattern) where B: Backend, {
    /*
        The Tracker will be a sort of Table Widget.
        each pattern has a max length of 128 steps. 
        each step is a 16th note
        that is 8 bars at 16th note resolution

        if screen size allows it, tracker should draw all 8 tracks of a pattern
        overwise it should display 4 at a time

    */
}

/// draws an individual track of the tracker
fn draw_track<B> (f: &mut Frame<B>, app: &mut App, area: Rect, track: &mut Track) where B: Backend, {
    /*  
        table of each setting
        note, instrument, etc

        no header
    */
}