use std::collections::HashMap;

use crate::{dom::BreakIf, Dom, DomNode, DomNodeKind, RenderIf};

const INDENT: &str = "    ";
const COLUMN_LIMIT: usize = 10;

#[derive(PartialEq, Eq, Hash)]
struct NodeId(usize);

#[derive(Default)]
struct NodeIdCounter(usize);

impl NodeIdCounter {
    fn get(&self) -> NodeId {
        NodeId(self.0)
    }

    fn next(&mut self) -> NodeId {
        self.0 += 1;
        self.get()
    }

    fn reset(&mut self) {
        self.0 = 0;
    }
}
#[derive(Default)]
struct IndentTracker {
    indent_level: usize,
}

impl IndentTracker {
    fn get(&self) -> String {
        INDENT.repeat(self.indent_level)
    }

    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn dedent(&mut self) {
        self.indent_level -= 1;
    }

    fn is_overboard(&self, flat_width: usize) -> bool {
        self.indent_level * INDENT.len() + flat_width > COLUMN_LIMIT
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, Debug, PartialEq, Clone, Copy)]
enum tribool {
    yes,
    #[default]
    maybe,
    no,
}

#[derive(Default, Debug)]
struct NodeLayout {
    flat_width: usize,
    should_break: tribool,
}

pub struct Layout<'dom> {
    dom: &'dom Dom,
    node_ids: NodeIdCounter,
    node_layouts: HashMap<NodeId, NodeLayout>,
    indent: IndentTracker,
}

impl<'dom> Layout<'dom> {
    pub fn new(dom: &'dom Dom) -> Self {
        Self {
            dom,
            node_ids: NodeIdCounter::default(),
            node_layouts: HashMap::default(),
            indent: IndentTracker::default(),
        }
    }

    pub fn compute_flat_widths(&mut self) {
        for child in self.dom.children() {
            self.compute_flat_width(child);
        }

        self.node_ids.reset();
    }

    fn compute_flat_width(&mut self, node: DomNode) -> usize {
        let node_id = self.node_ids.next();

        if let RenderIf::Broken = node.render_if {
            self.node_layouts.entry(node_id).or_default().flat_width = 0;

            return 0;
        }

        let mut width = 0;

        match &node.kind {
            DomNodeKind::Group | DomNodeKind::Indent | DomNodeKind::IndentSlot => {}
            DomNodeKind::Text(text) => width += text.len(),
            DomNodeKind::Token(token) => width += token.text().len(),
        }

        for child in node.children() {
            width += self.compute_flat_width(child);
        }

        self.node_layouts.entry(node_id).or_default().flat_width = width;

        width
    }

    pub fn compute_should_breaks(&mut self) {
        for child in self.dom.children() {
            self.compute_should_break(child, &NodeId(0));
        }

        self.node_ids.reset();
    }

    fn compute_should_break(&mut self, node: DomNode, parent_node_id: &NodeId) {
        let node_id = self.node_ids.next();

        match &node.break_if {
            BreakIf::ExceedsColumnLimit => {
                let node_flat_width = self.node_layouts.get(&node_id).unwrap().flat_width;

                if self.indent.is_overboard(node_flat_width) {
                    self.node_layouts.get_mut(&node_id).unwrap().should_break = tribool::yes;
                } else {
                    self.node_layouts.get_mut(&node_id).unwrap().should_break = tribool::no;
                }
            }
            BreakIf::Never => {
                self.node_layouts.get_mut(&node_id).unwrap().should_break = tribool::no
            }
            BreakIf::Always => {
                self.node_layouts.get_mut(&node_id).unwrap().should_break = tribool::yes
            }
            BreakIf::SameAsParent => {
                let parent_should_break = self
                    .node_layouts
                    .get(parent_node_id)
                    .map(|l| l.should_break)
                    .unwrap_or(tribool::no);

                self.node_layouts.get_mut(&node_id).unwrap().should_break = parent_should_break;
            }
        }

        match &node.kind {
            DomNodeKind::Indent => self.indent.indent(),
            _ => {}
        }

        for child in node.children() {
            self.compute_should_break(child, &node_id);
        }

        match &node.kind {
            DomNodeKind::Indent => self.indent.dedent(),
            _ => {}
        }
    }

    pub fn render(&mut self) -> Result<String, std::fmt::Error> {
        let mut buf = String::new();

        for child in self.dom.children() {
            self.render_node(child, &mut buf, &NodeId(0));
        }

        self.node_ids.reset();

        Ok(buf)
    }

    fn render_node(
        &mut self,
        node: DomNode,
        fmt: &mut impl std::fmt::Write,
        parent_node_id: &NodeId,
    ) -> std::fmt::Result {
        let node_id = self.node_ids.next();
        let parent_broken = self
            .node_layouts
            .get(parent_node_id)
            .map(|l| &l.should_break)
            .unwrap_or(&tribool::no);

        let should_render = match &node.render_if {
            RenderIf::Always => true,
            RenderIf::Broken => parent_broken == &tribool::yes,
            RenderIf::Flat => parent_broken == &tribool::no,
        };

        if should_render {
            match &node.kind {
                DomNodeKind::Group | DomNodeKind::Indent => {}
                DomNodeKind::IndentSlot => write!(fmt, "{}", self.indent.get())?,
                DomNodeKind::Text(text) => write!(fmt, "{}", text)?,
                DomNodeKind::Token(token) => write!(fmt, "{}", token.text())?,
            }
        }

        match &node.kind {
            DomNodeKind::Indent => self.indent.indent(),
            _ => {}
        }

        for child in node.children() {
            self.render_node(child, fmt, &node_id)?;
        }

        match &node.kind {
            DomNodeKind::Indent => self.indent.dedent(),
            _ => {}
        }

        Ok(())
    }
}

impl std::fmt::Debug for Layout<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for child in self.dom.children() {
            debug_node(
                fmt,
                &child,
                0,
                0,
                &mut NodeIdCounter::default(),
                &self.node_layouts,
            )?;
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

#[allow(clippy::write_with_newline)]
fn debug_node(
    fmt: &mut std::fmt::Formatter,
    node: &DomNode,
    indent_level: usize,
    indent_by: usize,
    node_ids: &mut NodeIdCounter,
    node_layouts: &HashMap<NodeId, NodeLayout>,
) -> std::fmt::Result {
    let node_id = node_ids.next();

    const INDENT_STEP: usize = 2;

    let indent = " ".repeat(indent_by);
    let indent = indent.as_str();
    let has_children = node.children().next().is_some();

    match &node.kind {
        DomNodeKind::Group => write!(fmt, "{indent}<group")?,
        DomNodeKind::Indent => write!(fmt, "{indent}<indent")?,
        DomNodeKind::IndentSlot => write!(fmt, "{indent}<indent_slot")?,
        DomNodeKind::Text(content) => write!(fmt, "{indent}<text content={content:?}")?,
        DomNodeKind::Token(token) => write!(fmt, "{indent}<token content=\"{token}\"")?,
    }

    match node.render_if {
        RenderIf::Always => {}
        RenderIf::Broken => write!(fmt, " render_if=broken")?,
        RenderIf::Flat => write!(fmt, " render_if=flat")?,
    }

    match node.break_if {
        BreakIf::ExceedsColumnLimit => {}
        BreakIf::Never => write!(fmt, " break_if=never")?,
        BreakIf::Always => write!(fmt, " break_if=always")?,
        BreakIf::SameAsParent => write!(fmt, " break_if=parent")?,
    }

    if !has_children {
        write!(fmt, " /")?;
    }

    let node_layout = node_layouts.get(&node_id).unwrap();
    write!(
        fmt,
        "> flat_width={} should_break={:?} indent_lvl={} node_id={}",
        node_layout.flat_width, node_layout.should_break, indent_level, node_id.0
    )?;

    for child in node.children() {
        write!(fmt, "\n")?;
        debug_node(
            fmt,
            &child,
            indent_level
                + if let DomNodeKind::Indent = node.kind {
                    1
                } else {
                    0
                },
            indent_by + INDENT_STEP,
            node_ids,
            node_layouts,
        )?;
    }

    if has_children {
        write!(fmt, "\n{indent}")?;

        match node.kind {
            DomNodeKind::Group => write!(fmt, "</group>")?,
            DomNodeKind::Indent => write!(fmt, "</indent>")?,
            DomNodeKind::IndentSlot => write!(fmt, "</indent_slot>")?,
            DomNodeKind::Text(_) => write!(fmt, "</text>")?,
            DomNodeKind::Token(_) => write!(fmt, "</token>")?,
        }
    }

    Ok(())
}
