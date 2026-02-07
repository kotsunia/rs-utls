use std::env;
// use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--version") {
        println!("yes but in rust v0.0.0");
        return;
    }

    let text = args.get(1).unwrap_or(&"y".to_string()).clone();

    loop {
        println!("{}", text)
    }
}
