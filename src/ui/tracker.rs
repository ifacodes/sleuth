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
struct Step {

    /// current step note
    note: String,
    
    /// current step instrument
    instrument: u64,
}

/// a struct to store an vector of steps of length size
struct Track {
    /// vector of steps
    steps: Vec<Step>,
}

impl Track {
    fn new(length: u8) -> Track {
        let step_iter = iter::repeat_with(|| Step { note: "C4".to_string(), instrument: 1}).take(length.into());
        Track {
            steps: step_iter.collect(),
        }
    }
}
/// a struct to store pattern data
struct Pattern {

    /// tracks in pattern
    tracks: Vec<Track>,

    /// length of the pattern (0 to 128)
    length: u8,

}

impl Pattern {
    fn new(length: u8) -> Pattern {
        let track_iter = iter::repeat_with(|| Track::new(length)).take(8);
        Pattern {
            tracks: track_iter.collect::<Vec<Track>>(),
            length: length
        }
    }
}

/// draws the tracker widget? UI Element? 
pub fn draw_tracker<B> (f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend, {
    /*
        The Tracker will be a sort of Table Widget.
        each pattern has a max length of 128 steps. 
        each step is a 16th note
        that is 8 bars at 16th note resolution

        if screen size allows it, tracker should draw all 8 tracks of a pattern
        overwise it should display 4 at a time

    */
    let row_style = Style::default().fg(Color::White);
    Table::new(
        ["1", "2", "3", "4", "5", "6", "7", "8"].into_iter(),
        vec![
            Row::StyledData(["Test"].into_iter(), row_style)
        ].into_iter()
    );
}

/// draws an individual track of the tracker
pub fn draw_track<B> (f: &mut Frame<B>, app: &mut App, area: Rect) where B: Backend, {
    
}