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
        println!("{}", input);
    }
}
