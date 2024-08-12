use super::dicom_file_reader::{self, DicomError, DicomFileData};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JobError {
    #[error("could not read directory: `{0}`, err: {1}")]
    Dir(PathBuf, io::Error),
    #[error("could not handle path: `{0}`")]
    UndefinedPath(PathBuf),
    #[error("io error: {0}")]
    IO(#[from] io::Error),

    #[error("could not start rayon thread pull: {0}")]
    RayonBuild(#[from] rayon::ThreadPoolBuildError),

    #[error("could not handle dicom file: {0}")]
    Dicom(#[from] DicomError),
}

#[derive(Debug)]
enum JobsType {
    Dir(DirJob),
    File(FileJob),
    UndefinedPath(UndefinedPathJob),
}

#[derive(Debug)]
struct DirJob(PathBuf);

impl DirJob {
    /// read folder and create list of jobs for each entry
    fn scan_jobs<'a>(self) -> Result<Vec<JobsType>, JobError> {
        fs::read_dir(&self.0)
            .map_err(|err| JobError::Dir(self.0, err))?
            .map(|res| res.map(|e| e.path()).map_err(Into::<JobError>::into))
            .map(|path_result| path_result.and_then(check_path))
            .collect::<Result<Vec<_>, JobError>>()
    }

    fn do_job(self, scope: &rayon::Scope<'_>, ctx: JobCtx) -> Result<(), JobError> {
        self.scan_jobs()?.into_iter().for_each(|entry| {
            let ctx = ctx.clone();
            scope.spawn(move |s| do_job(entry, s, ctx.clone())) // Recursive call here
        });
        Ok(())
    }
}

#[derive(Clone)]
pub struct JobCtx {
    tx: Sender<DicomFileData>,
}

impl JobCtx {
    pub fn new() -> (JobCtx, mpsc::Receiver<DicomFileData>) {
        let (tx, rx) = mpsc::channel();
        (JobCtx { tx }, rx)
    }

    fn send(&self, data: DicomFileData) {
        if let Err(_err) = self.tx.send(data) {
            log::error!("could not send DicomFileData to tx")
        }
    }
}

#[derive(Debug)]
struct FileJob(PathBuf);

impl FileJob {
    fn do_job<'a>(self, ctx: JobCtx) -> Result<(), JobError> {
        let file_data = dicom_file_reader::handle_file(self.0.clone())?;
        ctx.send(file_data);
        Ok(())
    }
}

#[derive(Debug)]
struct UndefinedPathJob(PathBuf);

impl UndefinedPathJob {
    fn do_job<'a>(self) -> Result<(), JobError> {
        Err(JobError::UndefinedPath(self.0))
    }
}

fn check_path(path: PathBuf) -> Result<JobsType, JobError> {
    let metadata = fs::metadata(&path)?;

    if metadata.is_dir() {
        Ok(JobsType::Dir(DirJob(path)))
    } else if metadata.is_file() {
        Ok(JobsType::File(FileJob(path)))
    } else if metadata.is_symlink() {
        check_path(path.canonicalize()?)
    } else {
        Ok(JobsType::UndefinedPath(UndefinedPathJob(path)))
    }
}

fn do_job_result(job: JobsType, scope: &rayon::Scope<'_>, ctx: JobCtx) -> Result<(), JobError> {
    match job {
        JobsType::Dir(dir_job) => dir_job.do_job(scope, ctx.clone())?,
        JobsType::File(file_job) => file_job.do_job(ctx)?,
        JobsType::UndefinedPath(job) => job.do_job()?,
    }

    Ok(())
}

fn do_job(job: JobsType, scope: &rayon::Scope<'_>, ctx: JobCtx) {
    if let Err(err) = do_job_result(job, scope, ctx) {
        log::error!("got error on handle:{err}")
    }
}

pub fn start_job(path: PathBuf, pool: rayon::ThreadPool, ctx: JobCtx) -> Result<(), JobError> {
    let first_job = check_path(path)?;

    pool.scope(|scope| do_job(first_job, scope, ctx));
    Ok(())
}
