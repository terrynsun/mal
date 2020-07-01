use std::io;
use std::io::Write;

fn read(s: String) -> String {
    s
}

fn eval(s: String) -> String {
    s
}

fn print(s: String) -> String {
    s
}

fn rep(s: String) -> String {
    print(eval(read(s)))
}

fn repl_loop() {
    print!("user> ");
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading line");

    let output = rep(input);

    print!("{}", output);
}

fn main() {
    loop {
        repl_loop();
    }
}
