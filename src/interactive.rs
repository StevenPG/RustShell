use std::io::{stdout, Write};
use std::{fs, process};

use std::env::current_dir;
use std::path::Path;

/// Main "shell loop"
pub fn shell_loop() {
    let mut current_directory = current_working_directory();
    loop {
        input(&mut current_directory)
    }
}

/// Takes input and performs a shell operation based on the string input
fn input(current_directory: &mut String) {
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
        "help" => help(),
        "env" => env(),
        "ls" => ls(current_directory),
        "cwd" => println!("{}", &current_directory),
        "cd" => println!("{}", change_current_directory(tokens.clone(), current_directory)),
        _ => println!("Invalid Command, unable to match."),
    }
}

/// Prints out a list of commands
fn help() {
    println!("{}", "  exit - exits the program");
    println!("{}", "  help - prints out the available options");
    println!("{}", "  env  - prints out all visible environment variables");
    println!("{}", "  ls   - prints out every file in the current directory");
    println!("{}", "  cwd  - display the current working directory");
    println!("{}", "  cd   - changes the active shell directory");
}

/// Print out everything in the current directory
fn ls(current_directory: &String) {
    println!("CurrDir:{}", current_directory);
    let paths_result = fs::read_dir(&Path::new(current_directory));

    println!("{:?}", paths_result);

    match paths_result {
        Err(why) => {
            format!("! {:?}", why.kind());
        },
        Ok(paths) => for path in paths {
            let cleaned_path = path.unwrap().path().to_str().unwrap().replace(current_directory, "");
            println!(".{}", cleaned_path)
        },
    }
}

/// Changes the current directory based on the provided input
fn change_current_directory(tokens: Vec<&str>, current_directory: &mut String) -> String {
    println!("InProg::current_directory->{}", current_directory);
    if tokens.len() != 2 {
        return format!("Invalid arguments supplied to `cd` command, Ex. `cd /my/directory`");
    }
    if !tokens.get(0).unwrap().eq(&String::from("cd")) {
        return format!("Something went wrong, this is not a valid `cd` command, `cd /my/directory`");
    }

    // Check whether this is an absolute or relative path
    let path = *tokens.get(1).unwrap();

    if path.starts_with('/') {
        current_directory.clear();
        current_directory.push_str(path);
        format!("Changed our directory to {}", path)
    } else {
        format!("Redo and prepend /")
    }
}

/// Returns the current working directory of the rust terminal
pub fn current_working_directory() -> String {
    current_dir().unwrap().as_os_str().to_os_string().into_string().unwrap()
}

/// Exit the shell due to an error
fn exit() {
    process::exit(0x0100);
}

/// Tokenize a string into a Vector that can be processed
fn parse_first_token<'a>(tokens: &'a Vec<&str>) -> &'a str {
    let first = *tokens.get(0).unwrap();
    return first.trim();
}

/// Parse the command out into individual tokens
fn tokenize(user_input: &String) -> Vec<&str> {
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
    for var in std::env::vars() {
        println!("{}={}", var.0, var.1);
    }
}

