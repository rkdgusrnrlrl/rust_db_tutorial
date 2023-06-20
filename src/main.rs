use std::io::{stdin, stdout, Write};


enum MetaCommandResult {
    MetaCommandUnrecognizedCommand
}

enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

fn prompt(name: &str) -> String {
    print!("{}", name);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}

fn do_meta_command(input: &str) -> MetaCommandResult {
    if input == ".exit" {
        std::process::exit(0);
    } else {
        return MetaCommandResult::MetaCommandUnrecognizedCommand;
    }
}

fn prepare_statement(input: &str) -> PrepareResult {
    if input.starts_with("insert") {
        PrepareResult::PrepareSuccess
    } else if input.starts_with("select") {
        PrepareResult::PrepareSuccess
    } else {
        PrepareResult::PrepareUnrecognizedStatement
    }
}

enum StatementType {
    StatementInsert,
    StatementSelect
}

struct ExcuteStatement<'a> {
    input: &'a str,
    statement_type: StatementType
}

fn main() {
    loop {
        let input = prompt("db > ");
        if input.starts_with(".") {
            match do_meta_command(&input) {
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command '{}'.", input);
                    continue;
                }
            }
        }

        match prepare_statement(&input) {
            PrepareResult::PrepareSuccess => break,
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'.", input);
                    continue;   
            }
        }
    }
}
