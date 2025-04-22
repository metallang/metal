// SPDX-License-Identifier: MIT

use break_::parse_break_expr;
use for_::parse_for_expr;
use loop_::parse_loop_expr;
use metal_ast::{SyntaxKind, N, T};
use metal_lexer::{Span, Token};
use while_::parse_while_expr;

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
use crate::restrictions::RestrictionFlags;

mod bp;
pub mod break_;
pub mod call;
pub mod defer;
pub mod for_;
pub mod if_;
pub mod let_;
pub mod lit;
pub mod loop_;
pub mod paren;
pub mod return_;
pub mod struct_;
pub mod while_;

pub fn parse_expr(parser: &mut crate::parser::Parser) {
    parse_expr_with_binding_power(parser, BindingPower::ZERO)
}

// warning: tricky stuff, be careful and read https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
fn parse_expr_with_binding_power(parser: &mut crate::parser::Parser, min_bp: BindingPower) {
    let checkpoint = parser.checkpoint();

    // main expression
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
        T![while] => parse_while_expr(parser),
        T![for] => parse_for_expr(parser),
        T![loop] => parse_loop_expr(parser),
        T![break] => parse_break_expr(parser),
        // prefix ops
        op if let Some(bp) = binding_power_for(op, Flavor::Prefix) => {
            parser.start_node_at(N![PrefixExpr], checkpoint);

            // for prettiness we use _at again so that the prefix op node comes before the expr node
            parser.eat_any();

            parse_expr_with_binding_power(parser, bp); // rhs

            parser.end_node();
        }
        other => todo!("{other:#?}"),
    }

    loop {
        // postfix ops
        if let Some(Some(bp)) = parser
            .peek(0)
            .map(|token| binding_power_for(token.kind, Flavor::Postfix))
            && (bp.l_value() >= min_bp.l_value())
        {
            match parser.peek(0).unwrap().kind {
                T!['('] => parse_call_expr(parser, checkpoint),
                T!['{'] => {
                    if parser
                        .restrictions()
                        .include(RestrictionFlags::NO_STRUCT_EXPR)
                    {
                        break;
                    }

                    parse_struct_expr(parser, checkpoint);
                }
                _ => unreachable!(),
            }

            continue;
        }

        merge_op_tokens(parser);

        // binary ops
        if let Some(Some(bp)) = parser
            .peek(0)
            .map(|token| binding_power_for(token.kind, Flavor::Infix))
            && (bp.l_value() >= min_bp.l_value())
        {
            parser.start_node_at(N![BinaryExpr], checkpoint);

            // the lhs is now here

            parser.eat_any();

            parse_expr_with_binding_power(parser, bp.as_r_value()); // rhs

            parser.end_node();

            continue;
        }

        break;
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
