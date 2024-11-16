use crate::{expr::Expr, types::Type};

#[derive(Debug, Clone)]
pub struct Let {
    pub name: &'static str,
    pub ty: Type,
    pub expr: Option<Expr>,
}
