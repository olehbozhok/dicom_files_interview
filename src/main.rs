#![deny(clippy::unwrap_used)]
use std::{
    io::{stderr, Write},
    process::ExitCode,
};

mod app_config;
use app_config::get_config;

mod app;

fn main() -> ExitCode {
    if let Err(err) = stderrlog::new()
        .module(module_path!())
        .verbosity(log::Level::Info)
        .init()
    {
        _ = stderr().write_all(format!("error on init logger: {err}").as_bytes());
    };

    let config = get_config();

    if let Err(err) = app::App::new(config).start_handle() {
        log::error!("error: {err}");
        ExitCode::FAILURE
    } else {
        log::info!("done");
        ExitCode::SUCCESS
    }
}
