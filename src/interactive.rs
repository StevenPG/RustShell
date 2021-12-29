use std::io::{stdout, Write};
use std::process;

use std::env::current_dir;

/// Main "shell loop"
pub fn shell_loop() {
    loop {
        input()
    }
}

/// Takes input and performs a shell operation based on the string input
fn input() {
    let mut line = String::new();
    print!("> ");
    flush();

    std::io::stdin().read_line(&mut line).unwrap();
    let user_input = String::from(line);
    let first_token = parse_first_token(&user_input);

    match first_token.trim() {
        "exit" => exit(),
        "cwd" => println!("{}", current_working_directory()),
        "cd" => println!("{}", change_current_directory(user_input)),
        _ => println!("Invalid Command, unable to match."),
    }
}

/// Returns the current working directory of the rust terminal
fn current_working_directory() -> String {
    current_dir().unwrap().as_os_str().to_os_string().into_string().unwrap()
}

/// Changes the current directory based on the provided input
fn change_current_directory(user_input: String) -> String {
    // check string is made up of correctly
    // set_current_dir("/workdir");
    format!("Provided `cd` command => {}, work in progress", user_input.trim())
}

/// Exit the shell due to an error
fn exit() {
    process::exit(0x0100);
}

/// Tokenize a string into a Vector that can be processed
fn parse_first_token(user_input: &String) -> &str {
    let v: Vec<&str> = user_input.split(' ').collect();
    let first = *v.get(0).unwrap();
    return first.trim();
}

/// Flush terminal so output is reliable and correct
fn flush() {
    let flush_result = stdout().flush();
    if flush_result.is_err() {
        println!("Shell panic, failed to flush stdout correctly");
        exit();
    }
}

