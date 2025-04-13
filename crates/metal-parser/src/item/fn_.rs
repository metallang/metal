// SPDX-License-Identifier: MIT

use metal_ast::{N, T};

use crate::block::parse_block;
use crate::common::{parse_expr_specifier, parse_mutability, parse_name, parse_type_qualifier};
use crate::generics::utils::parse_name_generics;

pub fn parse_fn_item(parser: &mut crate::parser::Parser) {
    parser.start_node(N![FnItem]);

    parser.maybe_eat(T![def]);
    parse_name_generics(parser);
    parse_fn_signature(parser);
    parse_block(parser);

    parser.end_node();
}

pub fn parse_fn_signature(parser: &mut crate::parser::Parser) {
    parser.start_node(N![FnSignature]);

    parser.maybe_eat(T!['(']);
    parse_fn_inputs(parser);
    parser.maybe_eat(T![')']);

    parse_type_qualifier(parser);

    parser.end_node();
}

pub fn parse_fn_inputs(parser: &mut crate::parser::Parser) {
    parser.start_node(N![FnInputs]);

    while parser.is_not_at_eof_or(T![')']) {
        parse_fn_input(parser);
        parser.maybe_eat(T![,]);
    }

    parser.end_node();
}

pub fn parse_fn_input(parser: &mut crate::parser::Parser) {
    parser.start_node(N![FnInput]);

    parse_fn_input_modifiers(parser);
    parse_name(parser);
    parse_type_qualifier(parser);
    parse_expr_specifier(parser);

    parser.end_node();
}

pub fn parse_fn_input_modifiers(parser: &mut crate::parser::Parser) {
    parser.start_node(N![FnInputModifiers]);

    while parser.is_not_at_eof_or(T![@ident]) {
        parse_fn_input_modifier(parser);
    }

    parser.end_node();
}

pub fn parse_fn_input_modifier(parser: &mut crate::parser::Parser) {
    parser.start_node(N![FnInputModifier]);

    match parser.peek(0).unwrap().kind {
        T![mut] => parse_mutability(parser),
        T![capture] => parse_fn_input_modifier_capture(parser),
        other => todo!("{other:#?}"),
    };

    parser.end_node();
}

pub fn parse_fn_input_modifier_capture(parser: &mut crate::parser::Parser) {
    parser.start_node(N![FnInputModifierCapture]);

    parser.maybe_eat(T![capture]);
    parser.maybe_eat(T!['(']);
    parse_fn_input_modifier_capture_kind(parser);
    parser.maybe_eat(T![')']);

    parser.end_node();
}

pub fn parse_fn_input_modifier_capture_kind(parser: &mut crate::parser::Parser) {
    parser.start_node(N![FnInputModifierCaptureKind]);

    match parser.peek(0).unwrap().kind {
        T![owned] => parser.eat_any(),
        T![ref] => parser.eat_any(),
        T![mut] => parser.eat_any(),
        other => todo!("{other:#?}"),
    }

    parser.end_node();
}
