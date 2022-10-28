use std::fs;

use tui::widgets::{TableState};

pub struct StatefulTable {
    pub items: Vec<fs::DirEntry>,
    pub state: TableState,
}

impl StatefulTable {
    pub fn new(items: Vec<fs::DirEntry>) -> StatefulTable {
        let mut state = TableState::default();
        state.select(Some(0));

        StatefulTable { 
            items, 
            state, 
        }
    }

    pub fn set_items(&mut self, items: Vec<fs::DirEntry>) {
        self.items = items;
        let mut state = TableState::default();
        state.select(Some(0));
        self.state = state
    }

    pub fn get_selected(&self) -> &fs::DirEntry {
        self.items.get(self.state.selected().unwrap()).unwrap()
    }

    pub fn next(&mut self) {
        let i = {
            match self.state.selected() {
                Some(i) => {
                    if i >= self.items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                },
                None => 0
            }
        };
        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = {
            match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.items.len() - 1
                    } else {
                        i - 1
                    }
                },
                None => 0
            }
        };
        self.state.select(Some(i));
    }
}