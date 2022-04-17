use std::borrow::Cow;

use dash_core::parser::expr::Expr;
use dash_core::parser::expr::GroupingExpr;
use dash_core::parser::expr::LiteralExpr;
use dash_core::parser::statement::Statement;
use dash_core::parser::token::TokenType;
use itertools::Itertools;

pub trait Deduce {
    fn deduce(&mut self);
}

impl<'a, D: Deduce> Deduce for &mut [D] {
    fn deduce(&mut self) {
        for expr in self.iter_mut() {
            expr.deduce();
        }
    }
}

fn stringify_expr<'a>(e: &mut Expr<'a>) -> Cow<'a, str> {
    e.deduce();

    match e {
        Expr::Array(arr) => Cow::Owned(arr.iter_mut().map(stringify_expr).join(",")),
        Expr::Literal(LiteralExpr::String(s)) => s.to_owned(),
        Expr::Literal(LiteralExpr::Boolean(b)) => Cow::Borrowed(if *b { "true" } else { "false" }),
        Expr::Literal(LiteralExpr::Number(n)) => Cow::Owned(n.to_string()),
        Expr::Literal(LiteralExpr::Identifier(i)) => i.to_owned(),
        Expr::Literal(LiteralExpr::Null) => Cow::Borrowed("null"),
        Expr::Literal(LiteralExpr::Undefined) => Cow::Borrowed("undefined"),
        _ => todo!(),
    }
}

fn to_numeric(e: &mut Expr) -> f64 {
    e.deduce();

    match e {
        Expr::Array(arr) => match arr.first_mut() {
            Some(e) => to_numeric(e),
            _ => f64::NAN,
        },
        Expr::Literal(LiteralExpr::Boolean(b)) => *b as i8 as f64,
        Expr::Literal(LiteralExpr::Number(n)) => *n,
        Expr::Literal(LiteralExpr::Null) => 0.0,
        Expr::Literal(LiteralExpr::Undefined) => f64::NAN,
        other => todo!("{other:?}"),
    }
}

fn make_primitive(e: &mut Expr) {
    e.deduce();

    if let Expr::Array(arr) = e {
        match arr.first_mut() {
            Some(ae) => {
                let s = stringify_expr(ae);
                *e = Expr::Literal(LiteralExpr::String(s));
            }
            None => {
                *e = Expr::string_literal(Default::default());
            }
        }
    }
}

impl<'a> Deduce for Expr<'a> {
    fn deduce(&mut self) {
        match self {
            Expr::Binary(b) => {
                assert_eq!(b.operator, TokenType::Plus);

                make_primitive(&mut b.left);
                make_primitive(&mut b.right);

                let lhs_string = matches!(&*b.left, Expr::Literal(LiteralExpr::String(_)));
                let rhs_string = matches!(&*b.right, Expr::Literal(LiteralExpr::String(_)));

                if lhs_string || rhs_string {
                    let lhs = stringify_expr(&mut b.left);
                    let rhs = stringify_expr(&mut b.right);

                    *self = Expr::Literal(LiteralExpr::String(lhs + rhs));
                } else {
                    let lhs = to_numeric(&mut b.left);
                    let rhs = to_numeric(&mut b.right);

                    *self = Expr::number_literal(lhs + rhs);
                }
            }
            Expr::Array(e) => e.as_mut_slice().deduce(),
            Expr::Unary(u) => {
                u.expr.deduce();

                match &*u.expr {
                    Expr::Array(arr) => {
                        assert!(arr.is_empty()); // TODO

                        match u.operator {
                            TokenType::LogicalNot => {
                                *self = Expr::bool_literal(false);
                            }
                            TokenType::Plus => {
                                *self = Expr::number_literal(0.0);
                            }
                            _ => todo!(),
                        }
                    }
                    Expr::Literal(lit) => match lit {
                        LiteralExpr::Boolean(boo) => match u.operator {
                            TokenType::LogicalNot => {
                                *self = Expr::bool_literal(!boo);
                            }
                            TokenType::Plus => {
                                *self = Expr::number_literal(if *boo { 1.0 } else { 0.0 });
                            }
                            _ => todo!(),
                        },
                        LiteralExpr::Number(num) => match u.operator {
                            TokenType::LogicalNot => {
                                *self = Expr::bool_literal(*num == 0.0);
                            }
                            TokenType::Plus => { /* nothing */ }
                            _ => todo!(),
                        },
                        LiteralExpr::String(st) => match u.operator {
                            TokenType::LogicalNot => {
                                *self = Expr::bool_literal(st.is_empty());
                            }
                            TokenType::Plus => {
                                let num = st.parse::<f64>().unwrap_or(f64::NAN);
                                *self = Expr::number_literal(num);
                            }
                            other => todo!("{other:?}"),
                        },
                        other => todo!("{other:?}"),
                    },
                    other => todo!("{other:?}"),
                }
            }
            Expr::Grouping(GroupingExpr(e)) => {
                assert_eq!(e.len(), 1);

                let x = stringify_expr(&mut e[0]);
                *self = Expr::Literal(LiteralExpr::String(x));
            }
            Expr::Literal(_) => { /* nothing */ }
            _ => todo!("{self:?}"),
        }
    }
}

impl<'a> Deduce for Statement<'a> {
    fn deduce(&mut self) {
        match self {
            Statement::Expression(e) => e.deduce(),
            _ => todo!(),
        }
    }
}
