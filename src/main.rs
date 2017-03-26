extern crate rustyline;

use std::io::{self, Read, Stderr, Write};
use std::collections::HashMap;
use std::process::Command;

use rustyline::error::ReadlineError;
use rustyline::Editor;


struct Shell<'a> {
    builtins: HashMap<&'a str, &'a str>,
    readline: Editor<()>,
}

impl<'a> Shell<'a> {
    fn new() -> Shell<'a> {
        let mut builtins = HashMap::new();
        builtins.insert("history", "TODO");
        let mut readline = Editor::<()>::new();
        if let Err(_) = readline.load_history("history.txt") {
            println!("No previous history.");
        }
        Shell {
            builtins: builtins,
            readline: readline,
        }
    }

    fn handle_string(&mut self, line: String) {
        self.readline.add_history_entry(&line);
        let mut words = line.split_whitespace();
        match words.nth(0) {
            Some(cmd) => {
                let cmd = cmd.clone();
                let args = words.collect::<Vec<_>>();
                let mut command = Command::new(cmd);
                command.args(args);
                match command.output() {
                    Ok(output) => {
                        println!("{}", String::from_utf8(output.stdout).unwrap());
                    }
                    _ => {
                        println!("no such command: {}", cmd);
                        // TODO: figure out why the below doesn't work
                        // io::stderr().write(format!("No such command: {}", cmd).as_bytes());
                    }
                }
            },
            None => {
                return // no command!
            }
        }
    }

    fn rl(&mut self) -> Result<(), ()> {
        let line = self.readline.readline("$ ");
        match line {
            Ok(line) => {
                self.handle_string(line);
                Ok(())
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                Err(())
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                Err(())
            },
            Err(err) => {
                println!("Error: {:?}", err);
                Err(())
            }
        }
    }

    fn save_history(&mut self) {
        self.readline.save_history("history.txt").unwrap();
    }
}

fn main() {
    // `()` can be used when no completer is required
    let mut shell = Shell::new();
    loop {
        match shell.rl() {
            Ok(_) => {
                continue;
            }
            Err(_) => {
                break
            }
        }
    }
}
