use tui::{widgets::{Widget, StatefulWidget, Block, Paragraph}, layout::{Rect, Alignment}, buffer::Buffer, style::Style};

#[derive(Default)]
pub struct Document<'a> {
    lines: Vec<String>,
    block: Option<Block<'a>>,
    style: Style,
}

impl<'a> Document<'a> {
    pub fn new(lines: Vec<String>) -> Document<'a> {
        Document { 
            lines, 
            block: None,
            style: Style::default(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Document<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Document<'a> {
        self.style = style;
        self
    }
}

impl<'a> Widget for Document<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = DocumentState::default();
        StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<'a> StatefulWidget for Document<'a> {
    type State = DocumentState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let inner_area = match self.block {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            },
            None => area,
        };

        let paragraph = Paragraph::new(self.lines[state.offset..].join("\n"))
            .style(self.style)
            .alignment(Alignment::Left);
        paragraph.render(inner_area, buf);
    }
}

#[derive(Default)]
pub struct DocumentState {
    offset: usize,
}

impl DocumentState {
    pub fn get_offset(&self) -> usize {
        self.offset
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
}