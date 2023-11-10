pub struct TabManager<'a> {
    pub index: usize,
    pub titles: Vec<&'a str>,
}

impl<'a> Default for TabManager<'a> {
    fn default() -> Self {
        Self {
            titles: vec!["Keyboard View", "Tab0", "Tab1", "Tab2", "Tab3", "Tab4"],
            index: 0,
        }
    }
}

impl<'a> TabManager<'a> {
    pub fn new(titles: Vec<&'a str>) -> Self {
        Self {
            titles,
            index: 0,
        }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }
    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
