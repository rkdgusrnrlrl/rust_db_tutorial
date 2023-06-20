use std::io::{stdin, stdout, Write};


fn prompt(name: &str) -> String {
    print!("{}", name);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().to_string();
}

fn main() {
    loop {
        let input = prompt("db > ");
        if input == ".exit" {
            break;
        } else {
            println!("Unrecognized command '{}'.", input);
        }    
    }
    std::process::exit(0);
}
