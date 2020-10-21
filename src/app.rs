pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    //pub tabs: TabsState<'a>,
    pub unicode: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, unicode: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            unicode,
        }
    }

    pub fn on_tick(&mut self) {
        // update the app!
    }
}