use super::app::TabMode;

pub struct TabManager<'a> {
    index: usize,
    titles: Vec<&'a str>,
}

impl<'a> TabManager<'a> {
    pub fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
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
    pub fn titles(&self) -> &Vec<&str> {
        &self.titles
    }
    pub fn current_index(&self) -> usize {
        self.index
    }
    pub fn current_tab(&self) -> TabMode {
        match self.index {
            0 => TabMode::LayerMode,
            1 => TabMode::OneKeyMode,
            _ => unreachable!(),
        }
    }
    pub fn go_to_tab(&mut self, index: usize) {
        if index < self.titles.len() {
            self.index = index
        }
    }
}
