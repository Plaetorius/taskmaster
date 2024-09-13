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
		let mut file = File::create(program.1.stdout.clone()).unwrap();
		let stdout = Stdio::from(file);
		file = File::create(program.1.stderr.clone()).unwrap();
		let stderr = Stdio::from(file);

		// let outputs = [Stdio::from(files[0]), Stdio::from(files[1])];

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
		
		println!("Process is being waited for");
		// DEBUG remove unwrap for a real handling
		handle.join().unwrap();
	}
}
