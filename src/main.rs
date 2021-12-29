use crate::interactive::shell_loop;

mod interactive;

fn main() {
    println!("Welcome to RustShell::Terminal");
    shell_loop();
}
