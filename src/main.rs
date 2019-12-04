use next_parser;

fn main() {
    let input = "
    fn main() {
        let x = 114514;
        return x;
    }
    ";

    match next_parser::parse(input) {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e),
    };
}
