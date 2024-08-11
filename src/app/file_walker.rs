use std::fs;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JobError {
    #[error("could not read directory: `{0}`, err: {1}")]
    Dir(PathBuf, io::Error),
    #[error("io error: {0}")]
    IO(#[from] io::Error),

    #[error("could not start rayon thread pull: {0}")]
    RayonBuild(#[from] rayon::ThreadPoolBuildError),
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
            .map(|res| {
                res.map(|e| e.path())
                    .map_err(Into::<JobError>::into)
            })
            .map(|path_result| path_result.and_then(check_path))
            .collect::<Result<Vec<_>, JobError>>()
    }

    fn do_job(self, scope: &rayon::Scope<'_>) -> Result<(), JobError> {
        self.scan_jobs()?.into_iter().for_each(|entry| {
            scope.spawn(move |s| do_job(entry, s)) // Recursive call here
        });
        Ok(())
    }
}

#[derive(Debug)]
struct FileJob(PathBuf);

impl FileJob {
    fn do_job<'a>(self) -> Result<(), JobError> {
        let string_path = self.0.to_string_lossy();
        println!("file path: {string_path}");
        Ok(())
    }
}

#[derive(Debug)]
struct UndefinedPathJob(PathBuf);

impl UndefinedPathJob {
    fn do_job<'a>(self) {
        let string_path = self.0.to_string_lossy();
        log::error!("could not handle path: {string_path}");
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

fn do_job_result(job: JobsType, scope: &rayon::Scope<'_>) -> Result<(), JobError> {
    match job {
        JobsType::Dir(dir_job) => dir_job.do_job(scope)?,
        JobsType::File(file_job) => file_job.do_job()?,
        JobsType::UndefinedPath(job) => job.do_job(),
    }

    Ok(())
}

fn do_job(job: JobsType, scope: &rayon::Scope<'_>) {
    if let Err(err) = do_job_result(job, scope) {
        log::error!("got error on handle:{err}")
    }
}

pub fn start_handle_job(path: PathBuf) -> Result<(), JobError> {
    let first_job = check_path(path)?;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    pool.scope(|scope| do_job(first_job, scope));

    Ok(())
}
