// SPDX-License-Identifier: MIT

use metal_ast::{SyntaxKind, T};

#[derive(Debug)]
pub struct BindingPower {
    value: u16,
    assoc: Assoc,
}

#[derive(Debug)]
pub enum Assoc {
    Left,
    Right,
    Inapplicable,
}

#[derive(Debug)]
pub enum Flavor {
    Prefix,
    Infix,
    Postfix,
}

impl BindingPower {
    pub const ZERO: Self = BindingPower {
        value: 0,
        assoc: Assoc::Inapplicable,
    };

    pub fn l_value(&self) -> u16 {
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

impl From<(u16, Assoc)> for BindingPower {
    fn from((value, assoc): (u16, Assoc)) -> Self {
        Self { value, assoc }
    }
}

metal_proc_macros::associativity_table! {
    [=]
    | [+=]
    | [-=]
    | [/=]
    | [*=]
    | [**=]
    | [%=]
    | [^=]
    | [&=]
    | [|=]
    | [<<=]
    | [>>=] infix right
    // -
    [..] infix other
    [||] infix left
    [&&] infix left
    // -
    [==]
    | [!=]
    | [>]
    | [>=]
    | [<]
    | [<=] infix other
    // -
    [|] infix left
    [^] infix left
    [&] infix left
    [<<] | [>>] infix left
    [+] | [-] infix left
    [*] | [/] | [%] infix left
    [**] infix right
    // -
    [+] | [-] | [!] | [~] | [*] prefix other
    // -
    ['('] | ['{'] postfix other
    [.] infix left
}
