use std::borrow::Cow;
use std::fmt::Display;
use std::str;

use dash_core::parser::expr::BinaryExpr;
use dash_core::parser::expr::Expr;
use dash_core::parser::expr::GroupingExpr;
use dash_core::parser::expr::LiteralExpr;
use dash_core::parser::expr::UnaryExpr;
use dash_core::parser::statement::Statement;
use dash_core::parser::token::TokenType;
use itertools::Either;
use itertools::Itertools;

pub trait Serialize<'a> {
    type Output;
    fn serialize(&self) -> Self::Output;
}

fn serialize_group<'a, S>(exprs: &[S]) -> String
where
    S: Serialize<'a>,
    S::Output: Display,
{
    exprs.iter().map(|e| e.serialize()).join(",")
}

impl<'a> Serialize<'a> for TokenType {
    type Output = Either<char, &'static str>;

    fn serialize(&self) -> Self::Output {
        match self {
            TokenType::Plus => Either::Left('+'),
            TokenType::LogicalNot => Either::Left('!'),
            _ => todo!(),
        }
    }
}

impl<'a> Serialize<'a> for BinaryExpr<'a> {
    type Output = String;

    fn serialize(&self) -> Self::Output {
        format!(
            "{}{}{}",
            self.left.serialize(),
            self.operator.serialize(),
            self.right.serialize()
        )
    }
}

impl<'a> Serialize<'a> for UnaryExpr<'a> {
    type Output = String;

    fn serialize(&self) -> Self::Output {
        format!("{}{}", self.operator.serialize(), self.expr.serialize())
    }
}

impl<'a> Serialize<'a> for LiteralExpr<'a> {
    type Output = Cow<'a, str>;

    fn serialize(&self) -> Self::Output {
        match self {
            LiteralExpr::Boolean(b) => Cow::Borrowed(if *b { "true" } else { "false" }),
            LiteralExpr::Number(n) => Cow::Owned(format!("{}", n)),
            LiteralExpr::Identifier(i) => i.to_owned(),
            LiteralExpr::String(s) => s.to_owned(),
            LiteralExpr::Null => Cow::Borrowed("null"),
            LiteralExpr::Undefined => Cow::Borrowed("undefined"),
        }
    }
}

impl<'a> Serialize<'a> for Expr<'a> {
    type Output = Cow<'a, str>;

    fn serialize(&self) -> Self::Output {
        match self {
            Expr::Binary(b) => Cow::Owned(b.serialize()),
            Expr::Array(e) => Cow::Owned(format!("[{}]", serialize_group(e))),
            Expr::Unary(u) => Cow::Owned(u.serialize()),
            Expr::Literal(lit) => lit.serialize(),
            Expr::Grouping(GroupingExpr(e)) => Cow::Owned(format!("({})", serialize_group(&e))),
            _ => todo!(),
        }
    }
}

impl<'a> Serialize<'a> for Statement<'a> {
    type Output = Cow<'a, str>;

    fn serialize(&self) -> Self::Output {
        match self {
            Statement::Expression(e) => Cow::Owned(format!("{};", e.serialize())),
            _ => todo!(),
        }
    }
}
