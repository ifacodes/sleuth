use argh::FromArgs;
/// Config 
#[derive(Debug, FromArgs)]
pub struct Config {

    /// time between ticks in ms
    #[argh(option, default = "250")]
    pub tick_rate: u64,

    /// enable unicode symbol usage
    #[argh(option, default = "true")]
    pub unicode: bool,

    /// enable toggle 4 / 8 track display
    #[argh(option, default = "true")]
    pub display8: bool,

}