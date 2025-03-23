use metal_ast::{N, T};

pub fn parse_lit_expr(parser: &mut crate::parser::parser_type!()) {
    parser.start_node(N![LitExpr]);

    assert!(parser
        .peek()
        .is_some_and(|token| token.kind == T![@number] || token.kind == T![@string]));

    parser.eat_any();

    parser.end_node();
}
