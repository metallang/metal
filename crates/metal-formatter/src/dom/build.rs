use crate::{
    dom::{BreakIf, Dom, DomNodeData, DomNodeKind, RenderIf},
    Result,
};

impl Dom {
    pub fn token(&mut self, token: metal_ast::SyntaxToken) -> DomNodeBuilder {
        DomNodeBuilder::new_node(self, DomNodeKind::Token(token))
    }

    pub fn text(&mut self, text: &'static str) -> DomNodeBuilder {
        DomNodeBuilder::new_node(self, DomNodeKind::Text(text))
    }

    pub fn indent(&mut self) -> DomNodeBuilder {
        DomNodeBuilder::new_node(self, DomNodeKind::Indent).break_if(BreakIf::SameAsParent)
    }

    pub fn indent_slot(&mut self) -> DomNodeBuilder {
        DomNodeBuilder::new_node(self, DomNodeKind::IndentSlot).render_if(RenderIf::Broken)
    }

    pub fn group(&mut self) -> DomNodeBuilder {
        DomNodeBuilder::new_node(self, DomNodeKind::Group)
    }
}

pub struct DomNodeBuilder<'dom> {
    node_idx: usize,
    dom: &'dom mut Dom,
}

impl<'dom> DomNodeBuilder<'dom> {
    fn new_node(dom: &'dom mut Dom, kind: DomNodeKind) -> Self {
        dom.nodes.push(DomNodeData { kind, .. });

        Self {
            node_idx: dom.nodes.len() - 1,
            dom,
        }
    }

    fn node_mut(&mut self) -> &mut DomNodeData {
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
