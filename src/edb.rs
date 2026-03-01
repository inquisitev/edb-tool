use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Date {
    day: i8,
    month: i8,
    year: i32,
}

impl Date {
    pub fn new(day: i8, month: i8, year: i32) -> Self {
        Self { day, month, year }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskState {
    DEFINED,
    IN_PROGRESS,
    DONE,
}

#[derive(Debug, Clone)]
pub struct Task {
    name: String,
    description: String,
    state: TaskState,
}

impl Task {
    pub fn new(name: String, description: String, state: TaskState) -> Self {
        Self {
            name,
            description,
            state,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
pub struct DayNotes {
    notes: String,
    tasks: Vec<Task>,
}

impl DayNotes {
    fn new(notes: String, tasks: Vec<Task>) -> Self {
        Self { notes, tasks }
    }
}

#[derive(Debug, Clone)]
pub struct EngineeringDayBook {
    day_notes: HashMap<Date, DayNotes>,
}

impl EngineeringDayBook {
    pub fn default() -> Self {
        Self {
            day_notes: HashMap::new(),
        }
    }
    fn new(day_notes: HashMap<Date, DayNotes>) -> Self {
        Self { day_notes }
    }

    pub fn get_defined_tasks(&self, date: Date) -> Vec<Task> {
        return self.day_notes[&date]
            .tasks
            .iter()
            .filter(|t| t.state == TaskState::DEFINED)
            .cloned()
            .collect();
    }

    pub fn get_in_progress_tasks(&self, date: Date) -> Vec<Task> {
        return self.day_notes[&date]
            .tasks
            .iter()
            .filter(|t| t.state == TaskState::IN_PROGRESS)
            .cloned()
            .collect();
    }

    pub fn get_finished_tasks(&self, date: Date) -> Vec<Task> {
        return self.day_notes[&date]
            .tasks
            .iter()
            .filter(|t| t.state == TaskState::DONE)
            .cloned()
            .collect();
    }

    pub fn example_data() -> Self {
        let mut note_map = HashMap::new();

        note_map.insert(
            Date::new(1, 2, 2020),
            DayNotes {
                notes: String::from("Stabilized core systems and cleaned up tech debt."),
                tasks: vec![
                    Task::new(
                        String::from("Clean up feature state"),
                        String::from("Remove legacy flags and unused config paths."),
                        TaskState::DONE,
                    ),
                    Task::new(
                        String::from("Refactor sprinkler system"),
                        String::from("Separate hardware abstraction from business logic."),
                        TaskState::IN_PROGRESS,
                    ),
                    Task::new(
                        String::from("Add structured logging"),
                        String::from("Replace println debugging with tracing crate."),
                        TaskState::DEFINED,
                    ),
                ],
            },
        );

        note_map.insert(
            Date::new(2, 2, 2020),
            DayNotes {
                notes: String::from("Focused on database modeling and persistence."),
                tasks: vec![
                    Task::new(
                        String::from("Design SQLite schema"),
                        String::from("Define tables for days and tasks with foreign keys."),
                        TaskState::DONE,
                    ),
                    Task::new(
                        String::from("Implement load()"),
                        String::from("Map SQL rows into EngineeringDayBook."),
                        TaskState::IN_PROGRESS,
                    ),
                    Task::new(
                        String::from("Add date key encoding"),
                        String::from("Implement YYYYMMDD conversion helpers."),
                        TaskState::DONE,
                    ),
                    Task::new(
                        String::from("Benchmark query performance"),
                        String::from("Compare indexed integer vs text ISO date."),
                        TaskState::DEFINED,
                    ),
                ],
            },
        );

        note_map.insert(
            Date::new(3, 2, 2020),
            DayNotes {
                notes: String::from("Improved internal tooling and CLI ergonomics."),
                tasks: vec![
                    Task::new(
                        String::from("Add pretty print output"),
                        String::from("Format tasks grouped by date chronologically."),
                        TaskState::DONE,
                    ),
                    Task::new(
                        String::from("Add filtering by state"),
                        String::from("Allow listing only IN_PROGRESS tasks."),
                        TaskState::IN_PROGRESS,
                    ),
                    Task::new(
                        String::from("Add task editing"),
                        String::from("Support renaming and updating state."),
                        TaskState::DEFINED,
                    ),
                ],
            },
        );

        note_map.insert(
            Date::new(4, 2, 2020),
            DayNotes {
                notes: String::from("Polished architecture and prepared for expansion."),
                tasks: vec![
                    Task::new(
                        String::from("Split domain and persistence layers"),
                        String::from("Keep SQLite logic isolated from core structs."),
                        TaskState::DEFINED,
                    ),
                    Task::new(
                        String::from("Add unit tests"),
                        String::from("Cover date encoding and task transitions."),
                        TaskState::DEFINED,
                    ),
                    Task::new(
                        String::from("Prepare migration strategy"),
                        String::from("Plan for schema evolution with versioning."),
                        TaskState::DEFINED,
                    ),
                    Task::new(
                        String::from("Document public API"),
                        String::from("Write README and usage examples."),
                        TaskState::DEFINED,
                    ),
                    Task::new(
                        String::from("Document public API 1"),
                        String::from("Write README and usage examples."),
                        TaskState::DEFINED,
                    ),
                    Task::new(
                        String::from("Document public API 2"),
                        String::from("Write README and usage examples."),
                        TaskState::DEFINED,
                    ),
                    Task::new(
                        String::from("Document public API 3 "),
                        String::from("Write README and usage examples."),
                        TaskState::DEFINED,
                    ),
                    Task::new(
                        String::from("Document public API 4"),
                        String::from("Write README and usage examples."),
                        TaskState::DEFINED,
                    ),
                ],
            },
        );
        return EngineeringDayBook {
            day_notes: note_map,
        };
    }
}
