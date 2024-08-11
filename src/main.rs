mod app_config;
use app_config::get_config;

mod app;

fn main() {
    stderrlog::new().module(module_path!()).init().unwrap();

    let config = get_config();

    if let Err(err) = app::start_handle_job(config.path.clone()) {
        let string_path = config.path.to_string_lossy();
        log::error!("could not start handle {string_path}, err: {err}")
    }

    println!("Hello, world!");
}
