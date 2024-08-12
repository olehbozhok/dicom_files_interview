#[deny(clippy::unwrap_used)]
use std::process::ExitCode;

mod app_config;
use app_config::get_config;

mod app;

fn main() -> ExitCode {
    stderrlog::new().module(module_path!()).init().unwrap();

    let config = get_config();

    if let Err(err) = app::App::new(config).start_handle() {
        log::error!("error: {err}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
