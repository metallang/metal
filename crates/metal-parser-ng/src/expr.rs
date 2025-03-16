use metal_ast_ng::SyntaxKind;
use metal_ast_ng::N;
use metal_ast_ng::T;

use crate::common::parse_name;

pub fn parse_expr(parser: &mut crate::parser::parser_type!()) {
    parse_expr_with_binding_power(parser, BindingPower::ZERO);
}

// warning: tricky stuff, be careful and read https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
fn parse_expr_with_binding_power(parser: &mut crate::parser::parser_type!(), min_bp: BindingPower) {
    let checkpoint = parser.checkpoint();

    // main expression
    parser.start_node(N![Expr]);

    match parser.peek().unwrap().kind {
        T![@ident] => parse_name(parser),
        T![@number] | T![@string] => {
            parser.start_node(N![LitExpr]);
            parser.eat_any();
            parser.end_node();
        }
        T!['('] => {
            parser.start_node(N![ParenExpr]);

            parser.eat_any();
            parse_expr(parser);
            parser.maybe_eat(T![')']);

            parser.end_node();
        }
        // prefix ops
        op if let Some(bp) = prefix_binding_power_for(op) => {
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
    while let Some(Some(bp)) = parser
        .peek()
        .map(|token| infix_binding_power_for(token.kind))
        && (bp.l_value() >= min_bp.l_value())
    {
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
        .peek()
        .map(|token| postfix_binding_power_for(token.kind))
        && (bp.l_value() >= min_bp.l_value())
    {
        match parser.peek().unwrap().kind {
            T!['('] => {
                parser.start_node_at(N![Expr], checkpoint);
                parser.start_node_at(N![CallExpr], checkpoint);

                // the callee is now here

                parser.eat_any(); // guaranteed to be an l_paren

                parser.start_node(N![CallExprArgs]);
                while !(parser.peek_is(T![')']) || parser.is_eof()) {
                    parse_expr(parser);
                    parser.maybe_eat(T![,]);
                }
                parser.end_node();

                parser.maybe_eat(T![')']);

                parser.end_node();
                parser.end_node();
            }
            T!['['] => todo!(), // FIXME: not in the grammar yet
            _ => unreachable!(),
        }
    }
}

struct BindingPower {
    value: u8,
    assoc: Assoc,
}

impl BindingPower {
    const ZERO: Self = BindingPower {
        value: 0,
        assoc: Assoc::Inapplicable,
    };

    fn l_value(&self) -> u8 {
        self.value
    }

    fn as_r_value(&self) -> BindingPower {
        let assoc_val = match self.assoc {
            Assoc::Left => 1,
            Assoc::Right | Assoc::Inapplicable => 0,
        };

        Self {
            value: self.value + assoc_val,
            assoc: Assoc::Inapplicable,
        }
    }
}

impl From<(u8, Assoc)> for BindingPower {
    fn from((value, assoc): (u8, Assoc)) -> Self {
        Self { value, assoc }
    }
}

#[derive(Debug)]
enum Assoc {
    Left,
    Right,
    Inapplicable,
}

#[rustfmt::skip]
fn infix_binding_power_for(op: SyntaxKind) -> Option<BindingPower> {
    Some(match op {
        T![.] => (29, Assoc::Left),
        // {postfix ops}
        // {prefix ops}
        T![**] => (23, Assoc::Right),
        T![*] | T![/] | T![%] => (21, Assoc::Left),
        T![+] | T![-] => (19, Assoc::Left),
        T![<<] | T![>>] => (17, Assoc::Left),
        T![&] => (15, Assoc::Left),
        T![^] => (13, Assoc::Left),
        T![|] => (11, Assoc::Left),
        // -
        T![==] | T![!=]
        | T![>] | T![>=]
        | T![<] | T![<=] => (9, Assoc::Inapplicable),
        // -
        T![&&] => (7, Assoc::Left),
        T![||] => (5, Assoc::Left),
        T![..] => (3, Assoc::Inapplicable),
        // -
        T![=]
        | T![+=] | T![-=] | T![/=] | T![*=]
        | T![**=] | T![%=]
        | T![^=] | T![&=] | T![|=]
        | T![<<=] | T![>>=] => (1, Assoc::Right),
        // -
        _ => return None,
    }
    .into())
}

fn postfix_binding_power_for(op: SyntaxKind) -> Option<BindingPower> {
    Some(
        match op {
            T!['('] | T!['['] => (27, Assoc::Inapplicable),
            _ => return None,
        }
        .into(),
    )
}

fn prefix_binding_power_for(op: SyntaxKind) -> Option<BindingPower> {
    Some(
        match op {
            T![+] | T![-] | T![!] | T![~] | T![*] => (25, Assoc::Inapplicable),
            _ => return None,
        }
        .into(),
    )
}
