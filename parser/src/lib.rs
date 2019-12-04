mod ast;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(next);

pub fn parse<'inp>(
    input: &'inp str,
) -> Result<ast::Program, lalrpop_util::ParseError<usize, next::Token<'inp>, &'static str>> {
    let parser = next::NextParser::new();
    parser.parse(input)
}

#[cfg(test)]
mod tests {
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub next);

    use crate::ast;

    #[test]
    fn let_stmt_ok() {
        let result = next::NextParser::new().parse("let i = 114514;");
        assert!(result.is_ok());

        let result = next::NextParser::new().parse("let ident = ident2;");
        assert!(result.is_ok());

        let result = next::NextParser::new().parse("let ident = Ident2;");
        assert!(result.is_err());
    }

    #[test]
    fn let_stmt_err() {
        let result = next::NextParser::new().parse("let ident = Ident2;");
        assert!(result.is_err());

        let result = next::NextParser::new().parse("let ident = 1dent2;");
        assert!(result.is_err());
    }

    #[test]
    fn fn_stmt_ok() {
        let result = next::NextParser::new().parse("fn id(x) { return x; }");
        assert!(result.is_ok());

        let result = next::NextParser::new().parse("fn add (x){ let i = 1  ; return i;}");
        assert!(result.is_ok());
    }

    #[test]
    fn fn_stmt_err() {
        let result = next::NextParser::new().parse("fnid(x) { return x; }");
        assert!(result.is_err());

        let result = next::NextParser::new().parse("fn add (x){ let i = 1  ; return;}");
        assert!(result.is_err());
    }
}
