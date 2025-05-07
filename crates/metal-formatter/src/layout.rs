use crate::{dom::BreakIf, Dom, DomNode, DomNodeKind, RenderIf};

const INDENT: &str = "    ";
const COLUMN_LIMIT: usize = 20;

#[derive(PartialEq, Eq, Hash)]
struct NodeId(usize);

#[allow(non_camel_case_types)]
#[derive(Default, Debug)]
enum tribool {
    yes,
    #[default]
    maybe,
    no,
}

#[derive(Debug)]
struct NodeLayoutInfo {
    flat_width: usize,
    should_break: tribool,
}

#[derive(Default)]
pub struct LayoutState {
    nodes: std::collections::HashMap<NodeId, NodeLayoutInfo>,
    column: usize,
    last_group: Option<NodeId>,
}

impl NodeLayoutInfo {
    fn new(flat_width: usize) -> Self {
        Self {
            flat_width,
            should_break: tribool::maybe,
        }
    }
}

#[extend::ext]
impl DomNode<'_> {
    fn id(&self) -> NodeId {
        NodeId(self.children.as_ptr() as usize)
    }
}

impl NodeId {
    unsafe fn get_bare_node(&self) -> &crate::dom::BareDomNode {
        let ptr = (self.0 as *const crate::dom::BareDomNode).offset(-1);

        ptr.as_ref().unwrap()
    }

    unsafe fn get_children_slice(&self, len: usize) -> &[crate::dom::BareDomNode] {
        std::slice::from_raw_parts(self.0 as *const crate::dom::BareDomNode, len)
    }
}

impl std::fmt::Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bare = unsafe { self.get_bare_node() };
        let children = unsafe { self.get_children_slice(bare.len) };

        let node = crate::dom::DomNode { bare, children };

        node.fmt(f)
    }
}

impl LayoutState {
    pub fn dbg_nodes(&self) {
        dbg!(&self.nodes);
    }

    pub fn compute_flat_widths(&mut self, dom: &Dom) {
        for child in dom.children() {
            self.compute_flat_width(&child);
        }
    }

    pub fn compute_should_breaks(&mut self, dom: &Dom) {
        for child in dom.children() {
            self.compute_should_break(&child);
        }
    }

    fn compute_should_break(&mut self, node: &DomNode<'_>) {
        let cached_node = self.nodes.get_mut(&node.id()).unwrap_or_else(|| {
            panic!("expected every node to be cached before entering second layout pass")
        });

        match node.break_if() {
            BreakIf::ExceedsColumnLimit => {
                cached_node.should_break = if self.column + cached_node.flat_width > COLUMN_LIMIT {
                    tribool::yes
                } else {
                    tribool::no
                };
            }
            BreakIf::Never => {
                cached_node.should_break = tribool::no;
            }
        }

        let mut old_last_group = None;
        let mut old_column = None;

        if matches!(node.kind(), DomNodeKind::Group) {
            old_last_group = Some(self.last_group.replace(node.id()));
        }

        if matches!(cached_node.should_break, tribool::yes)
            && matches!(node.kind(), DomNodeKind::Indent)
        {
            let new_column = self.column + INDENT.len();
            old_column = Some(std::mem::replace(&mut self.column, new_column));
        }

        for child in node.children() {
            self.compute_should_break(&child);
        }

        if let Some(old_last_group) = old_last_group {
            self.last_group = old_last_group;
        }

        if let Some(old_column) = old_column {
            self.column = old_column;
        }
    }

    fn compute_flat_width(&mut self, node: &DomNode<'_>) -> usize {
        if matches!(node.render_if(), RenderIf::Broken) {
            self.nodes.insert(node.id(), NodeLayoutInfo::new(0));
            return 0;
        }

        let base_width = match node.kind() {
            DomNodeKind::Group | DomNodeKind::Indent => 0,
            DomNodeKind::Text(text) => text.len(),
            DomNodeKind::Token(token) => token.text().len(),
        };

        let mut children_width = 0;

        for child in node.children() {
            children_width += self.compute_flat_width(&child);
        }

        let total_width = base_width + children_width;

        self.nodes
            .insert(node.id(), NodeLayoutInfo::new(total_width));

        total_width
    }
}
