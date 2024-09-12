mod models;
mod parsing;
mod exec;

use parsing::parse_config;
use exec::exec_programs;


// TODO For communication between manager and children, look at the mspc std module
fn main() {
	let parsed_result = parse_config("config.toml");  // TODO Set a default value

    let config = match parsed_result {
        Ok(config) => {
			config
        }
        Err(e) => {
            println!("Error parsing config: {}", e);
			return ;
        }
    };

	exec_programs(config);
}
