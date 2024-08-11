mod app_config;
use app_config::get_config;

fn main() {
    let config = get_config();

    println!("Hello, world!");
}
