use crate::{app::App, edb::EngineeringDayBook};

pub mod app;
pub mod edb;
pub mod event;
pub mod ui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let mut app = App::new(EngineeringDayBook::example_data());
    let result = app.run(terminal).await;
    ratatui::restore();
    result
}
