/*
Copyright (C) 2018  Nathan Typanski

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/

extern crate rustyline;

use std::io::{self, Read, Stderr, Write};
use std::collections::HashMap;
use std::process::Command;
use std::vec::Vec;

use rustyline::error::ReadlineError;
use rustyline::Editor;


struct Shell<'a> {
    builtins: HashMap<&'a str, &'a str>,
    history: Vec<String>,
    readline: Editor<()>,
}

impl<'a> Shell<'a> {
    fn new() -> Shell<'a> {
        let mut builtins = HashMap::new();
        builtins.insert("history", "TODO");
        let mut readline = Editor::<()>::new();
        if let Err(_) = readline.load_history("history.txt") {
            println!("No previous history.");
        };
        let history: Vec<String> = Vec::new();
        Shell {
            builtins: builtins,
            history: history,
            readline: readline,
        }
    }

    fn history(&mut self, words: std::str::SplitWhitespace) {
        println!("{}", self.history.join("\n"));
    }

    fn handle_string(&mut self, line: String) {
        self.readline.add_history_entry(&line);
        let mut words = line.split_whitespace();
        self.history.push(line.clone());
        match words.nth(0) {
            Some(cmd) => {
                if self.builtins.contains_key(cmd) {
                    if cmd == "history" {
                        self.history(words);
                    }
                } else {
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
