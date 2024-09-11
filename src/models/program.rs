use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Program {
    pub cmd: String,
	pub args: Vec<String>,
    pub numprocs: u16,
    pub umask: u16,
    pub workingdir: String,
    pub autostart: bool,
    pub autorestart: String,
    pub exitcodes: Vec<u8>,
    pub startretries: i16,
    pub starttime: u16, // TODO make a list of consts
    pub stopsignal: String,
    pub stoptime: u16,
    pub stdout: String,
    pub stderr: String,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub programs: HashMap<String, Program>,
}