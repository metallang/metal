use crate::{Dom, DomNode};

impl Dom {
    pub fn children_mut(&mut self) -> impl Iterator<Item = &mut DomNode> {
        // safety: the 'static lifetime is a placeholder; the Dom actually owns all DomNodes, so changing
        // it here to the conrete lifetime of &mut self is correct
        #[allow(clippy::unnecessary_cast)] // https://github.com/rust-lang/rust-clippy/issues/12860
        let data = self.nodes.as_mut_ptr() as *mut DomNode<'_>;
        let len = self.nodes.len();

        // safety: trivially correct
        let children = unsafe { std::slice::from_raw_parts_mut(data, len) };

        ChildrenMutIter::new(children, None)
    }
}

impl<'parent> DomNode<'parent> {
    pub fn children_mut(&mut self) -> impl Iterator<Item = &mut DomNode> {
        // unlikely: nodes without children are not expected to have children_mut called on them
        if std::hint::unlikely(self.len == 0) {
            return ChildrenMutIter::new(&mut [], None);
        }

        // safety: DomNodes are allocated into a single Vec, and if `self.len > 0` there
        // are guaranteed to be `len` valid DomNodes after this one. the lifetime parameter
        // of DomNode refers to the DomNode's parent, thus setting it here to the lifetime
        // of &mut self (implicit) is correct. since children of a node are only accessible
        // via its parent, and we have an exclusive borrow of the parent (&mut self), the
        // mutable access is correct
        #[allow(clippy::unnecessary_cast)] // https://github.com/rust-lang/rust-clippy/issues/12860
        let data = unsafe { (self as *mut DomNode<'parent>).offset(1) } as *mut DomNode<'_>;

        // safety: the pointer is correct as described above, thus trivially correct
        let children = unsafe { std::slice::from_raw_parts_mut(data, self.len) };

        // safety: the parent cannot be mutated from children, thus an immutable reference to self
        // coexisting with mutable references to children is correct
        let parent = Some(&*self);

        ChildrenMutIter::new(children, parent)
    }
}

struct ChildrenMutIter<'parent> {
    children: &'parent mut [DomNode<'parent>],
    parent: Option<&'parent DomNode<'parent>>,
    cursor: usize = 0,
}
impl<'parent> ChildrenMutIter<'parent> {
    fn new(
        children: &'parent mut [DomNode<'parent>],
        parent: Option<&'parent DomNode<'parent>>,
    ) -> Self {
        Self {
            children,
            parent,
            ..
        }
    }
}
impl<'parent> Iterator for ChildrenMutIter<'parent> {
    type Item = &'parent mut DomNode<'parent>;
    #[allow(unused_mut)]
    fn next(&mut self) -> Option<Self::Item> {
        // safety: this implementation guarantees that the nodes after the updated self.cursor
        // are not accessed, mutably or immutably, by the yielded node, therefore no double
        // mutable borrow to the same memory region is created
        let children = unsafe { (self.children as *mut [DomNode<'parent>]).as_mut_unchecked() };
        let node = children.get_mut(self.cursor)?;

        node.parent = self.parent;
        self.cursor += node.len + 1;

        Some(node)
    }
}
