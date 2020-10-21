use crossterm::event;

/// Contains all actions used in Slueth
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Actions {

    /// quit action
    Quit,

    /// directional movement actions
    Left,
    
    Right,
    
    Up,
    
    Down,

    Unknown,
}

/*

    eventually replace this with proper keybindings from a configs file

*/
impl From<event::KeyEvent> for Actions {
    fn from(key: event::KeyEvent) -> Self {
        match key {
            event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            } => Actions::Quit,
            event::KeyEvent {
                code: event::KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            } => Actions::Quit,
            event::KeyEvent {
                code: event::KeyCode::Left,
                ..
            } => Actions::Left,
            event::KeyEvent {
                code: event::KeyCode::Right,
                ..
            } => Actions::Right,
            event::KeyEvent {
                code: event::KeyCode::Up,
                ..
            } => Actions::Up,
            event::KeyEvent {
                code: event::KeyCode::Down,
                ..
            } => Actions::Down,
            _ => Actions::Unknown,
        }
    }
}

