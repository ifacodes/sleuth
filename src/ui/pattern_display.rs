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
        let mut tracks = [Track::default(), Track::default(), Track::default(), Track::default(), Track::default(), Track::default(), Track::default(), Track::default(), ];
        Pattern {
            tracks: tracks,
            length: 128,
        }
    }
}

impl Pattern {
    /// creates a new Pattern with a custom length
    pub fn new(length: usize) -> Pattern {
        let mut tracks = [Track::new(length), Track::new(length), Track::new(length), Track::new(length), Track::new(length), Track::new(length), Track::new(length), Track::new(length), ];
        Pattern {
            tracks: tracks,
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

    // each track is 6? characters long. so 8 * 6 = 48

    let constraints = if app.display8 {
        vec![Constraint::Ratio(1,8), Constraint::Ratio(1,8), Constraint::Ratio(1,8), Constraint::Ratio(1,8), Constraint::Ratio(1,8), Constraint::Ratio(1,8), Constraint::Ratio(1,8), Constraint::Ratio(1,8)]
    }
    else {
        vec![Constraint::Ratio(1,4), Constraint::Ratio(1,4), Constraint::Ratio(1,4), Constraint::Ratio(1,4)]
    };
    
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .margin(1)
        .split(area);

    if app.display8 {
        draw_track(f, app, chunks[0], &mut pattern.tracks[0]);
        draw_track(f, app, chunks[1], &mut pattern.tracks[1]);
        draw_track(f, app, chunks[2], &mut pattern.tracks[2]);
        draw_track(f, app, chunks[3], &mut pattern.tracks[3]);
        draw_track(f, app, chunks[4], &mut pattern.tracks[4]);
        draw_track(f, app, chunks[5], &mut pattern.tracks[5]);
        draw_track(f, app, chunks[6], &mut pattern.tracks[6]);
        draw_track(f, app, chunks[7], &mut pattern.tracks[7]);
    }
    else {
        draw_track(f, app, chunks[app.display4[0]], &mut pattern.tracks[app.display4[0]]);
        draw_track(f, app, chunks[app.display4[1]], &mut pattern.tracks[app.display4[1]]);
        draw_track(f, app, chunks[app.display4[2]], &mut pattern.tracks[app.display4[2]]);
        draw_track(f, app, chunks[app.display4[3]], &mut pattern.tracks[app.display4[3]]);
    }
    
} 

/// draws an individual track of the tracker
fn draw_track<B> (f: &mut Frame<B>, app: &mut App, area: Rect, track: &mut Track) where B: Backend, {
    /*  
        table of each setting
        note, instrument, etc

        no header
    */
    

}