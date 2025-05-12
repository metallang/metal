use crate::{
    dom::{BreakIf, Dom, DomNode, DomNodeKind, RenderIf},
    Result,
};

pub struct DomBuilder {
    dom: Dom,
}

impl DomBuilder {
    pub fn new() -> Self {
        Self {
            dom: Dom { nodes: vec![] },
        }
    }

    fn new_node(&mut self, kind: DomNodeKind) -> DomNodeBuilder<'_> {
        let node = DomNode::new(kind);

        self.dom.nodes.push(node);

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

    pub fn group(&mut self) -> DomNodeBuilder {
        self.new_node(DomNodeKind::Group)
    }

    pub fn finish(mut self) -> Dom {
        fn set_parent_recursive(node: &mut DomNode, parent: Option<*const DomNode>) {
            if let Some(parent) = parent {
                // safety: the dom is read-only, so at this point where we have full ownership
                // of it and are only modifying .parent the pointers provided here will never
                // be invalidated without the whole dom (where they are stored) going down
                unsafe {
                    node.set_parent(parent);
                }
            }

            for child in node.children_mut() {
                set_parent_recursive(child, Some(node as *const DomNode));
            }
        }

        for child in self.dom.children_mut() {
            set_parent_recursive(child, None);
        }

        self.dom
    }
}

pub struct DomNodeBuilder<'this> {
    // safety requirement: must be a valid DomNode index in DomBuilder's
    // Dom for at least as long as this DomNodeBuilder exists (for at least 'this)
    node_idx: usize,
    builder: &'this mut DomBuilder,
}

impl<'builder> DomNodeBuilder<'builder> {
    // safety requirement: builder must have at least one node
    unsafe fn from_last_dom_node(builder: &'builder mut DomBuilder) -> Self {
        // safety: node_idx is a valid index because the caller guaranteed that builder has
        // at least one node, the dom is append-only and we have exclusive borrow of the dom

        Self {
            node_idx: builder.dom.nodes.len() - 1,
            builder,
        }
    }

    fn node_mut(&mut self) -> &mut DomNode {
        // safety: guaranteed to exist by the instantiator of DomNodeBuilder
        unsafe { self.builder.dom.nodes.get_unchecked_mut(self.node_idx) }
    }

    pub fn render_if(mut self, rule: RenderIf) -> Self {
        self.node_mut().render_if = rule;

        self
    }

    pub fn break_if(mut self, rule: BreakIf) -> Self {
        self.node_mut().break_if = rule;

        self
    }

    /// # Panics
    ///
    /// Panics if the node ends up having more than [isize::MAX] children after the call to `f`.
    pub fn children(mut self, f: impl FnOnce(&mut DomBuilder) -> Result) -> Result<Self> {
        let old_len = self.builder.dom.nodes.len();

        f(self.builder)?;

        let new_len = self.builder.dom.nodes.len();

        let children_added: isize = (new_len - old_len)
            .try_into()
            .unwrap_or_else(|_| panic!("nodes cannot have more than isize::MAX children"));

        // we use += here to allow multiple consequtive .children calls
        // this relies on obtaining DomNodeBuilders requiring &mut Dom, which
        // ensures that the dom doesn't get modified while we still provide
        // this interface for adding children to this node. if it didn't ensure
        // that, one could've saved the DomNodeBuilder, created some nodes after
        // it, and then tried adding children to it again, which wouldn't work
        // since we're using a flat vec + child count for storing children
        self.node_mut().len += children_added;

        Ok(self)
    }
}
