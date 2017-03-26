extern crate rustyline;

use std::io::{self, Read};
use rustyline::error::ReadlineError;
use rustyline::Editor;


fn handle_string(mut rl: &mut Editor<()>, line: String) {
    rl.add_history_entry(&line);
    println!("Line: {}", line);
}


fn main() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                handle_string(&mut rl, line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
