use crate::model;
use crate::parsing;

use chumsky::prelude::*;
use model::ast::*;

use parsing::token::Token;

pub fn parser() -> impl Parser<Token, Module, Error = Simple<Token>> {
    module().then_ignore(end())
}

fn module() -> impl Parser<Token, Module, Error = Simple<Token>> {
    member().repeated().map(|members| Module { members })
}

fn member() -> impl Parser<Token, Member, Error = Simple<Token>> {
    func().map(Member::Func)
}

fn func() -> impl Parser<Token, Func, Error = Simple<Token>> {
    visibility()
        .then_ignore(just(Token::Fn))
        .then(ident())
        .then_ignore(just(Token::LParen))
        .then_ignore(just(Token::RParen))
        .then_ignore(just(Token::LBrace))
        .then(stmt().repeated().then(expr().or_not()))
        .then_ignore(just(Token::RBrace))
        .map(|((visibility, name), (body, tail))| Func {
            visibility,
            name,
            body,
            tail,
        })
}

fn stmt() -> impl Parser<Token, Stmt, Error = Simple<Token>> {
    choice((
        var_decl().map(Stmt::VarDecl),
        expr_stmt().map(Stmt::ExprStmt),
    ))
}

fn expr_stmt() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    expr().then_ignore(just(Token::SemiColon))
}

fn var_decl() -> impl Parser<Token, Decl, Error = Simple<Token>> {
    just(Token::Let)
        .ignore_then(
            just(Token::Mut)
                .to(true)
                .or_not()
                .map(|t| t.unwrap_or(false)),
        )
        .then(ident())
        .then(just(Token::Colon).ignore_then(ident()).or_not())
        .then(just(Token::Eq).ignore_then(expr()).or_not())
        .then_ignore(just(Token::SemiColon))
        .map(|(((mutable, name), type_annot), value)| Decl {
            mutable,
            name,
            type_annot,
            value,
        })
}

fn expr() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    recursive(|expr| {
        let expr = || expr.clone();

        choice((func_call(expr()), assign(expr()), var()))
    })
}

fn var() -> impl Parser<Token, Expr, Error = Simple<Token>> {
    ident().map(Expr::Var)
}

fn func_call(
    expr: impl Clone + Parser<Token, Expr, Error = Simple<Token>>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> {
    var()
        .or(expr
            .clone()
            .delimited_by(just(Token::LParen), just(Token::RParen)))
        .then(
            expr.separated_by(just(Token::Comma))
                .then_ignore(just(Token::Comma).or_not())
                .delimited_by(just(Token::LParen), just(Token::RParen)),
        )
        .map(|(func, args)| Expr::FuncCall {
            func: Box::new(func),
            args,
        })
}

fn assign(
    expr: impl Clone + Parser<Token, Expr, Error = Simple<Token>>,
) -> impl Parser<Token, Expr, Error = Simple<Token>> {
    var()
        .then_ignore(just(Token::Eq))
        .then(expr)
        .map(|(target, value)| Expr::Assign {
            target: Box::new(target),
            value: Box::new(value),
        })
}

fn ident() -> impl Parser<Token, String, Error = Simple<Token>> {
    filter_map(|span, token| match token {
        Token::Ident(ident) => Ok(ident),
        _ => Err(Simple::custom(span, "expected an identifier")),
    })
}

fn visibility() -> impl Parser<Token, Visibility, Error = Simple<Token>> {
    let public = just(Token::Pub).to(Visibility::Public);

    choice((public,))
        .or_not()
        .map(|vis| vis.unwrap_or(Visibility::Private))
}
