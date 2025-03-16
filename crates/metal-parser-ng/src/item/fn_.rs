use metal_ast_ng::N;
use metal_ast_ng::T;

use crate::block::parse_block;
use crate::common::parse_expr_specifier;
use crate::common::parse_mutability;
use crate::common::parse_name;
use crate::common::parse_type_qualifier;

pub fn parse_fn_item(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![FnItem]);

    parser.maybe_eat(T![def]);
    parse_name(parser);
    parse_fn_signature(parser);
    parse_block(parser);

    parser.end_node();
}

pub fn parse_fn_signature(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![FnSignature]);

    parser.maybe_eat(T!['(']);
    parse_fn_inputs(parser);
    parser.maybe_eat(T![')']);

    parse_type_qualifier(parser);

    parser.end_node();
}

pub fn parse_fn_inputs(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![FnInputs]);

    while !(parser.peek_is(T![')']) || parser.is_eof()) {
        parse_fn_input(parser);
        parser.maybe_eat(T![,]);
    }

    parser.end_node();
}

pub fn parse_fn_input(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![FnInput]);

    parse_mutability(parser);
    parse_name(parser);
    parse_type_qualifier(parser);
    parse_expr_specifier(parser);

    parser.end_node();
}
