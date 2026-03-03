use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ch: u32;
    let mut lflag: bool = false;

    let mut p: PathBuf;

    for args in args.iter().skip(1) {
        match args.as_str() {
            "-L" => lflag = true,
            "-P" => lflag = false,

            _ => {
                usage();
            }
        }
        let p: Option<PathBuf> = if lflag { env::current_dir().ok() } else { None };
    }

    if let Option::<PathBuf> = p {
        println!("{}", Path::display())
    } else {
        println!("where te fuck is it")
    }
}

fn usage() {
    eprintln!("Usage: program [-L | -P]");
    std::process::exit(1);
}
