use crate::models::Config;
use std::process::Command;


// Documentation on std::process::Command and related methods
// https://doc.rust-lang.org/std/process/struct.Command.html#method.args

pub fn exec_programs(config: Config) {
	for program in config.programs.iter() {
		// DEBUG
		println!("{} {:#?}", program.0, program.1);
		// Create command
		let mut exec_ls = Command::new(program.1.cmd.clone());
		// Set the args
		exec_ls.args(program.1.args.clone());
		// Distinction on result status
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