// SPDX-License-Identifier: MIT

//! This file is @generated by the build script, do not edit by hand.
use crate::{AstToken, SyntaxKind, SyntaxToken};
/// Represents the `{` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LBraceToken {
    syntax: SyntaxToken,
}
impl AstToken for LBraceToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::L_BRACE_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `}` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RBraceToken {
    syntax: SyntaxToken,
}
impl AstToken for RBraceToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::R_BRACE_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `@ident` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LitIdentToken {
    syntax: SyntaxToken,
}
impl AstToken for LitIdentToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LIT_IDENT_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `pub` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PubToken {
    syntax: SyntaxToken,
}
impl AstToken for PubToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PUB_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `mut` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutToken {
    syntax: SyntaxToken,
}
impl AstToken for MutToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MUT_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EqToken {
    syntax: SyntaxToken,
}
impl AstToken for EqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `:` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColonToken {
    syntax: SyntaxToken,
}
impl AstToken for ColonToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COLON_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `abstract` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractToken {
    syntax: SyntaxToken,
}
impl AstToken for AbstractToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ABSTRACT_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `def` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DefToken {
    syntax: SyntaxToken,
}
impl AstToken for DefToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DEF_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `;` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemicolonToken {
    syntax: SyntaxToken,
}
impl AstToken for SemicolonToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SEMICOLON_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `const` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstToken {
    syntax: SyntaxToken,
}
impl AstToken for ConstToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CONST_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `enum` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumToken {
    syntax: SyntaxToken,
}
impl AstToken for EnumToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENUM_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `(` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LParenToken {
    syntax: SyntaxToken,
}
impl AstToken for LParenToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::L_PAREN_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `)` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RParenToken {
    syntax: SyntaxToken,
}
impl AstToken for RParenToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::R_PAREN_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `,` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommaToken {
    syntax: SyntaxToken,
}
impl AstToken for CommaToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::COMMA_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `import` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImportToken {
    syntax: SyntaxToken,
}
impl AstToken for ImportToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IMPORT_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `.` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DotToken {
    syntax: SyntaxToken,
}
impl AstToken for DotToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DOT_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `return` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnToken {
    syntax: SyntaxToken,
}
impl AstToken for ReturnToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::RETURN_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `struct` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructToken {
    syntax: SyntaxToken,
}
impl AstToken for StructToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STRUCT_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `type` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeToken {
    syntax: SyntaxToken,
}
impl AstToken for TypeToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TYPE_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `[` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LBracketToken {
    syntax: SyntaxToken,
}
impl AstToken for LBracketToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::L_BRACKET_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `]` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RBracketToken {
    syntax: SyntaxToken,
}
impl AstToken for RBracketToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::R_BRACKET_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `&` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AmpToken {
    syntax: SyntaxToken,
}
impl AstToken for AmpToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AMP_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `+` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlusToken {
    syntax: SyntaxToken,
}
impl AstToken for PlusToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PLUS_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `-` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MinusToken {
    syntax: SyntaxToken,
}
impl AstToken for MinusToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MINUS_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `!` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BangToken {
    syntax: SyntaxToken,
}
impl AstToken for BangToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BANG_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `~` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TildeToken {
    syntax: SyntaxToken,
}
impl AstToken for TildeToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TILDE_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `+=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlusEqToken {
    syntax: SyntaxToken,
}
impl AstToken for PlusEqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PLUS_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `-=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MinusEqToken {
    syntax: SyntaxToken,
}
impl AstToken for MinusEqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MINUS_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `/=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SlashEqToken {
    syntax: SyntaxToken,
}
impl AstToken for SlashEqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SLASH_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `*=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StarEqToken {
    syntax: SyntaxToken,
}
impl AstToken for StarEqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STAR_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `**=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Star2EqToken {
    syntax: SyntaxToken,
}
impl AstToken for Star2EqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STAR2_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `%=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PercentEqToken {
    syntax: SyntaxToken,
}
impl AstToken for PercentEqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PERCENT_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `^=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaretEqToken {
    syntax: SyntaxToken,
}
impl AstToken for CaretEqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CARET_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `&=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AmpEqToken {
    syntax: SyntaxToken,
}
impl AstToken for AmpEqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AMP_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `|=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipeEqToken {
    syntax: SyntaxToken,
}
impl AstToken for PipeEqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PIPE_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `<<=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lt2EqToken {
    syntax: SyntaxToken,
}
impl AstToken for Lt2EqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LT2_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `>>=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gt2EqToken {
    syntax: SyntaxToken,
}
impl AstToken for Gt2EqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GT2_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `/` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SlashToken {
    syntax: SyntaxToken,
}
impl AstToken for SlashToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::SLASH_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `*` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StarToken {
    syntax: SyntaxToken,
}
impl AstToken for StarToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STAR_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `**` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Star2Token {
    syntax: SyntaxToken,
}
impl AstToken for Star2Token {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STAR2_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `%` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PercentToken {
    syntax: SyntaxToken,
}
impl AstToken for PercentToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PERCENT_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `&&` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Amp2Token {
    syntax: SyntaxToken,
}
impl AstToken for Amp2Token {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::AMP2_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `||` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pipe2Token {
    syntax: SyntaxToken,
}
impl AstToken for Pipe2Token {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PIPE2_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `==` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Eq2Token {
    syntax: SyntaxToken,
}
impl AstToken for Eq2Token {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EQ2_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `!=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BangEqToken {
    syntax: SyntaxToken,
}
impl AstToken for BangEqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BANG_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `>` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GtToken {
    syntax: SyntaxToken,
}
impl AstToken for GtToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GT_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `>=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GtEqToken {
    syntax: SyntaxToken,
}
impl AstToken for GtEqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GT_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `<` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LtToken {
    syntax: SyntaxToken,
}
impl AstToken for LtToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LT_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `<=` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LtEqToken {
    syntax: SyntaxToken,
}
impl AstToken for LtEqToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LT_EQ_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `^` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CaretToken {
    syntax: SyntaxToken,
}
impl AstToken for CaretToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CARET_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `|` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipeToken {
    syntax: SyntaxToken,
}
impl AstToken for PipeToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PIPE_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `<<` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lt2Token {
    syntax: SyntaxToken,
}
impl AstToken for Lt2Token {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LT2_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `>>` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gt2Token {
    syntax: SyntaxToken,
}
impl AstToken for Gt2Token {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::GT2_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `..` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dot2Token {
    syntax: SyntaxToken,
}
impl AstToken for Dot2Token {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::DOT2_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `@number` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LitNumToken {
    syntax: SyntaxToken,
}
impl AstToken for LitNumToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LIT_NUM_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `@string` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LitStrToken {
    syntax: SyntaxToken,
}
impl AstToken for LitStrToken {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LIT_STR_TOKEN
    }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxToken {
        &self.syntax
    }
}
/// Represents the `LitExpr` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LitExprToken {
    /// See [LitNumToken].
    LitNum(LitNumToken),
    /// See [LitStrToken].
    LitStr(LitStrToken),
}
impl AstToken for LitExprToken {
    #[allow(clippy::match_like_matches_macro)]
    #[allow(clippy::wildcard_enum_match_arm)]
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::LIT_NUM_TOKEN => true,
            SyntaxKind::LIT_STR_TOKEN => true,
            _ => false,
        }
    }
    #[allow(clippy::wildcard_enum_match_arm)]
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::LIT_NUM_TOKEN => {
                Some(LitExprToken::LitNum(LitNumToken::cast(syntax)?))
            }
            SyntaxKind::LIT_STR_TOKEN => {
                Some(LitExprToken::LitStr(LitStrToken::cast(syntax)?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        match self {
            LitExprToken::LitNum(it) => it.syntax(),
            LitExprToken::LitStr(it) => it.syntax(),
        }
    }
}
/// Represents the `PrefixExprOp` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrefixExprOpToken {
    /// See [PlusToken].
    Plus(PlusToken),
    /// See [MinusToken].
    Minus(MinusToken),
    /// See [BangToken].
    Bang(BangToken),
    /// See [TildeToken].
    Tilde(TildeToken),
}
impl AstToken for PrefixExprOpToken {
    #[allow(clippy::match_like_matches_macro)]
    #[allow(clippy::wildcard_enum_match_arm)]
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::PLUS_TOKEN => true,
            SyntaxKind::MINUS_TOKEN => true,
            SyntaxKind::BANG_TOKEN => true,
            SyntaxKind::TILDE_TOKEN => true,
            _ => false,
        }
    }
    #[allow(clippy::wildcard_enum_match_arm)]
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::PLUS_TOKEN => {
                Some(PrefixExprOpToken::Plus(PlusToken::cast(syntax)?))
            }
            SyntaxKind::MINUS_TOKEN => {
                Some(PrefixExprOpToken::Minus(MinusToken::cast(syntax)?))
            }
            SyntaxKind::BANG_TOKEN => {
                Some(PrefixExprOpToken::Bang(BangToken::cast(syntax)?))
            }
            SyntaxKind::TILDE_TOKEN => {
                Some(PrefixExprOpToken::Tilde(TildeToken::cast(syntax)?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        match self {
            PrefixExprOpToken::Plus(it) => it.syntax(),
            PrefixExprOpToken::Minus(it) => it.syntax(),
            PrefixExprOpToken::Bang(it) => it.syntax(),
            PrefixExprOpToken::Tilde(it) => it.syntax(),
        }
    }
}
/// Represents the `BinaryExprOp` token.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryExprOpToken {
    /// See [EqToken].
    Eq(EqToken),
    /// See [PlusEqToken].
    PlusEq(PlusEqToken),
    /// See [MinusEqToken].
    MinusEq(MinusEqToken),
    /// See [SlashEqToken].
    SlashEq(SlashEqToken),
    /// See [StarEqToken].
    StarEq(StarEqToken),
    /// See [Star2EqToken].
    Star2Eq(Star2EqToken),
    /// See [PercentEqToken].
    PercentEq(PercentEqToken),
    /// See [CaretEqToken].
    CaretEq(CaretEqToken),
    /// See [AmpEqToken].
    AmpEq(AmpEqToken),
    /// See [PipeEqToken].
    PipeEq(PipeEqToken),
    /// See [Lt2EqToken].
    Lt2Eq(Lt2EqToken),
    /// See [Gt2EqToken].
    Gt2Eq(Gt2EqToken),
    /// See [PlusToken].
    Plus(PlusToken),
    /// See [MinusToken].
    Minus(MinusToken),
    /// See [SlashToken].
    Slash(SlashToken),
    /// See [StarToken].
    Star(StarToken),
    /// See [Star2Token].
    Star2(Star2Token),
    /// See [PercentToken].
    Percent(PercentToken),
    /// See [Amp2Token].
    Amp2(Amp2Token),
    /// See [Pipe2Token].
    Pipe2(Pipe2Token),
    /// See [Eq2Token].
    Eq2(Eq2Token),
    /// See [BangEqToken].
    BangEq(BangEqToken),
    /// See [GtToken].
    Gt(GtToken),
    /// See [GtEqToken].
    GtEq(GtEqToken),
    /// See [LtToken].
    Lt(LtToken),
    /// See [LtEqToken].
    LtEq(LtEqToken),
    /// See [CaretToken].
    Caret(CaretToken),
    /// See [AmpToken].
    Amp(AmpToken),
    /// See [PipeToken].
    Pipe(PipeToken),
    /// See [Lt2Token].
    Lt2(Lt2Token),
    /// See [Gt2Token].
    Gt2(Gt2Token),
    /// See [Dot2Token].
    Dot2(Dot2Token),
    /// See [DotToken].
    Dot(DotToken),
}
impl AstToken for BinaryExprOpToken {
    #[allow(clippy::match_like_matches_macro)]
    #[allow(clippy::wildcard_enum_match_arm)]
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::EQ_TOKEN => true,
            SyntaxKind::PLUS_EQ_TOKEN => true,
            SyntaxKind::MINUS_EQ_TOKEN => true,
            SyntaxKind::SLASH_EQ_TOKEN => true,
            SyntaxKind::STAR_EQ_TOKEN => true,
            SyntaxKind::STAR2_EQ_TOKEN => true,
            SyntaxKind::PERCENT_EQ_TOKEN => true,
            SyntaxKind::CARET_EQ_TOKEN => true,
            SyntaxKind::AMP_EQ_TOKEN => true,
            SyntaxKind::PIPE_EQ_TOKEN => true,
            SyntaxKind::LT2_EQ_TOKEN => true,
            SyntaxKind::GT2_EQ_TOKEN => true,
            SyntaxKind::PLUS_TOKEN => true,
            SyntaxKind::MINUS_TOKEN => true,
            SyntaxKind::SLASH_TOKEN => true,
            SyntaxKind::STAR_TOKEN => true,
            SyntaxKind::STAR2_TOKEN => true,
            SyntaxKind::PERCENT_TOKEN => true,
            SyntaxKind::AMP2_TOKEN => true,
            SyntaxKind::PIPE2_TOKEN => true,
            SyntaxKind::EQ2_TOKEN => true,
            SyntaxKind::BANG_EQ_TOKEN => true,
            SyntaxKind::GT_TOKEN => true,
            SyntaxKind::GT_EQ_TOKEN => true,
            SyntaxKind::LT_TOKEN => true,
            SyntaxKind::LT_EQ_TOKEN => true,
            SyntaxKind::CARET_TOKEN => true,
            SyntaxKind::AMP_TOKEN => true,
            SyntaxKind::PIPE_TOKEN => true,
            SyntaxKind::LT2_TOKEN => true,
            SyntaxKind::GT2_TOKEN => true,
            SyntaxKind::DOT2_TOKEN => true,
            SyntaxKind::DOT_TOKEN => true,
            _ => false,
        }
    }
    #[allow(clippy::wildcard_enum_match_arm)]
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::EQ_TOKEN => Some(BinaryExprOpToken::Eq(EqToken::cast(syntax)?)),
            SyntaxKind::PLUS_EQ_TOKEN => {
                Some(BinaryExprOpToken::PlusEq(PlusEqToken::cast(syntax)?))
            }
            SyntaxKind::MINUS_EQ_TOKEN => {
                Some(BinaryExprOpToken::MinusEq(MinusEqToken::cast(syntax)?))
            }
            SyntaxKind::SLASH_EQ_TOKEN => {
                Some(BinaryExprOpToken::SlashEq(SlashEqToken::cast(syntax)?))
            }
            SyntaxKind::STAR_EQ_TOKEN => {
                Some(BinaryExprOpToken::StarEq(StarEqToken::cast(syntax)?))
            }
            SyntaxKind::STAR2_EQ_TOKEN => {
                Some(BinaryExprOpToken::Star2Eq(Star2EqToken::cast(syntax)?))
            }
            SyntaxKind::PERCENT_EQ_TOKEN => {
                Some(BinaryExprOpToken::PercentEq(PercentEqToken::cast(syntax)?))
            }
            SyntaxKind::CARET_EQ_TOKEN => {
                Some(BinaryExprOpToken::CaretEq(CaretEqToken::cast(syntax)?))
            }
            SyntaxKind::AMP_EQ_TOKEN => {
                Some(BinaryExprOpToken::AmpEq(AmpEqToken::cast(syntax)?))
            }
            SyntaxKind::PIPE_EQ_TOKEN => {
                Some(BinaryExprOpToken::PipeEq(PipeEqToken::cast(syntax)?))
            }
            SyntaxKind::LT2_EQ_TOKEN => {
                Some(BinaryExprOpToken::Lt2Eq(Lt2EqToken::cast(syntax)?))
            }
            SyntaxKind::GT2_EQ_TOKEN => {
                Some(BinaryExprOpToken::Gt2Eq(Gt2EqToken::cast(syntax)?))
            }
            SyntaxKind::PLUS_TOKEN => {
                Some(BinaryExprOpToken::Plus(PlusToken::cast(syntax)?))
            }
            SyntaxKind::MINUS_TOKEN => {
                Some(BinaryExprOpToken::Minus(MinusToken::cast(syntax)?))
            }
            SyntaxKind::SLASH_TOKEN => {
                Some(BinaryExprOpToken::Slash(SlashToken::cast(syntax)?))
            }
            SyntaxKind::STAR_TOKEN => {
                Some(BinaryExprOpToken::Star(StarToken::cast(syntax)?))
            }
            SyntaxKind::STAR2_TOKEN => {
                Some(BinaryExprOpToken::Star2(Star2Token::cast(syntax)?))
            }
            SyntaxKind::PERCENT_TOKEN => {
                Some(BinaryExprOpToken::Percent(PercentToken::cast(syntax)?))
            }
            SyntaxKind::AMP2_TOKEN => {
                Some(BinaryExprOpToken::Amp2(Amp2Token::cast(syntax)?))
            }
            SyntaxKind::PIPE2_TOKEN => {
                Some(BinaryExprOpToken::Pipe2(Pipe2Token::cast(syntax)?))
            }
            SyntaxKind::EQ2_TOKEN => {
                Some(BinaryExprOpToken::Eq2(Eq2Token::cast(syntax)?))
            }
            SyntaxKind::BANG_EQ_TOKEN => {
                Some(BinaryExprOpToken::BangEq(BangEqToken::cast(syntax)?))
            }
            SyntaxKind::GT_TOKEN => Some(BinaryExprOpToken::Gt(GtToken::cast(syntax)?)),
            SyntaxKind::GT_EQ_TOKEN => {
                Some(BinaryExprOpToken::GtEq(GtEqToken::cast(syntax)?))
            }
            SyntaxKind::LT_TOKEN => Some(BinaryExprOpToken::Lt(LtToken::cast(syntax)?)),
            SyntaxKind::LT_EQ_TOKEN => {
                Some(BinaryExprOpToken::LtEq(LtEqToken::cast(syntax)?))
            }
            SyntaxKind::CARET_TOKEN => {
                Some(BinaryExprOpToken::Caret(CaretToken::cast(syntax)?))
            }
            SyntaxKind::AMP_TOKEN => {
                Some(BinaryExprOpToken::Amp(AmpToken::cast(syntax)?))
            }
            SyntaxKind::PIPE_TOKEN => {
                Some(BinaryExprOpToken::Pipe(PipeToken::cast(syntax)?))
            }
            SyntaxKind::LT2_TOKEN => {
                Some(BinaryExprOpToken::Lt2(Lt2Token::cast(syntax)?))
            }
            SyntaxKind::GT2_TOKEN => {
                Some(BinaryExprOpToken::Gt2(Gt2Token::cast(syntax)?))
            }
            SyntaxKind::DOT2_TOKEN => {
                Some(BinaryExprOpToken::Dot2(Dot2Token::cast(syntax)?))
            }
            SyntaxKind::DOT_TOKEN => {
                Some(BinaryExprOpToken::Dot(DotToken::cast(syntax)?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxToken {
        match self {
            BinaryExprOpToken::Eq(it) => it.syntax(),
            BinaryExprOpToken::PlusEq(it) => it.syntax(),
            BinaryExprOpToken::MinusEq(it) => it.syntax(),
            BinaryExprOpToken::SlashEq(it) => it.syntax(),
            BinaryExprOpToken::StarEq(it) => it.syntax(),
            BinaryExprOpToken::Star2Eq(it) => it.syntax(),
            BinaryExprOpToken::PercentEq(it) => it.syntax(),
            BinaryExprOpToken::CaretEq(it) => it.syntax(),
            BinaryExprOpToken::AmpEq(it) => it.syntax(),
            BinaryExprOpToken::PipeEq(it) => it.syntax(),
            BinaryExprOpToken::Lt2Eq(it) => it.syntax(),
            BinaryExprOpToken::Gt2Eq(it) => it.syntax(),
            BinaryExprOpToken::Plus(it) => it.syntax(),
            BinaryExprOpToken::Minus(it) => it.syntax(),
            BinaryExprOpToken::Slash(it) => it.syntax(),
            BinaryExprOpToken::Star(it) => it.syntax(),
            BinaryExprOpToken::Star2(it) => it.syntax(),
            BinaryExprOpToken::Percent(it) => it.syntax(),
            BinaryExprOpToken::Amp2(it) => it.syntax(),
            BinaryExprOpToken::Pipe2(it) => it.syntax(),
            BinaryExprOpToken::Eq2(it) => it.syntax(),
            BinaryExprOpToken::BangEq(it) => it.syntax(),
            BinaryExprOpToken::Gt(it) => it.syntax(),
            BinaryExprOpToken::GtEq(it) => it.syntax(),
            BinaryExprOpToken::Lt(it) => it.syntax(),
            BinaryExprOpToken::LtEq(it) => it.syntax(),
            BinaryExprOpToken::Caret(it) => it.syntax(),
            BinaryExprOpToken::Amp(it) => it.syntax(),
            BinaryExprOpToken::Pipe(it) => it.syntax(),
            BinaryExprOpToken::Lt2(it) => it.syntax(),
            BinaryExprOpToken::Gt2(it) => it.syntax(),
            BinaryExprOpToken::Dot2(it) => it.syntax(),
            BinaryExprOpToken::Dot(it) => it.syntax(),
        }
    }
}
