use crate::{Dom, DomNode};

impl Dom {
    pub fn children(&self) -> impl Iterator<Item = &DomNode> {
        // safety: the 'static lifetime is a placeholder; the Dom actually owns all DomNodes, so changing
        // it here to the conrete lifetime of &self is correct
        #[allow(clippy::unnecessary_cast)] // https://github.com/rust-lang/rust-clippy/issues/12860
        let data = self.nodes.as_ptr() as *const DomNode<'_>;
        let len = self.nodes.len();

        // safety: trivially correct
        let children = unsafe { std::slice::from_raw_parts(data, len) };

        ChildrenIter::new(children, None)
    }
}
impl<'parent> DomNode<'parent> {
    pub fn children(&self) -> impl Iterator<Item = &DomNode> {
        // unlikely: nodes without children are not expected to have children_mut called on them
        if std::hint::unlikely(self.len == 0) {
            return ChildrenIter::new(&[], None);
        }

        // safety: DomNodes are allocated into a single Vec, and if `self.len > 0` there
        // are guaranteed to be `len` valid DomNodes after this one. the lifetime parameter
        // of DomNode refers to the DomNode's parent, thus setting it here to the lifetime
        // of &self (implicit) is correct. since children of a node are only accessible
        // via its parent, and we have a shared borrow of the parent (&self), the shared
        // access is correct
        #[allow(clippy::unnecessary_cast)] // https://github.com/rust-lang/rust-clippy/issues/12860
        let data = unsafe { (self as *const DomNode<'parent>).offset(1) } as *const DomNode<'_>;

        // safety: the pointer is correct as described above, thus trivially correct
        let children = unsafe { std::slice::from_raw_parts(data, self.len) };

        ChildrenIter::new(children, Some(self))
    }
}

struct ChildrenIter<'parent> {
    children: &'parent [DomNode<'parent>],
    parent: Option<&'parent DomNode<'parent>>,
    cursor: usize = 0,
}

impl<'parent> ChildrenIter<'parent> {
    fn new(
        children: &'parent [DomNode<'parent>],
        parent: Option<&'parent DomNode<'parent>>,
    ) -> Self {
        Self {
            children,
            parent,
            ..
        }
    }
}

impl<'parent> Iterator for ChildrenIter<'parent> {
    type Item = &'parent DomNode<'parent>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.children.get(self.cursor)?;

        // safety: technically not safe, but needed for iteration to work. parents are
        // refreshed every time a new iterator is created, so the parent is always valid
        // note: an immut iter can override the previous immut iter's set parent while the previous immut iter is still alive, that's fine since the parent doesn't change
        unsafe { (node as *const DomNode<'parent> as *mut DomNode<'parent>).as_mut_unchecked() }
            .parent = self.parent;

        self.cursor += node.len + 1;

        Some(node)
    }
}
