mod ast;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(next);

pub fn parse() {
    let result = next::NextParser::new().parse("114514;(23);0;");

    match result {
        Ok(v) => println!("Result => {:?}", v),
        Err(e) => println!("{:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use lalrpop_util::lalrpop_mod;
    lalrpop_mod!(pub next);

    use crate::ast;

    macro_rules! int {
        ($e:expr) => {
            ast::Expr::Int($e)
        };
    }

    macro_rules! expr_stmt {
        ($e:expr) => {
            ast::Stmt::ExprStmt($e)
        };
    }

    macro_rules! program {
        ( $( $e:expr ),*) => {
            {
                let mut tmp = vec![];
                $(
                    tmp.push($e);
                )*
                ast::Program(tmp)
            }
        };
    }

    macro_rules! eint {
        ($e:expr) => {
            expr_stmt!(int!($e))
        };
    }

    #[test]
    fn exists_next_parser() {
        assert!(next::NextParser::new().parse("114514;").is_ok());
        assert_eq!(
            next::NextParser::new().parse("114514;").unwrap(),
            program![expr_stmt!(int!(114514))]
        );
    }

    #[test]
    fn multi_expr_stmts() {
        assert_eq!(
            next::NextParser::new().parse("114514;(23);0;").unwrap(),
            program![eint!(114514), eint!(23), eint!(0)]
        );
    }
}
