use std::io::{self, BufRead, Write};

enum MetaCommand {
    EXIT,
}

enum Statement {
    INSERT,
    SELECT,
}

fn print_prompt() {
    print!("db > ");
    let _ = io::stdout().flush();
}

fn parse_meta_command(line: &String) -> Result<MetaCommand, String> {
    match line.as_str() {
        ".exit" => Ok(MetaCommand::EXIT),
        _ => Err(format!("Unrecognized command: '{}'", line)),
    }
}

fn prepare_statement(line: &String) -> Result<Statement, String> {
    if line.starts_with("insert") {
        Ok(Statement::INSERT)
    }
    else if line.starts_with("select") {
        Ok(Statement::SELECT)
    }
    else {
        Err(format!("Unrecognized command: '{}'", line))
    }
}

fn main() {
    let stdin = io::stdin();
    loop {
        print_prompt();
        let line = stdin.lock().lines().next().unwrap().unwrap();
        if line.starts_with(".") {
            match parse_meta_command(&line) {
                Ok(_) => std::process::exit(0),
                Err(e) => println!("{}", e),
            }
        }
        else {
            match prepare_statement(&line) {
                Ok(Statement::INSERT) => println!("Inserting..."),
                Ok(Statement::SELECT) => println!("Selecting stuff..."),
                Err(e) => println!("{}", e),
            }
        }


    }
}
