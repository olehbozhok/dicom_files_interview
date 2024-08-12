use rayon::ThreadPoolBuildError;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
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
    #[error("{0}")]
    OutputError(#[from] OutputError),
    #[error("could not build thread pool: {0}")]
    ThreadPoolBuild(#[from] ThreadPoolBuildError),
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
            .build()?;

        let (job_ctx, rx) = JobCtx::new(pool);

        jobs::start_job(self.config.path.clone(), job_ctx)?;

        let mut pipe_output = get_pipe_output(self.config.result_filepath.clone())?;

        let display_formatter = DisplayStrategy::new(self.config.output_format);

        display_formatter.display_headers(&mut pipe_output)?;
        while let Ok(file_data) = rx.recv() {
            display_formatter.display_element(&file_data, &mut pipe_output)?
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum OutputError {
    #[error("could not open result file: {0}")]
    OpenFile(#[from] io::Error),
}

fn get_pipe_output(file_path: Option<PathBuf>) -> Result<Box<dyn Write>, OutputError> {
    let pipe: Box<dyn Write> = match file_path {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    };
    Ok(pipe)
}
