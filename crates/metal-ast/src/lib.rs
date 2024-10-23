//! Direct representation of source code.

// All structures here are prefixed with AST to save on aliasing when importing them.
// It also allows to quickly filter out unrelated items in autocompletion.

pub type ASTFile<'src> = Vec<ASTStatement<'src>>;

pub enum ASTStatement<'src> {
    Import(ImportStmt<'src>),
}

pub struct ImportStmt<'src> {
    _p: std::marker::PhantomData<&'src ()>,
}
