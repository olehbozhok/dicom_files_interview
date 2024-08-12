use std::io;
use thiserror::Error;

use crate::app_config::Cli;

mod jobs;
use jobs::JobCtx;
use jobs::JobError;

mod dicom_file_reader;

mod display_strategy;
use display_strategy::{DisplayFormat, DisplayStrategy};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("error on handle job: {0}")]
    Job(#[from] JobError),
    #[error("{0}")]
    Other(#[from] anyhow::Error),
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

        let mut pipe_output = io::stdout();

        let display_formatter = DisplayStrategy::new(self.config.output_format);

        display_formatter.display_headers(&mut pipe_output)?;
        while let Ok(file_data) = rx.recv() {
            display_formatter.display_element(&file_data, &mut pipe_output)?
        }

        Ok(())
    }
}
