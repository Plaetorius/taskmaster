mod models;
mod parsing;
use parsing::parse_config;

fn main() {
	let config = parse_config("config.toml");  // TODO Set a default value

    match config {
        Ok(config) => {
            println!("{:#?}", config);
        }
        Err(e) => {
            println!("Error parsing config: {}", e);
        }
    }
}
