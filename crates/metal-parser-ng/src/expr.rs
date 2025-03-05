use metal_ast_ng::SyntaxKind;
use metal_ast_ng::T;

use crate::common::parse_name;

mod binary;

pub fn parse_expr(parser: &mut crate::parser::parser_type!()) {
    let at = parser.checkpoint();

    parser.start_node(SyntaxKind::EXPR_NODE);

    match parser.peek().expect("expected an expr").kind {
        SyntaxKind::LIT_IDENT_TOKEN => parse_name(parser),
        SyntaxKind::LIT_NUM_TOKEN | SyntaxKind::LIT_STR_TOKEN => {
            parser.start_node(SyntaxKind::LIT_EXPR_NODE);

            if parser.peek_is(SyntaxKind::LIT_NUM_TOKEN)
                || parser.peek_is(SyntaxKind::LIT_STR_TOKEN)
            {
                parser.eat_any();
            }

            parser.end_node();
        }
        other => todo!("{other:#?}"),
    }

    parser.end_node();

    if parser.peek_is(SyntaxKind::L_PAREN_TOKEN) {
        parser.start_node_at(SyntaxKind::CALL_EXPR_NODE, at);

        parser.eat_any();

        if !parser.peek_is(SyntaxKind::R_PAREN_TOKEN) {
            parser.start_node(SyntaxKind::CALL_EXPR_ARGS_NODE);

            while !(parser.peek_is(SyntaxKind::R_PAREN_TOKEN) || parser.is_eof()) {
                parse_expr(parser);
                parser.maybe_eat(SyntaxKind::COMMA_TOKEN);
            }

            parser.end_node();
        }

        parser.maybe_eat(SyntaxKind::R_PAREN_TOKEN);

        parser.end_node();
    }

    if parser.peek().is_some_and(|t| is_binary_expr_op(t.kind)) {
        parser.start_node_at(SyntaxKind::BINARY_EXPR_NODE, at);

        parser.eat_any();

        parse_expr(parser);

        parser.end_node();
    }
}

fn is_binary_expr_op(kind: SyntaxKind) -> bool {
    // assignment
    kind == T![=]
    || kind == T![+=]
    || kind == T![-=]
    || kind == T![/=]
    || kind == T![*=]
    || kind == T![**=]
    || kind == T![%=]
    || kind == T![^=]
    || kind == T![&=]
    || kind == T![|=]
    || kind == T![<<=]
    || kind == T![>>=]
    // math
    || kind == T![+]
    || kind == T![-]
    || kind == T![/]
    || kind == T![*]
    || kind == T![**]
    || kind == T![%]
    // logic
    || kind == T![&&]
    || kind == T![||]
    || kind == T![==]
    || kind == T![!=]
    // comparison
    || kind == T![>]
    || kind == T![>=]
    || kind == T![<]
    || kind == T![<=]
    // bitwise
    || kind == T![^]
    || kind == T![&]
    || kind == T![|]
    || kind == T![<<]
    || kind == T![>>]
    // range
    || kind == T![..]
    // special
    || kind == T![.]
}
