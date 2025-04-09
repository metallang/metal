// SPDX-License-Identifier: MIT

use metal_ast::{SyntaxKind, N, T};
use metal_lexer::{Span, Token};

use crate::block::parse_block;
use crate::common::parse_name;
use crate::expr::bp::{binding_power_for, BindingPower, Flavor};
use crate::expr::call::parse_call_expr;
use crate::expr::defer::parse_defer_expr;
use crate::expr::if_::parse_if_expr;
use crate::expr::let_::parse_let_expr;
use crate::expr::lit::parse_lit_expr;
use crate::expr::paren::parse_paren_expr;
use crate::expr::return_::parse_return_expr;
use crate::expr::struct_::parse_struct_expr;
use crate::item::fn_::parse_fn_item;
use crate::parser::ParsingContext;

mod bp;
pub mod call;
pub mod defer;
pub mod if_;
pub mod let_;
pub mod lit;
pub mod paren;
pub mod return_;
pub mod struct_;

pub fn parse_expr(parser: &mut crate::parser::Parser) {
    parse_expr_with_binding_power(parser, BindingPower::ZERO)
}

// warning: tricky stuff, be careful and read https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
fn parse_expr_with_binding_power(parser: &mut crate::parser::Parser, min_bp: BindingPower) {
    let checkpoint = parser.checkpoint();

    // main expression
    parser.start_node(N![Expr]);

    match parser.peek(0).unwrap().kind {
        T![@ident] => parse_name(parser),
        T![@number] | T![@string] => parse_lit_expr(parser),
        T!['('] => parse_paren_expr(parser),
        T!['{'] => parse_block(parser),
        T![return] => parse_return_expr(parser),
        T![if] => parse_if_expr(parser),
        T![defer] => parse_defer_expr(parser),
        T![let] => parse_let_expr(parser),
        T![def] => parse_fn_item(parser),
        // prefix ops
        op if let Some(bp) = binding_power_for(op, Flavor::Prefix) => {
            parser.start_node_at(N![PrefixExpr], checkpoint);

            // for prettiness we use _at again so that the prefix op node comes before the expr node
            parser.start_node_at(N![PrefixExprOp], checkpoint);
            parser.eat_any();
            parser.end_node();

            parse_expr_with_binding_power(parser, bp); // rhs

            parser.end_node();
        }
        other => todo!("{other:#?}"),
    }

    parser.end_node();

    // binary ops
    loop {
        merge_op_tokens(parser);

        let Some(Some(bp)) = parser
            .peek(0)
            .map(|token| binding_power_for(token.kind, Flavor::Infix))
        else {
            break;
        };

        if bp.l_value() < min_bp.l_value() {
            break;
        }

        parser.start_node_at(N![Expr], checkpoint);
        parser.start_node_at(N![BinaryExpr], checkpoint);

        // the lhs is now here

        parser.start_node(N![BinaryExprOp]);
        parser.eat_any();
        parser.end_node();

        parse_expr_with_binding_power(parser, bp.as_r_value()); // rhs

        parser.end_node();
        parser.end_node();
    }

    // postfix ops
    while let Some(Some(bp)) = parser
        .peek(0)
        .map(|token| binding_power_for(token.kind, Flavor::Postfix))
        && (bp.l_value() >= min_bp.l_value())
    {
        match parser.peek(0).unwrap().kind {
            T!['('] => parse_call_expr(parser, checkpoint),
            T!['{'] => {
                if !parser.is_in_cx(ParsingContext::IfExprCond) {
                    parse_struct_expr(parser, checkpoint);
                } else {
                    break;
                }
            }
            _ => unreachable!(),
        }
    }
}

// TODO: remove this and match on (peek(0), peek(1), peek(2)) directly
#[rustfmt::skip]
fn merge_op_tokens(parser: &mut crate::parser::Parser) {
    macro peek($n:literal) {
        parser.peek($n).map(|token| token.kind)
    }

    macro arm {
        (@priv subpat _) => { _ },
        (@priv subpat $t:tt) => { Some(T![$t]) },
        ($($t:tt)*) => {
            (
                $(
                    arm!(@priv subpat $t)
                ),*
            )
        },
    }

    // mind the order
    match (peek!(0), peek!(1), peek!(2)) {
        arm!(< < =) => merge::<3>(parser, T![<<=]),
        arm!(> > =) => merge::<3>(parser, T![>>=]),
        arm!(* * =) => merge::<3>(parser, T![+=]),
        arm!(+ = _) => merge::<2>(parser, T![+=]),
        arm!(- = _) => merge::<2>(parser, T![-=]),
        arm!(/ = _) => merge::<2>(parser, T![/=]),
        arm!(* = _) => merge::<2>(parser, T![*=]),
        arm!(% = _) => merge::<2>(parser, T![%=]),
        arm!(^ = _) => merge::<2>(parser, T![^=]),
        arm!(& = _) => merge::<2>(parser, T![&=]),
        arm!(| = _) => merge::<2>(parser, T![|=]),
        arm!(* * _) => merge::<2>(parser, T![**]),
        arm!(& & _) => merge::<2>(parser, T![&&]),
        arm!(| | _) => merge::<2>(parser, T![||]),
        arm!(= = _) => merge::<2>(parser, T![==]),
        arm!(! = _) => merge::<2>(parser, T![!=]),
        arm!(> = _) => merge::<2>(parser, T![>=]),
        arm!(< = _) => merge::<2>(parser, T![<=]),
        arm!(< < _) => merge::<2>(parser, T![<<]),
        arm!(> > _) => merge::<2>(parser, T![>>]),
        arm!(. . _) => merge::<2>(parser, T![..]),
        arm!(| > _) => merge::<2>(parser, T![|>]),
        _ => {}
    }
}

fn merge<const N: usize>(parser: &mut crate::parser::Parser, kind: SyntaxKind) {
    let first_token = parser.next().unwrap();

    if N == 3 {
        parser.next();
    }

    let last_token = parser.peek_mut(0).unwrap();

    *last_token = Token {
        kind,
        span: Span {
            start: first_token.span.start,
            end: last_token.span.end,
        },
    };
}
