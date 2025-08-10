// SPDX-License-Identifier: MIT

use crate::dom::children::ref_::{ChildrenIter, DomNode};
use crate::{Dom, DomNodeData};

pub struct DomNodeMut<'parent> {
    node: &'parent mut DomNodeData,
    children: &'parent mut [DomNodeData],
}

impl std::ops::Deref for DomNodeMut<'_> {
    type Target = DomNodeData;

    fn deref(&self) -> &Self::Target {
        &*self.node
    }
}

impl std::ops::DerefMut for DomNodeMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.node
    }
}

impl DomNodeMut<'_> {
    pub fn as_ref(&self) -> DomNode<'_> {
        DomNode {
            node: self.node,
            children: self.children,
        }
    }

    pub fn children(&self) -> impl Iterator<Item = DomNode> {
        ChildrenIter {
            children: &*self.children,
            ..
        }
    }

    pub fn children_mut(&mut self) -> impl Iterator<Item = DomNodeMut> {
        ChildrenMutIter {
            children: self.children,
            ..
        }
    }
}

impl Dom {
    pub fn children_mut(&mut self) -> impl Iterator<Item = DomNodeMut> {
        ChildrenMutIter {
            children: self.nodes.as_mut_slice(),
            ..
        }
    }
}

struct ChildrenMutIter<'parent> {
    children: &'parent mut [DomNodeData],
}

impl<'parent> Iterator for ChildrenMutIter<'parent> {
    type Item = DomNodeMut<'parent>;

    fn next(&mut self) -> Option<Self::Item> {
        let children = std::mem::take(&mut self.children);

        let (node, rest) = children.split_at_mut_checked(1)?;
        let node = unsafe { node.first_mut().unwrap_unchecked() };
        let (children, rest) = rest.split_at_mut_checked(node.len)?;

        let view = DomNodeMut { node, children };

        self.children = rest;

        Some(view)
    }
}
