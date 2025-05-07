use super::BreakIf;
use crate::{
    dom::{BareDomNode, Dom, DomNodeKind, RenderIf},
    Result,
};

pub struct DomNodeEdit<'dom> {
    dom: &'dom mut Dom,
    node_idx: usize,
}

impl<'dom> DomNodeEdit<'dom> {
    pub(crate) fn node_mut(&mut self) -> &mut BareDomNode {
        &mut self.dom.nodes[self.node_idx]
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

impl Dom {
    pub(crate) fn new_node(&mut self, kind: DomNodeKind) -> DomNodeEdit<'_> {
        let node_idx = self.nodes.len();
        let node = BareDomNode {
            kind,
            render_if: Default::default(),
            break_if: Default::default(),
            len: 0,
        };

        self.nodes.push(node);

        DomNodeEdit {
            dom: self,
            node_idx,
        }
    }

    pub fn token(&mut self, token: metal_ast::SyntaxToken) -> DomNodeEdit {
        self.new_node(DomNodeKind::Token(token))
    }

    pub fn text(&mut self, text: &'static str) -> DomNodeEdit {
        self.new_node(DomNodeKind::Text(text))
    }

    pub fn indent(&mut self) -> DomNodeEdit {
        self.new_node(DomNodeKind::Indent)
    }

    pub fn group(&mut self) -> DomNodeEdit {
        self.new_node(DomNodeKind::Group)
    }
}
