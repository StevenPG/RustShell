use std::io::{stdout, Write};
use std::process;

use std::env::{current_dir, set_current_dir};
use std::path::Path;

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
    let tokens = tokenize(&user_input);
    let first_token = parse_first_token(&tokens);

    // Use only the first token to check for the command type
    match first_token.trim() {
        "exit" => exit(),
        "env" => env(),
        // TODO - do `ls` command
        "cwd" => println!("{}", current_working_directory()),
        "cd" => println!("{}", change_current_directory(tokens.clone())),
        _ => println!("Invalid Command, unable to match."),
    }
}

/// Changes the current directory based on the provided input
fn change_current_directory(tokens: Vec<&str>) -> String {
    if tokens.len() != 2 {
        return format!("Invalid arguments supplied to `cd` command, Ex. `cd /my/directory`");
    }
    if !tokens.get(0).unwrap().eq(&String::from("cd")) {
        return format!("Something went wrong, this is not a valid `cd` command, `cd /my/directory`");
    }

    // TODO - figure out whether to keep this, this idea doesn't work. Rust using chdir
    // TODO - change this to verify something exists and then keep track of cwd in program

    // Check whether this is an absolute or relative path
    let path = *tokens.get(1).unwrap();

    if path.starts_with('/') {
        let new_root = Path::new(path);
        let result = set_current_dir(new_root);
        if result.is_err() {
            format!("Error changing to {} => {}", path, result.as_ref().err().unwrap())
        } else {
            format!("Changed our directory to {}", path)
        }
    } else {
        let current_path = current_working_directory();
        let new_path = format!("{}/{}", current_path, path);
        let _new_root = Path::new(&new_path);
        let result = set_current_dir(_new_root);
        if result.is_err() {
            format!("Error changing to {} => {}", new_path, result.as_ref().err().unwrap())
        } else {
            format!("Changed our directory to {}", new_path)
        }
    }

    // set_current_dir("/workdir");
    // TODO - move into file in idiomatic rust manner
}

/// Returns the current working directory of the rust terminal
pub fn current_working_directory() -> String {
    // TODO - move into file in idiomatic rust manner
    current_dir().unwrap().as_os_str().to_os_string().into_string().unwrap()
}

/// Exit the shell due to an error
fn exit() {
    process::exit(0x0100);
}

/// Tokenize a string into a Vector that can be processed
fn parse_first_token<'a>(tokens: &'a Vec<&str>) -> &'a str {
    // TODO - move into file in idiomatic rust manner
    let first = *tokens.get(0).unwrap();
    return first.trim();
}

/// Parse the command out into individual tokens
fn tokenize(user_input: &String) -> Vec<&str> {
    // TODO - move into file in idiomatic rust manner
    user_input.split(' ').collect()
}

/// Flush terminal so output is reliable and correct
fn flush() {
    let flush_result = stdout().flush();
    if flush_result.is_err() {
        println!("Shell panic, failed to flush stdout correctly");
        exit();
    }
}

/// Prints the current configured environment variables
fn env() {
    // TODO - move into file in idiomatic rust manner
    for var in std::env::vars() {
        println!("{}={}", var.0, var.1);
    }
}

