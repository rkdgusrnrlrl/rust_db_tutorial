use std::io::{stdin, stdout, Write};


enum MetaCommandResult {
    MetaCommandUnrecognizedCommand
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

fn prepare_statement(input: &str) -> Option<ExcuteStatement> {
    if input.starts_with("insert") {
        Some(ExcuteStatement {
            input,
            statement_type: StatementType::StatementInsert
        })
    } else if input.starts_with("select") {
        Some(ExcuteStatement {
            input,
            statement_type: StatementType::StatementSelect
        })
    } else {
        None
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

fn excute_statement(statement:ExcuteStatement) {
    match statement.statement_type {
        StatementType::StatementInsert => {
            println!("This is where we would do an insert.");
        },
        StatementType::StatementSelect => {
            println!("This is where we would do a select.");
        }
    }
}

fn main() {
    loop {
        // meta command 처리(현재는 .exit 만 처리가능)
        let input = prompt("db > ");
        if input.starts_with(".") {
            match do_meta_command(&input) {
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command '{}'.", input);
                    continue;
                }
            }
        }

        // 쿼리문의 statement(insert, select) 인 struct 를 리턴
        let statement = prepare_statement(&input);
        if statement.is_none() {
            println!("Unrecognized keyword at start of '{}'.", input);
            continue;
        }

        excute_statement(statement.unwrap());
    }
}
