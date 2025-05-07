pub mod build;
pub mod children;
pub mod debug;

#[derive(Default)]
pub struct Dom {
    nodes: Vec<BareDomNode>,
}

#[derive(Debug)]
pub(crate) struct BareDomNode {
    kind: DomNodeKind,
    render_if: RenderIf,
    break_if: BreakIf,
    pub(crate) len: usize,
}

pub struct DomNode<'dom> {
    pub(crate) bare: &'dom BareDomNode,
    pub(crate) children: &'dom [BareDomNode],
}

#[derive(Debug)]
pub enum DomNodeKind {
    /// A simple node group. Doesn't have any special meaning.
    Group,
    /// A node group that indents children if the enclosing group is broken.
    Indent,
    /// A text node.
    Text(&'static str),
    /// A token node.
    Token(metal_ast::SyntaxToken),
}

#[derive(Default, Debug)]
pub enum RenderIf {
    #[default]
    Always,
    Broken,
    Flat,
}

#[derive(Default, Debug)]
pub enum BreakIf {
    #[default]
    ExceedsColumnLimit,
    Never,
    // Always,
}

impl DomNode<'_> {
    pub fn kind(&self) -> &DomNodeKind {
        &self.bare.kind
    }

    pub fn render_if(&self) -> &RenderIf {
        &self.bare.render_if
    }

    pub fn break_if(&self) -> &BreakIf {
        &self.bare.break_if
    }
}
