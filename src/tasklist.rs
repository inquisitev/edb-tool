use std::iter;
use std::iter::Sum;
use std::task;

use ratatui::layout::Size;
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::edb::Task;
use crate::taskcard::TaskCard;

struct WindowSlice {
    index: i32,
    card_count: i32,
}

fn calculate_cards_in_window(heights: &Vec<i32>, index: i32, height: usize) -> i32 {
    let mut card_count = 1;

    while heights[(index as usize)..((index + card_count + 1) as usize)]
        .iter()
        .sum::<i32>()
        < (height as i32)
    {
        card_count += 1;
        if (index + card_count + 1) as usize > heights.len() {
            break;
        }
    }

    return card_count;
}

fn calculate_window(heights: &Vec<i32>, selected_index: i32, height: usize) -> WindowSlice {
    let mut index = 0;
    let mut card_count = calculate_cards_in_window(heights, index, height);
    while selected_index > (index + card_count) {
        index += 1;
        card_count = calculate_cards_in_window(heights, index, height);
    }

    WindowSlice { index, card_count }
}

pub struct TaskList {
    list_name: String,
    tasks: Vec<Task>,
}
pub struct TaskListState {
    selected_index: i32,
    window_slice: WindowSlice,
}

impl TaskListState {
    pub fn is_selected_index_out_of_view(&self) -> bool {
        let below_window = self.selected_index < self.window_slice.index;
        let above_window =
            self.selected_index > (self.window_slice.index + self.window_slice.card_count);
        return (above_window || below_window);
    }
}

impl TaskList {
    pub fn new(list_name: String, tasks: Vec<Task>) -> Self {
        Self { list_name, tasks }
    }
}

impl TaskListState {
    pub fn new(selected_index: i32) -> Self {
        Self {
            selected_index,
            window_slice: WindowSlice {
                index: 0,
                card_count: 0,
            },
        }
    }
}

impl StatefulWidget for TaskList {
    type State = TaskListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let defined_block = Block::new()
            .border_style(Style::default().fg(Color::Magenta))
            .borders(Borders::ALL)
            .title(self.list_name);

        let mut y = area.y;

        let task_cards: Vec<TaskCard> = self.tasks.iter().map(|f| TaskCard::from(&f)).collect();
        let heights: Vec<i32> = task_cards.iter().map(|t| t.get_height()).collect();

        if (state.is_selected_index_out_of_view()) {
            state.window_slice =
                calculate_window(&heights, state.selected_index, (area.height - 4) as usize);
        }

        for i in
            state.window_slice.index..(state.window_slice.index + state.window_slice.card_count + 1)
        {
            let card: &TaskCard = &task_cards[i as usize];
            let r = Rect {
                x: area.x + 3,
                y: y + 2,
                width: area.width - 6,
                height: (card.get_height() as u16),
            };
            y += card.get_height() as u16;
            card.render(r, buf);
        }

        defined_block.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};

    use crate::{
        edb::Task,
        tasklist::{calculate_window, TaskList, TaskListState},
    };

    fn default_task_set(card_count: i32) -> Vec<Task> {
        let mut tasks: Vec<Task> = vec![];
        for i in 1..card_count {
            tasks.push(Task::new(
                String::from(format!("Test {}", i)),
                String::from("Description"),
                crate::edb::TaskState::DEFINED,
            ));
        }

        return tasks;
    }

    #[test]
    fn test_view_several_tasks_at_zero_scroll() {
        let tasks: Vec<Task> = default_task_set(6);

        let app = TaskList::new(String::from("Todo"), tasks);
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        let mut state = TaskListState::new(2);
        terminal
            .draw(|frame| frame.render_stateful_widget(app, frame.area(), &mut state))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }
    #[test]
    fn test_view_several_tasks_at_five_scroll() {
        let tasks: Vec<Task> = default_task_set(10);

        let app = TaskList::new(String::from("Todo"), tasks);
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        let mut state = TaskListState::new(5);
        terminal
            .draw(|frame| frame.render_stateful_widget(app, frame.area(), &mut state))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }
    #[test]
    fn test_view_several_tasks_at_eight_scroll() {
        let tasks: Vec<Task> = default_task_set(10);

        let app = TaskList::new(String::from("Todo"), tasks);
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        let mut state = TaskListState::new(8);
        terminal
            .draw(|frame| frame.render_stateful_widget(app, frame.area(), &mut state))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_calculate_window() {
        let heights: Vec<i32> = vec![3, 4, 9, 2, 3, 4, 4];
        let window_1 = calculate_window(&heights, 2, 9);
        assert_eq!(window_1.index, 0);
        assert_eq!(window_1.card_count, 2);
    }

    // #[test]
    // fn test_view_doesnt_scroll_when_bottom_is_in_bounds() {
    //     let tasks: Vec<Task> = default_task_set();
    //
    //     let app = TaskList::new(String::from("Todo"), tasks);
    //     let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
    //     let mut state = TaskListState::new(3);
    //     terminal
    //         .draw(|frame| frame.render_stateful_widget(app, frame.area(), &mut state))
    //         .unwrap();
    //     assert_snapshot!(terminal.backend());
    // }
}
