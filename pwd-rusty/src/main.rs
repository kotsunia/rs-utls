use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let argc = args.len();
    // let argv = args.clone();
    // ^ idk if i need them tbh lmfao

    let ch: u32;
    let mut __1flag: u32 = 0;

    let p: &[u8];

    for args in args.iter().skip(1) {
        match args.as_str() {
            "-L" => __1flag = 1,
            "-P" => __1flag = 0,

            _ => {
                println!("test")
            }
        }
    }
}
