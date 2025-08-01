pub mod build;
pub mod children;
pub mod debug;

#[derive(Default)]
pub struct Dom {
    pub(crate) nodes: Vec<DomNodeData>,
}

#[derive(Debug)]
pub struct DomNodeData {
    pub kind: DomNodeKind,
    pub render_if: RenderIf = RenderIf::Always,
    pub break_if: BreakIf = BreakIf::ExceedsColumnLimit,
    len: usize = 0,
}

#[derive(Debug)]
pub enum DomNodeKind {
    /// A simple node group. Doesn't have any special meaning.
    Group,
    /// A special node group that increases the indentation level if the enclosing group is broken.
    Indent,
    /// Inserts indentation text based on the enclosing group's indentation level.
    IndentSlot,
    /// A text node.
    Text(&'static str),
    /// A token node.
    Token(metal_ast::SyntaxToken),
}

#[derive(Debug)]
pub enum RenderIf {
    Always,
    Broken,
    Flat,
}

#[derive(Debug)]
pub enum BreakIf {
    ExceedsColumnLimit,
    Never,
    Always,
    SameAsParent,
}
