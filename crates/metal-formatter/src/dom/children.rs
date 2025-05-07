use crate::dom::{BareDomNode, Dom, DomNode};

impl<'dom> DomNode<'dom> {
    pub fn children(&self) -> impl Iterator<Item = DomNode> {
        DomNodeChildrenIter {
            children: self.children,
            current_idx: 0,
        }
    }
}

struct DomNodeChildrenIter<'dom> {
    children: &'dom [BareDomNode],
    current_idx: usize,
}

impl<'dom> Iterator for DomNodeChildrenIter<'dom> {
    type Item = DomNode<'dom>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.children.get(self.current_idx)?;

        let resolved = DomNode {
            bare: node,
            children: &self.children[self.current_idx + 1..=self.current_idx + node.len],
        };

        self.current_idx += node.len + 1;

        Some(resolved)
    }
}

impl Dom {
    pub fn children(&self) -> impl Iterator<Item = DomNode> {
        DomNodeChildrenIter {
            children: self.nodes.as_slice(),
            current_idx: 0,
        }
    }
}
