use metal_lexer::TokenKind;

pub enum Delimiter {
    /// Corresponds to [TokenKind::Semicolon](metal_lexer::TokenKind::Semicolon).
    Semicolon,
    /// Corresponds to [TokenKind::Comma](metal_lexer::TokenKind::Comma).
    Comma,
    /// Corresponds to [TokenKind::ClosingParenthesis](metal_lexer::TokenKind::ClosingParenthesis).
    ClosingParenthesis,
    /// Corresponds to [TokenKind::ClosingBrace](metal_lexer::TokenKind::ClosingBrace).
    ClosingBrace,
}

impl Delimiter {
    pub fn same_as(&self, kind: &TokenKind<'_>) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match (self, kind) {
            (Self::Semicolon, TokenKind::Semicolon) => true,
            (Self::Comma, TokenKind::Comma) => true,
            (Self::ClosingParenthesis, TokenKind::ClosingParenthesis) => true,
            (Self::ClosingBrace, TokenKind::ClosingBrace) => true,
            _ => false,
        }
    }
}
