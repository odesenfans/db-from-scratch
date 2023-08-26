mod error;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use sql_jr_parser::commands::ast::SqlQuery;
use sql_jr_parser::parse::Parse;

const HISTORY_FILE: &str = "./history.txt";

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                match SqlQuery::parse_from_raw(line.as_ref()) {
                    Ok(q) => println!("{q:?}"),
                    Err(e) => eprintln!("{e:?}"),
                }
                rl.add_history_entry(line.as_str())
                    .expect("Could not add line to history");
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                // CTRL-C, do nothing
            }
            Err(ReadlineError::Eof) => {
                // CTRL-D, exit
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.save_history(HISTORY_FILE)
}
