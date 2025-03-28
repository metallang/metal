// SPDX-License-Identifier: MIT

use metal_ast::{SyntaxKind, T};

pub struct BindingPower {
    value: u8,
    assoc: Assoc,
}

impl BindingPower {
    pub const ZERO: Self = BindingPower {
        value: 0,
        assoc: Assoc::Inapplicable,
    };

    pub fn l_value(&self) -> u8 {
        self.value
    }

    pub fn as_r_value(&self) -> BindingPower {
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
pub enum Assoc {
    Left,
    Right,
    Inapplicable,
}

pub fn infix_binding_power_for(op: SyntaxKind) -> Option<BindingPower> {
    let bp = match op {
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
        T![==] | T![!=] | T![>] | T![>=] | T![<] | T![<=] => (9, Assoc::Inapplicable),
        // -
        T![&&] => (7, Assoc::Left),
        T![||] => (5, Assoc::Left),
        T![..] => (3, Assoc::Inapplicable),
        // -
        T![=]
        | T![+=]
        | T![-=]
        | T![/=]
        | T![*=]
        | T![**=]
        | T![%=]
        | T![^=]
        | T![&=]
        | T![|=]
        | T![<<=]
        | T![>>=] => (1, Assoc::Right),
        // -
        _ => return None,
    };

    Some(bp.into())
}

pub fn postfix_binding_power_for(op: SyntaxKind) -> Option<BindingPower> {
    Some(
        match op {
            T!['('] | T!['{'] => (27, Assoc::Inapplicable),
            _ => return None,
        }
        .into(),
    )
}

pub fn prefix_binding_power_for(op: SyntaxKind) -> Option<BindingPower> {
    Some(
        match op {
            T![+] | T![-] | T![!] | T![~] | T![*] => (25, Assoc::Inapplicable),
            _ => return None,
        }
        .into(),
    )
}
