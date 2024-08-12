#[deny(clippy::unwrap_used)]
mod app_config;
use app_config::get_config;

mod app;

fn main() {
    stderrlog::new().module(module_path!()).init().unwrap();

    let config = get_config();

    if let Err(err) = app::App::new(config).start_handle() {
        log::error!("error: {err}")
    }
}
