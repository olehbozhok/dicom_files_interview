use jobs::JobError;

use crate::app_config::Cli;

mod jobs;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("error on handle job: {0}")]
    IO(#[from] JobError),
}

pub struct App {
    config: Cli,
}

impl App {
    pub fn new(config: Cli) -> App {
        App { config }
    }

    pub fn start_handle(&self) -> Result<(), AppError> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.config.num_workers)
            .build()
            .unwrap();
        jobs::start_job(self.config.path.clone(), pool)?;

        Ok(())
    }
}
