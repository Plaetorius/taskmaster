use crate::models::Config;
use std::process::Command;

pub fn exec_programs(config: Config) {
	for program in config.programs.iter() {
		println!("{} {:#?}", program.0, program.1);
		let mut exec_ls = Command::new(program.1.cmd.clone());
		match exec_ls.status() {
			Ok(status) => {
				if status.success() {
					println!("Program {} executed successfully!", program.0);
				} else {
					println!("Program {} failed!", program.0);
				}
			}
			Err(e) => {
				println!("Error starting {}: {}", program.0, e);
			}
		}
		println!();
	}
}