use crate::models::Config;
use std::process::Command;
use std::thread;
use std::time::Duration;



// Documentation on std::process::Command and related methods
// https://doc.rust-lang.org/std/process/struct.Command.html#method.args

pub fn exec_programs(config: Config) {
	for program in config.programs.iter() {
		// DEBUG
		println!("{} {:#?}", program.0, program.1);

		let clone_name = program.0.clone();
		let clone_program = program.1;
		
		// Create command
		let mut exec_ls = Command::new(clone_program.cmd.clone());

		// Set the args
		exec_ls.args(clone_program.args.clone());
		let handle = thread::spawn(move || {
			// TODO remove me
			thread::sleep(Duration::from_secs(1));
			
			// Distinction on result status
			match exec_ls.status() {
				Ok(status) => {
					if status.success() {
						println!("Program {} executed successfully!", clone_name);
					} else {
						println!("Program {} failed!", clone_name);
					}
				}
				Err(e) => {
					println!("Error starting {}: {}", clone_name, e);
				}
			}
		});
		
		println!("Salut les p'tits potes");
		// DEBUG remove unwrap
		handle.join().unwrap();
	}
}
