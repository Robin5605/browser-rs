use crate::document::DocumentState;

pub struct DocumentViewer {
    lines: Vec<String>,
    state: DocumentState,
}

impl DocumentViewer {
    pub fn new(contents: String) -> DocumentViewer {
        DocumentViewer { 
            lines: contents.split("\n").map(|i| String::from(i)).collect(), 
            state: DocumentState::default(), 
        }
    }

    pub fn scroll_down(&mut self) {
        if self.state.get_offset() < self.lines.len() {
            self.state.set_offset(self.state.get_offset().saturating_add(1));
        }
    }

    pub fn scroll_up(&mut self) {
        if self.state.get_offset() > 0 {
            self.state.set_offset(self.state.get_offset().saturating_sub(1));
        }
    }

    pub fn set_contents(&mut self, contents: String) {
        self.lines = contents.split("\n").map(|i| String::from(i)).collect();
    }

    pub fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    pub fn get_mut_state(&mut self) -> &mut DocumentState {
        &mut self.state
    }
}