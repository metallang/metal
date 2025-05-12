use std::marker::PhantomData;

use super::{Dom, DomNode};

impl Dom {
    /// # Panics
    ///
    /// Panics if the Dom has over [isize::MAX] children
    pub fn children_mut(&mut self) -> impl Iterator<Item = &mut DomNode> {
        // safety: conditions naturally satisfied
        unsafe {
            ChildrenMutIter::new(
                self.nodes.as_mut_ptr(),
                self.nodes.len().try_into().unwrap(),
            )
        }
    }
}

impl DomNode {
    pub fn children_mut(&self) -> impl Iterator<Item = &mut DomNode> {
        // unlikely: nodes without children are not expected to have children_mut called on them
        if std::hint::unlikely(self.len == 0) {
            // safety: max_offset = -1 is out of the required "validness" range, so it's safe to pass a nullptr
            return unsafe { ChildrenMutIter::new(std::ptr::null_mut(), -1) };
        }

        // safety: modifying children of this node does not modify this, parent, or sibling nodes,
        // and the &self -> &mut DomNode* borrow ensures that DomNode::parent(&self) is also valid
        let self_as_mut = self as *const DomNode as usize as *mut DomNode;

        // safety: DomNodes are allocated into a single Vec, and if `self.len > 0` there
        // are guaranteed to be `len` valid DomNodes after this one
        let data = unsafe { self_as_mut.offset(1) };

        // safety: see the above comment
        unsafe { ChildrenMutIter::new(data, self.len) }
    }
}

struct ChildrenMutIter<'parent> {
    first_child_ptr: *mut DomNode,
    max_offset: isize,
    cur_offset: isize = 0,
    _dom: PhantomData<&'parent ()> = PhantomData,
}

impl<'parent> ChildrenMutIter<'parent> {
    // safety: any first_child_ptr.offset(N) where 0 <= N < max_offset must be a valid DomNode
    // (values outside of that range are allowed to be invalid) for the duration of 'parent for
    // the mutable access
    unsafe fn new(first_child_ptr: *mut DomNode, max_offset: isize) -> Self {
        Self {
            first_child_ptr,
            max_offset,
            ..
        }
    }
}

impl<'parent> Iterator for ChildrenMutIter<'parent> {
    type Item = &'parent mut DomNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_offset >= self.max_offset {
            return None;
        }

        // safety: instantiator guaranteed that any first_child_ptr.offset(N) where N < max_offset is a valid DomNode for mutable access
        let node = unsafe {
            self.first_child_ptr
                .offset(self.cur_offset)
                .as_mut_unchecked()
        };

        self.cur_offset += node.len + 1;

        Some(node)
    }
}
