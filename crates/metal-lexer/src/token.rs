// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use logos::Logos;

use crate::callbacks::{convert, convert_radix, trim_quotes};
use crate::error::Error;
use crate::Span;

/// A lexical unit of Metal code.
#[derive(Debug)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub span: Span,
}

// FIXME: Switch to a manual implementation. logos, even with our fixes, is not fully sound.

#[rustfmt::skip]
#[derive(Logos, Debug, PartialEq, enum_as_inner::EnumAsInner)]
#[logos(error = Error)]
#[logos(skip " \t\r\n")]
#[logos(subpattern int = r"[0-9](_?[0-9])*_?")]
#[logos(subpattern float = r"(?&int)\.(?&int)")]
#[logos(subpattern string = "(\"[^\"\r\n]*\")|('[^'\r\n]*')")]
pub enum TokenKind<'src> {
    /* Literals */

    #[regex(r"(\p{XID_Start}|_)\p{XID_Continue}*")]
    Ident(&'src str),

    #[regex("(?&int)", convert)]
    NumLit(u64),

    #[regex("0b[01](_?[01])*", convert_radix::<2>)]
    BinaryNumLit(u64),

    #[regex("0x[0-9abcdefABCDEF](_?[0-9abcdefABCDEF])*", convert_radix::<16>)]
    HexNumLit(u64),

    #[regex(r"(?&float)", convert)]
    FloatLit(f64),

    #[regex(r"(?&float)[eE][+-]?(?&int)", convert)]
    EFloatLit(f64),

    #[regex("(?&string)", trim_quotes::<false>)]
    StrLit(&'src str),

    #[regex("true|false", convert)]
    BoolLit(bool),

    /* Comparison */

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanOrEqual,

    #[token(">")]
    GreaterThan,

    #[token(">=")]
    GreaterThanOrEqual,

    #[token("==")]
    Equal,

    #[token("!=")]
    NotEqual,

    /* Logical */

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("!")]
    Not,

    /* Bitwise operators */

    #[token("&")]
    BitwiseAnd,

    #[token("|")]
    BitwiseOr,

    #[token("~")]
    BitwiseNot,

    #[token("^")]
    BitwiseXor,

    #[token("<<")]
    BitwiseShiftLeft,

    #[token(">>")]
    BitwiseShiftRight,

    /* Math */

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("**")]
    Power,

    #[token("/")]
    Divide,

    #[token("%")]
    Remainder,

    /* Assignment */

    #[token("=")]
    Assignment,

    #[token("+=")]
    PlusAssignment,

    #[token("-=")]
    MinusAssignment,

    #[token("*=")]
    MultiplyAssignment,

    #[token("**=")]
    PowerAssignment,

    #[token("/=")]
    DivideAssignment,

    #[token("%=")]
    RemainderAssignment,

    #[token("&=")]
    BitwiseAndAssignment,

    #[token("|=")]
    BitwiseOrAssignment,

    #[token("~=")]
    BitwiseNotAssignment,

    #[token("^=")]
    BitwiseXorAssignment,

    #[token("<<=")]
    BitwiseShiftLeftAssignment,

    #[token(">>=")]
    BitwiseShiftRightAssignment,

    /* Control flow */

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("for")]
    For,

    #[token("while")]
    While,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("return")]
    Return,

    #[token("switch")]
    Switch,

    /* Keywords */

    #[token("const")]
    Const,

    #[token("def")]
    Def,

    #[token("defer")]
    Defer,

    #[token("enum")]
    Enum,

    #[token("import")]
    Import,

    #[token("let")]
    Let,

    #[token("mut")]
    Mut,

    #[token("pub")]
    Pub,

    #[token("struct")]
    Struct,

    #[token("type")]
    Type,

    /* Punctuation */

    #[regex("@")]
    Annotation,

    #[token("(")]
    OpeningParenthesis,

    #[token(")")]
    ClosingParenthesis,

    #[token("[")]
    OpeningBracket,

    #[token("]")]
    ClosingBracket,

    #[token("{")]
    OpeningBrace,

    #[token("}")]
    ClosingBrace,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token(".")]
    Period,

    #[token("..")]
    Range,

    #[token(":")]
    Colon,

    /* Specials */

    #[regex("//[^\n]*")]
    Comment(&'src str),
}
