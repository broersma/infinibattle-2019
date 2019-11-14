mod reader;
mod writer;

use crate::BotId;
use engine::prelude::PlayerId;
use std::{
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
};

pub struct Bot {
    pub id: BotId,
    pub display_name: String,
    process: Child,

    writer: ChildStdin,
    receiver: Receiver<Result<String, io::Error>>,
}

impl Bot {
    pub fn new(id: usize, display_name: String, path: &PathBuf, bot_args: &Vec<String>) -> Self {
        let directory = path
            .parent()
            .expect(&format!("Invalid bot path: '{}'", path.display()));
        let file_name = path
            .file_name()
            .expect(&format!("Invalid bot filename: '{}'", path.display()))
            .to_str()
            .unwrap();
        let extension = path
            .extension()
            .expect(&format!("Invalid bot extension: '{}'", path.display()))
            .to_str();

        let (command, args) = match extension.unwrap().to_lowercase().as_str() {
            "dll" => ("dotnet", vec![file_name]),
            "py" => {
			let mut python_args = vec![file_name];
			python_args.append(&mut bot_args.iter().map(|s| s.as_str()).collect());
			("python3", python_args)
		}
            "jar" => ("java", vec!["-jar", file_name]),
            "php" => ("php", vec![file_name]),
            "toml" if file_name.to_lowercase() == "cargo.toml" => {
                let compile_output = Command::new("cargo")
                    .args(vec!["build", "--release"])
                    .current_dir(directory)
                    .output()
                    .expect("Could not build Rust bot");

                io::stdout().write_all(&compile_output.stdout).unwrap();
                io::stderr().write_all(&compile_output.stderr).unwrap();

                let mut cargo_args = vec!["run", "--release", "--quiet", "--"];
	  			cargo_args.append(&mut bot_args.iter().map(|s| s.as_str()).collect());
                ("cargo", cargo_args)
            }
            _ => panic!("Unknown bot extension: '{}'", file_name),
        };

        let mut process = Command::new(command)
            .args(args)
            .current_dir(directory)
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .ok()
            .expect(&format!("Failed to start bot '{}'", file_name));

        let stdin = process.stdin.take().expect("Could not get bot stdin");
        let stdout = process.stdout.take().expect("Could not get bot stdout");

        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            BufReader::new(stdout).lines().for_each(|line| {
                sender.send(line).unwrap();
            });
        });

        Self {
            id,
            display_name,
            process,
            writer: stdin,
            receiver,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.id
    }
}

impl Drop for Bot {
    fn drop(&mut self) {
        // Try to kill the child process, but ignore failure
        let _ = self.process.kill();
    }
}
