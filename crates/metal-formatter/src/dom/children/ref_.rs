// SPDX-License-Identifier: MIT

use crate::{Dom, DomNodeData};

#[derive(Clone)]
pub struct DomNode<'parent> {
    pub(super) node: &'parent DomNodeData,
    pub(super) children: &'parent [DomNodeData],
}

impl std::ops::Deref for DomNode<'_> {
    type Target = DomNodeData;

    fn deref(&self) -> &Self::Target {
        self.node
    }
}

impl DomNode<'_> {
    pub fn children(&self) -> impl Iterator<Item = DomNode> {
        // eprintln!("{:?}", self.node);

        ChildrenIter {
            children: self.children,
            ..
        }
    }
}
impl Dom {
    pub fn children(&self) -> impl Iterator<Item = DomNode> {
        ChildrenIter {
            children: self.nodes.as_slice(),
            ..
        }
    }
}

pub(super) struct ChildrenIter<'parent> {
    pub(super) children: &'parent [DomNodeData],
}

impl<'parent> Iterator for ChildrenIter<'parent> {
    type Item = DomNode<'parent>;

    fn next(&mut self) -> Option<Self::Item> {
        let children = std::mem::take(&mut self.children);

        let (node, rest) = children.split_at_checked(1)?;
        let node = unsafe { node.first().unwrap_unchecked() };
        let (children, rest) = rest.split_at_checked(node.len)?;

        let view = DomNode { node, children };

        self.children = rest;

        Some(view)
    }
}
