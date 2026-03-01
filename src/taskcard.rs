use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::edb::Task;

pub struct TaskCard {
    task_name: String,
}

impl TaskCard {
    pub fn from(task: &Task) -> Self {
        let name = task.get_name().clone();
        Self { task_name: name }
    }

    pub fn get_height(&self) -> i32 {
        5
    }
}

impl Widget for &TaskCard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let test = Block::new().borders(Borders::ALL);
        let paragraph = Paragraph::new(self.task_name.clone()).block(test);
        paragraph.render(area, buf);
    }
}
