pub mod build;
pub mod children;
pub mod debug;

#[derive(Default)]
pub struct Dom {
    nodes: Vec<DomNode<'static>>,
}

pub struct DomNode<'parent> {
    kind: DomNodeKind,
    render_if: RenderIf = RenderIf::Always,
    break_if: BreakIf = BreakIf::ExceedsColumnLimit,
    parent: Option<&'parent DomNode<'parent>> = None,
    len: usize = 0,
}

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

pub enum RenderIf {
    Always,
    Broken,
    Flat,
}

pub enum BreakIf {
    ExceedsColumnLimit,
    Never,
    // Always,
}

impl<'parent> DomNode<'parent> {
    fn new(kind: DomNodeKind) -> Self {
        Self { kind, .. }
    }

    pub fn kind(&self) -> &DomNodeKind {
        &self.kind
    }

    pub fn render_if(&self) -> &RenderIf {
        &self.render_if
    }

    pub fn break_if(&self) -> &BreakIf {
        &self.break_if
    }

    pub fn parent(&self) -> Option<&DomNode> {
        self.parent
    }
}
