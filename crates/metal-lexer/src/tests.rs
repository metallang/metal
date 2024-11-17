// Copyright (c) Metal contributors
// Licensed under the MIT License. See LICENSE file in the project root for details.

use crate::TokenKind;

macro_rules! test_eq {
    ($input: expr, $($expected: expr),*) => {
        {
            let lexed = $crate::lex($input);
            let mut lexemes = vec![];

            for token in lexed {
                lexemes.push(token.map(|t| t.kind));
            }

            assert_eq!(lexemes, vec![$($expected),*]);
        }
    };
}

#[test]
fn test_identifier() {
    test_eq!("x", Ok(TokenKind::Ident("x")));
    test_eq!("xyz", Ok(TokenKind::Ident("xyz")));
    test_eq!("XYZ", Ok(TokenKind::Ident("XYZ")));
    test_eq!("X1", Ok(TokenKind::Ident("X1")));
    test_eq!("X1X", Ok(TokenKind::Ident("X1X")));
    test_eq!("X_", Ok(TokenKind::Ident("X_")));
    test_eq!("X_X", Ok(TokenKind::Ident("X_X")));
    test_eq!("_X", Ok(TokenKind::Ident("_X")));
    test_eq!("X1_", Ok(TokenKind::Ident("X1_")));
    test_eq!("X_1", Ok(TokenKind::Ident("X_1")));
    test_eq!("X_1X", Ok(TokenKind::Ident("X_1X")));
    test_eq!("X1_X", Ok(TokenKind::Ident("X1_X")));
    test_eq!("X__X", Ok(TokenKind::Ident("X__X")));
    test_eq!("X_X1", Ok(TokenKind::Ident("X_X1")));
    test_eq!("你", Ok(TokenKind::Ident("你")));
    test_eq!("你好", Ok(TokenKind::Ident("你好")));
    test_eq!("你1", Ok(TokenKind::Ident("你1")));
    test_eq!("你1你", Ok(TokenKind::Ident("你1你")));
    test_eq!("你_", Ok(TokenKind::Ident("你_")));
    test_eq!("你_你", Ok(TokenKind::Ident("你_你")));
    test_eq!("_你", Ok(TokenKind::Ident("_你")));
    test_eq!("你1_", Ok(TokenKind::Ident("你1_")));
    test_eq!("你_1", Ok(TokenKind::Ident("你_1")));
    test_eq!("你_1你", Ok(TokenKind::Ident("你_1你")));
    test_eq!("你1_你", Ok(TokenKind::Ident("你1_你")));
    test_eq!("你__你", Ok(TokenKind::Ident("你__你")));
    test_eq!("你_你1", Ok(TokenKind::Ident("你_你1")));
    test_eq!("п", Ok(TokenKind::Ident("п")));
    test_eq!("привет", Ok(TokenKind::Ident("привет")));
    test_eq!("ПРИВЕТ", Ok(TokenKind::Ident("ПРИВЕТ")));
    test_eq!("П1", Ok(TokenKind::Ident("П1")));
    test_eq!("П1П", Ok(TokenKind::Ident("П1П")));
    test_eq!("П_", Ok(TokenKind::Ident("П_")));
    test_eq!("П_П", Ok(TokenKind::Ident("П_П")));
    test_eq!("_П", Ok(TokenKind::Ident("_П")));
    test_eq!("П1_", Ok(TokenKind::Ident("П1_")));
    test_eq!("П_1", Ok(TokenKind::Ident("П_1")));
    test_eq!("П_1П", Ok(TokenKind::Ident("П_1П")));
    test_eq!("П1_П", Ok(TokenKind::Ident("П1_П")));
    test_eq!("П__П", Ok(TokenKind::Ident("П__П")));
    test_eq!("П_П1", Ok(TokenKind::Ident("П_П1")));
}

#[test]
fn test_integer() {
    test_eq!("0", Ok(TokenKind::NumLit(0)));
    test_eq!("1", Ok(TokenKind::NumLit(1)));
    test_eq!("01", Ok(TokenKind::NumLit(1)));
    test_eq!("001", Ok(TokenKind::NumLit(1)));
    test_eq!("10", Ok(TokenKind::NumLit(10)));
    test_eq!("100", Ok(TokenKind::NumLit(100)));
    test_eq!("101", Ok(TokenKind::NumLit(101)));
}

#[test]
fn test_binaryinteger() {
    test_eq!("0b0", Ok(TokenKind::BinaryNumLit(0)));
    test_eq!("0b1", Ok(TokenKind::BinaryNumLit(1)));
    test_eq!("0b01", Ok(TokenKind::BinaryNumLit(1)));
    test_eq!("0b001", Ok(TokenKind::BinaryNumLit(1)));
    test_eq!("0b10", Ok(TokenKind::BinaryNumLit(2)));
    test_eq!("0b100", Ok(TokenKind::BinaryNumLit(4)));
    test_eq!("0b101", Ok(TokenKind::BinaryNumLit(5)));
}

#[test]
fn test_hexinteger() {
    test_eq!("0x0", Ok(TokenKind::HexNumLit(0)));
    test_eq!("0x1", Ok(TokenKind::HexNumLit(1)));
    test_eq!("0x01", Ok(TokenKind::HexNumLit(1)));
    test_eq!("0x10", Ok(TokenKind::HexNumLit(16)));
    test_eq!("0x123", Ok(TokenKind::HexNumLit(291)));
    test_eq!("0xffffff", Ok(TokenKind::HexNumLit(16777215)));
    test_eq!("0xf62fda", Ok(TokenKind::HexNumLit(16134106)));
    test_eq!("0xf62fdaa", Ok(TokenKind::HexNumLit(258145706)));
}

#[test]
fn test_scientificfloat() {
    test_eq!("1.5e+0", Ok(TokenKind::EFloatLit(1.5)));
    test_eq!("1.5e+1", Ok(TokenKind::EFloatLit(15.0)));
    test_eq!("1.5e+2", Ok(TokenKind::EFloatLit(150.0)));
    test_eq!("1.5e-0", Ok(TokenKind::EFloatLit(1.5)));
    test_eq!("1.5e-1", Ok(TokenKind::EFloatLit(0.15)));
    test_eq!("1.5e-2", Ok(TokenKind::EFloatLit(0.015)));
}

#[test]
fn test_float() {
    test_eq!("0.0", Ok(TokenKind::FloatLit(0.0)));
    test_eq!("0.1", Ok(TokenKind::FloatLit(0.1)));
    test_eq!("0.01", Ok(TokenKind::FloatLit(0.01)));
    test_eq!("1.0", Ok(TokenKind::FloatLit(1.0)));
    test_eq!("10.0", Ok(TokenKind::FloatLit(10.0)));
    test_eq!("100.0", Ok(TokenKind::FloatLit(100.0)));
}

#[test]
fn test_number_underscores() {
    test_eq!("0_1", Ok(TokenKind::NumLit(1)));
    test_eq!("0_0_1", Ok(TokenKind::NumLit(1)));
    test_eq!("1_0", Ok(TokenKind::NumLit(10)));
    test_eq!("1_00", Ok(TokenKind::NumLit(100)));
    test_eq!("10_1", Ok(TokenKind::NumLit(101)));

    test_eq!("0b0_1", Ok(TokenKind::BinaryNumLit(1)));
    test_eq!("0b0_0_1", Ok(TokenKind::BinaryNumLit(1)));
    test_eq!("0b1_0", Ok(TokenKind::BinaryNumLit(2)));
    test_eq!("0b1_00", Ok(TokenKind::BinaryNumLit(4)));
    test_eq!("0b10_1", Ok(TokenKind::BinaryNumLit(5)));

    test_eq!("0x0_1", Ok(TokenKind::HexNumLit(1)));
    test_eq!("0x1_0", Ok(TokenKind::HexNumLit(16)));
    test_eq!("0x1_2_3", Ok(TokenKind::HexNumLit(291)));
    test_eq!("0xff_ff_ff", Ok(TokenKind::HexNumLit(16777215)));
    test_eq!("0xf_62_f_da", Ok(TokenKind::HexNumLit(16134106)));
    test_eq!("0xf6_2f_d_a_a", Ok(TokenKind::HexNumLit(258145706)));

    test_eq!("1.5e+0", Ok(TokenKind::EFloatLit(1.5)));
    test_eq!("1_5.0e+1", Ok(TokenKind::EFloatLit(150.0)));
    test_eq!("15.0_0e+1", Ok(TokenKind::EFloatLit(150.0)));
    test_eq!("1.5e-0_0", Ok(TokenKind::EFloatLit(1.5)));
    test_eq!("1.5e-0_1", Ok(TokenKind::EFloatLit(0.15)));
    test_eq!("15.0e-0_2", Ok(TokenKind::EFloatLit(0.15)));

    test_eq!("0.0_1", Ok(TokenKind::FloatLit(0.01)));
    test_eq!("1_0.0", Ok(TokenKind::FloatLit(10.0)));
    test_eq!("1_0_0.0", Ok(TokenKind::FloatLit(100.0)));
    test_eq!("10.0_0", Ok(TokenKind::FloatLit(10.0)));
    test_eq!("1_00.0", Ok(TokenKind::FloatLit(100.0)));
    test_eq!("10_0.0", Ok(TokenKind::FloatLit(100.0)));
}

#[test]
fn test_string() {
    test_eq!(r#""hello""#, Ok(TokenKind::StrLit("hello")));
    test_eq!(r#""你好""#, Ok(TokenKind::StrLit("你好")));
    test_eq!(r#""привет""#, Ok(TokenKind::StrLit("привет")));
    test_eq!(
        r#""~!@#$%^&*()_+-=`|""#,
        Ok(TokenKind::StrLit("~!@#$%^&*()_+-=`|"))
    );
    test_eq!(r#""\r\n\f\\""#, Ok(TokenKind::StrLit(r"\r\n\f\\"))); // TODO: unescape strings
    test_eq!(r#""""#, Ok(TokenKind::StrLit("")));
}

#[test]
fn test_boolean() {
    test_eq!("true", Ok(TokenKind::BoolLit(true)));
    test_eq!("false", Ok(TokenKind::BoolLit(false)));
}

#[test]
fn test_comparison() {
    test_eq!("<", Ok(TokenKind::LessThan));
    test_eq!("<=", Ok(TokenKind::LessThanOrEqual));
    test_eq!(">", Ok(TokenKind::GreaterThan));
    test_eq!(">=", Ok(TokenKind::GreaterThanOrEqual));
    test_eq!("==", Ok(TokenKind::Equal));
    test_eq!("!=", Ok(TokenKind::NotEqual));
}

#[test]
fn test_logical() {
    test_eq!("&&", Ok(TokenKind::And));
    test_eq!("||", Ok(TokenKind::Or));
    test_eq!("!", Ok(TokenKind::Not));
}

#[test]
fn test_bitwise() {
    test_eq!("&", Ok(TokenKind::BitwiseAnd));
    test_eq!("|", Ok(TokenKind::BitwiseOr));
    test_eq!("~", Ok(TokenKind::BitwiseNot));
    test_eq!("^", Ok(TokenKind::BitwiseXor));
    test_eq!("<<", Ok(TokenKind::BitwiseShiftLeft));
    test_eq!(">>", Ok(TokenKind::BitwiseShiftRight));
}

#[test]
fn test_math() {
    test_eq!("+", Ok(TokenKind::Plus));
    test_eq!("-", Ok(TokenKind::Minus));
    test_eq!("*", Ok(TokenKind::Multiply));
    test_eq!("**", Ok(TokenKind::Power));
    test_eq!("/", Ok(TokenKind::Divide));
    test_eq!("%", Ok(TokenKind::Remainder));
}

#[test]
fn test_assignment() {
    test_eq!("=", Ok(TokenKind::Assignment));
    test_eq!("+=", Ok(TokenKind::PlusAssignment));
    test_eq!("-=", Ok(TokenKind::MinusAssignment));
    test_eq!("*=", Ok(TokenKind::MultiplyAssignment));
    test_eq!("**=", Ok(TokenKind::PowerAssignment));
    test_eq!("/=", Ok(TokenKind::DivideAssignment));
    test_eq!("%=", Ok(TokenKind::RemainderAssignment));
    test_eq!("&=", Ok(TokenKind::BitwiseAndAssignment));
    test_eq!("|=", Ok(TokenKind::BitwiseOrAssignment));
    test_eq!("~=", Ok(TokenKind::BitwiseNotAssignment));
    test_eq!("^=", Ok(TokenKind::BitwiseXorAssignment));
    test_eq!("<<=", Ok(TokenKind::BitwiseShiftLeftAssignment));
    test_eq!(">>=", Ok(TokenKind::BitwiseShiftRightAssignment));
}

#[test]
fn test_control_flow() {
    test_eq!("if", Ok(TokenKind::If));
    test_eq!("else", Ok(TokenKind::Else));
    test_eq!("for", Ok(TokenKind::For));
    test_eq!("while", Ok(TokenKind::While));
    test_eq!("break", Ok(TokenKind::Break));
    test_eq!("continue", Ok(TokenKind::Continue));
    test_eq!("return", Ok(TokenKind::Return));
    test_eq!("switch", Ok(TokenKind::Switch));
}

#[test]
fn test_keywords() {
    test_eq!("const", Ok(TokenKind::Const));
    test_eq!("def", Ok(TokenKind::Def));
    test_eq!("defer", Ok(TokenKind::Defer));
    test_eq!("enum", Ok(TokenKind::Enum));
    test_eq!("import", Ok(TokenKind::Import));
    test_eq!("let", Ok(TokenKind::Let));
    test_eq!("mut", Ok(TokenKind::Mut));
    test_eq!("pub", Ok(TokenKind::Pub));
    test_eq!("struct", Ok(TokenKind::Struct));
    test_eq!("type", Ok(TokenKind::Type));
}

#[test]
fn test_punctuation() {
    test_eq!("@", Ok(TokenKind::Annotation));
    test_eq!("(", Ok(TokenKind::OpeningParenthesis));
    test_eq!(")", Ok(TokenKind::ClosingParenthesis));
    test_eq!("[", Ok(TokenKind::OpeningBracket));
    test_eq!("]", Ok(TokenKind::ClosingBracket));
    test_eq!("{", Ok(TokenKind::OpeningBrace));
    test_eq!("}", Ok(TokenKind::ClosingBrace));
    test_eq!(",", Ok(TokenKind::Comma));
    test_eq!(";", Ok(TokenKind::Semicolon));
    test_eq!(".", Ok(TokenKind::Period));
    test_eq!("..", Ok(TokenKind::Range));
    test_eq!(":", Ok(TokenKind::Colon));
}
