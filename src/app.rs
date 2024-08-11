use crate::app_config::Cli;

mod file_walker;
pub use file_walker::start_handle_job;

pub struct App(Cli);
