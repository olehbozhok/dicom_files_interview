
use thiserror::Error;

use crate::app_config::Cli;

mod jobs;
use jobs::JobCtx;
use jobs::JobError;

mod dicom_file_reader;

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
        let (job_ctx, rx) = JobCtx::new();

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.config.num_workers)
            .build()
            .unwrap();
        jobs::start_job(self.config.path.clone(), pool, job_ctx)?;

        while let Ok(msg) = rx.recv() {
            println!("{msg:?}");
        }

        Ok(())
    }
}
