use crate::{
    dom::{BreakIf, Dom, DomNode, DomNodeKind, RenderIf},
    Result,
};

impl Dom {
    fn new_node(&mut self, kind: DomNodeKind) -> DomNodeBuilder<'_> {
        self.nodes.push(DomNode::new(kind));

        // safety: just pushed a node
        unsafe { DomNodeBuilder::from_last_dom_node(self) }
    }

    pub fn token(&mut self, token: metal_ast::SyntaxToken) -> DomNodeBuilder {
        self.new_node(DomNodeKind::Token(token))
    }

    pub fn text(&mut self, text: &'static str) -> DomNodeBuilder {
        self.new_node(DomNodeKind::Text(text))
    }

    pub fn indent(&mut self) -> DomNodeBuilder {
        self.new_node(DomNodeKind::Indent)
    }

    pub fn indent_slot(&mut self) -> DomNodeBuilder {
        self.new_node(DomNodeKind::IndentSlot)
    }

    pub fn group(&mut self) -> DomNodeBuilder {
        self.new_node(DomNodeKind::Group)
    }
}

pub struct DomNodeBuilder<'dom> {
    // safety requirement: must be a valid DomNode index in DomBuilder's
    // Dom for at least as long as this DomNodeBuilder exists (for at least 'this)
    node_idx: usize,
    dom: &'dom mut Dom,
}

impl<'dom> DomNodeBuilder<'dom> {
    // safety requirement: builder must have at least one node
    unsafe fn from_last_dom_node(dom: &'dom mut Dom) -> Self {
        // safety: node_idx is a valid index because the caller guaranteed that dom has
        // at least one node, the dom is append-only and we have exclusive borrow of the dom
        Self {
            node_idx: dom.nodes.len() - 1,
            dom,
        }
    }

    fn node_mut(&mut self) -> &mut DomNode<'static> {
        // safety: guaranteed to exist by the instantiator of DomNodeBuilder
        unsafe { self.dom.nodes.get_unchecked_mut(self.node_idx) }
    }

    pub fn render_if(mut self, rule: RenderIf) -> Self {
        self.node_mut().render_if = rule;

        self
    }

    pub fn break_if(mut self, rule: BreakIf) -> Self {
        self.node_mut().break_if = rule;

        self
    }

    pub fn children(mut self, f: impl FnOnce(&mut Dom) -> Result) -> Result<Self> {
        let old_len = self.dom.nodes.len();

        f(self.dom)?;

        let new_len = self.dom.nodes.len();

        // we use += here to allow multiple consequtive .children calls
        // this relies on obtaining DomNodeBuilders requiring &mut Dom, which
        // ensures that the dom doesn't get modified while we still provide
        // this interface for adding children to this node. if it didn't ensure
        // that, one could've saved the DomNodeBuilder, created some nodes after
        // it, and then tried adding children to it again, which wouldn't work
        // since we're using a flat vec + child count for storing children
        self.node_mut().len += new_len - old_len;

        Ok(self)
    }
}
