# Config file
## Arguments
- cmd \<String\>: the path to the command to be executed
- args \<Vec\<String\>\>: args to be passed to the program
- numprocs \<u16\>: number of instances of the program to launch
- umask \<u16\>: program umask
- workingdir \<String\>: path to execute the program from
- autostart \<bool\>: should the program start automatically
- autorestart \<String\>: restart, no restart or restart if quited unexpectedly
- exitcodes \<Vec\<u8\>\>: expected exit codes
- startretries \<u16\>: number of tries to make at starting the program
- starttime \<u16\>: time (in seconds) before checking if the program is alive
- stopsignal \<String\>: name of the signal to send to the program to stop it
- stoptime \<u16\>: time (in seconds) before sending a stopsignal to the program
- stdout \<String\>: path to the file where to print the output
- stderr \<String\>: path to the file where to print the error logs
- env \<HashMap\<String, String\>\>: the envionment to be set for the program

[programs.false.env]
## Example
```toml
[programs.false]
cmd = "/usr/bin/false"
args = ["--version"]
numprocs = 1
umask = 22
workingdir = "." # Set it in the env?
autostart = true
autorestart = "unexpected"
exitcodes = [0, 2]
startretries = 3
starttime = 5
stopsignal = "TERM"
stoptime = 10
stdout = "/tmp/false.stdout"
stderr = "/tmp/false.stderr"

[programs.false.env]
SHELL = "/bin/bash"
```

## Processes
- [Redirection to Stdout/Stderr](https://stackoverflow.com/questions/42726095/how-to-implement-redirection-of-stdout-of-a-child-process-to-a-file)
- [Rust's std::process::Command documentation](https://doc.rust-lang.org/std/process/struct.Command.html)
- [What's a umask?](https://askubuntu.com/questions/44542/what-is-umask-and-how-does-it-work)