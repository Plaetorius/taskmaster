use crate::models::Config;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::fs::File;

pub fn exec_programs(config: Config) {
	for program in config.programs.iter() {
		// DEBUG
		println!("{} {:#?}", program.0, program.1);

		let clone_name = program.0.clone();
		let clone_program = program.1;
		
		// Setup stdout and stderr
		// TODO check if stdour or stderr may not be given
		let file_stdout = File::create(program.1.stdout.clone()).unwrap();
		let file_stderr = File::create(program.1.stderr.clone()).unwrap();
		let stdout = Stdio::from(file_stdout);
		let stderr = Stdio::from(file_stderr);

		// Create command
		let mut program_process = Command::new(clone_program.cmd.clone());

		// Set the args
		program_process.args(clone_program.args.clone())
			.current_dir(clone_program.workingdir.clone())
			.stdout(stdout)
			.stderr(stderr);

		let handle = thread::spawn(move || {
			// TODO remove me
			thread::sleep(Duration::from_secs(1));
			
			// Distinction on result status
			match program_process.status() {
				Ok(status) => {
					match status.code() {
						Some(code) => println!("Exit code: {}", code),
						None => println!("Process was stopped by a signal"),
					}
				}
				Err(e) => {
					println!("Error starting {}: {}", clone_name, e);
				}
			}
		});
		
		println!("Process is being waited for");
		// DEBUG remove unwrap for a real handling
		handle.join().unwrap();
	}
}
