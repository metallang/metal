use std::fmt::Debug;

pub mod build;
pub mod children;
pub mod debug;

#[derive(Debug)]
pub struct Dom {
    nodes: Vec<DomNode>,
}

#[derive(Debug)]
pub struct DomNode {
    kind: DomNodeKind,
    render_if: RenderIf = RenderIf::Always,
    break_if: BreakIf = BreakIf::ExceedsColumnLimit,
    parent: Option<*const DomNode> = None,
    len: isize = 0,
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
    // Always,
}

impl DomNode {
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
        // safety: guaranteed to be safe by the instantiator
        self.parent.map(|ptr| unsafe { ptr.as_ref_unchecked() })
    }

    // safety: new_parent must be valid for immut access for as long as this node is acessible
    unsafe fn set_parent(&mut self, new_parent: *const DomNode) {
        self.parent.replace(new_parent);
    }
}
