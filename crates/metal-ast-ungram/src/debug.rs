// SPDX-License-Identifier: MIT

use std::fmt::Debug;

use ungrammar::{NodeData, Rule, TokenData};

use crate::engram::Engram;

/// Returns a better [Debug] implementation for [Rule].
#[allow(dead_code)]
pub fn debug_rule<'a>(grammar: &'a Engram, rule: &'a Rule) -> impl Debug + 'a {
    DebugRule { grammar, rule }
}

fn debug_multiple_rules<'a>(grammar: &'a Engram, rules: &'a Vec<Rule>) -> DebugMultipleRules<'a> {
    DebugMultipleRules { grammar, rules }
}

fn debug_token_data(data: &TokenData) -> DebugTokenData<'_> {
    DebugTokenData(data)
}

fn debug_node_data(data: &NodeData) -> DebugNodeData<'_> {
    DebugNodeData(data)
}

struct DebugTokenData<'a>(&'a TokenData);

impl Debug for DebugTokenData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.name.fmt(f)
    }
}

struct DebugNodeData<'a>(&'a NodeData);

impl Debug for DebugNodeData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.name.fmt(f)
    }
}

struct DebugMultipleRules<'a> {
    grammar: &'a Engram,
    rules: &'a Vec<Rule>,
}

impl Debug for DebugMultipleRules<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.rules.iter().map(|rule| debug_rule(self.grammar, rule)))
            .finish()
    }
}

struct DebugRule<'a> {
    grammar: &'a Engram,
    rule: &'a Rule,
}

impl Debug for DebugRule<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.rule {
            Rule::Labeled { label, rule } => f
                .debug_struct("Rule::Labeled")
                .field("label", label)
                .field("rule", &debug_rule(self.grammar, rule.as_ref()))
                .finish(),
            Rule::Seq(rules) => f
                .debug_tuple("Rule::Seq")
                .field(&debug_multiple_rules(self.grammar, rules))
                .finish(),
            Rule::Alt(rules) => f
                .debug_tuple("Rule::Alt")
                .field(&debug_multiple_rules(self.grammar, rules))
                .finish(),
            Rule::Opt(rule) => f
                .debug_tuple("Rule::Opt")
                .field(&debug_rule(self.grammar, rule.as_ref()))
                .finish(),
            Rule::Rep(rule) => f
                .debug_tuple("Rule::Rep")
                .field(&debug_rule(self.grammar, rule.as_ref()))
                .finish(),
            Rule::Node(node) => f
                .debug_tuple("Rule::Node")
                .field(&debug_node_data(&self.grammar[node]))
                .finish(),
            Rule::Token(token) => f
                .debug_tuple("Rule::Token")
                .field(&debug_token_data(&self.grammar[token]))
                .finish(),
        }
    }
}
