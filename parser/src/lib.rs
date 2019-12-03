use lalrpop_util::lalrpop_mod;
lalrpop_mod!(next);

pub fn parse() {
    println!(
        "hello, {}!",
        next::NumParser::new().parse("114514").unwrap()
    );
}

#[cfg(test)]
mod tests {
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(next);
    #[test]
    fn exists_next_parser() {
        assert!(next::NumParser::new().parse("114514").is_ok());
        assert_eq!(next::NumParser::new().parse("114514").unwrap(), 114514);
    }
}
