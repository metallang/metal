// SPDX-License-Identifier: MIT

//! Various abstractions over [ungrammar]'s API.

use std::ops::Index;

use ungrammar::{Grammar, Node, NodeData, Token, TokenData};

/// A convenience wrapper around [Grammar].
pub struct Engram(Grammar);

impl Engram {
    /// Returns an iterator over the inner grammar's tokens.
    pub fn tokens(&self) -> impl Iterator<Item = &TokenData> {
        self.0.tokens().map(|token| &self.0[token])
    }

    /// Returns an iterator over the inner grammar's nodes.
    pub fn nodes(&self) -> impl Iterator<Item = &NodeData> {
        self.0.iter().map(|node| &self.0[node])
    }
}

impl From<Grammar> for Engram {
    fn from(value: Grammar) -> Self {
        Self(value)
    }
}

impl Index<&Token> for Engram {
    type Output = TokenData;

    fn index(&self, index: &Token) -> &Self::Output {
        &self.0[*index]
    }
}

impl Index<&Node> for Engram {
    type Output = NodeData;

    fn index(&self, index: &Node) -> &Self::Output {
        &self.0[*index]
    }
}
