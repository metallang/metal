pub mod build;
pub mod children;
pub mod debug;
pub mod render;

#[derive(Default)]
pub struct Dom {
    pub(crate) nodes: Vec<DomNodeData>,
}

#[derive(Debug)]
pub struct DomNodeData {
    // data
    pub kind: DomNodeKind,
    pub render_if: RenderIf = RenderIf::Always,
    pub break_if: BreakIf = BreakIf::ExceedsColumnLimit,
    // structure
    len: usize = 0,
    // layout
    broken: bool = false,
    width: usize = 0,
}

#[derive(Debug, PartialEq)]
pub enum DomNodeKind {
    /// A simple node group. Doesn't have any special meaning.
    Group,
    /// A special node group that increases the indentation level if the enclosing group is broken.
    Indent,
    /// Inserts indentation text based on the enclosing group's indentation level.
    IndentSlot,
    /// Inserts a newline.
    Newline,
    /// A text node.
    Text(&'static str),
    /// A token node.
    Token(metal_ast::SyntaxToken),
}

#[derive(Debug, PartialEq)]
pub enum RenderIf {
    Always,
    Broken,
    Flat,
}

#[derive(Debug, PartialEq)]
pub enum BreakIf {
    ExceedsColumnLimit,
    Never,
    Always,
    SameAsParent,
}
