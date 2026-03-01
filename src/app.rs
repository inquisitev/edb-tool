use crate::edb::{Date, EngineeringDayBook, Task};
use crate::event::{AppEvent, Event, EventHandler};
use crate::tasklist::{TaskList, TaskListState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: u8,
    /// Event handler.
    pub events: EventHandler,

    day_book: EngineeringDayBook,
    todo_column_state: TaskListState,
    inprogress_column_state: TaskListState,
    done_column_state: TaskListState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            day_book: EngineeringDayBook::default(),
            todo_column_state: TaskListState::new(3),
            inprogress_column_state: TaskListState::new(0),
            done_column_state: TaskListState::new(0),
        }
    }
}

impl App {
    pub fn new(day_book: EngineeringDayBook) -> Self {
        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
            day_book,
            todo_column_state: TaskListState::new(0),
            inprogress_column_state: TaskListState::new(0),
            done_column_state: TaskListState::new(0),
        }
    }

    pub fn render_app(&mut self, frame: &mut Frame) {
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(frame.area());

        let board_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(vertical_layout[0]);
        let day_book = &self.day_book;
        let todo_tasks: Vec<Task> = self.day_book.get_defined_tasks(Date::new(4, 2, 2020));
        let inprogress_tasks: Vec<Task> =
            self.day_book.get_in_progress_tasks(Date::new(2, 2, 2020));
        let done_tasks: Vec<Task> = self.day_book.get_finished_tasks(Date::new(2, 2, 2020));

        let todo_column = TaskList::new(String::from("Todo"), todo_tasks);
        // let inprogress_column = TaskList::new(String::from("In Progress"), inprogress_tasks);
        // let done_colmn = TaskList::new(String::from("Done"), done_tasks);

        frame.render_stateful_widget(todo_column, board_layout[0], &mut self.todo_column_state);
        // frame.render_stateful_widget(
        //     inprogress_column,
        //     board_layout[1],
        //     &mut self.inprogress_column_state,
        // );
        // frame.render_stateful_widget(done_colmn, board_layout[2], &mut self.done_column_state);
        frame.render_widget(
            Block::new().borders(Borders::ALL).title("Notes"),
            vertical_layout[1],
        );
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render_app(frame))?;

            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Increment => self.increment_counter(),
                    AppEvent::Decrement => self.decrement_counter(),
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Right => self.events.send(AppEvent::Increment),
            KeyCode::Left => self.events.send(AppEvent::Decrement),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
}
