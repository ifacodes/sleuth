use argh::FromArgs;
/// Config 
#[derive(Debug, FromArgs)]
pub struct Config {

    /// time between ticks in ms
    #[argh(option, default = "250")]
    pub tick_rate: u64,

    /// enable unicode symbol usage
    #[argh(option, default = "true")]
    pub unicode: bool
}