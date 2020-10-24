pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    //pub tabs: TabsState<'a>,
    pub unicode: bool,
    pub display8: bool,
    pub display4: Vec<usize>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, unicode: bool, display8: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            unicode,
            display8,
            display4: vec![0, 1, 2, 3],
        }
    }

    pub fn on_tick(&mut self) {
        // update the app!
    }
}