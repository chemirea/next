use next_parser;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() >= 2 {
        &args[1]
    } else {
        "sample_project/main.nx"
    };

    let mut f = File::open(filename).expect(&format!("{} file not found", filename));

    let mut input = String::new();
    f.read_to_string(&mut input)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");

    match next_parser::parse(&input) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    };
}
