use std::fmt::Debug;

use crate::{dom::BreakIf, Dom, DomNode, DomNodeKind, DomNodeMut, RenderIf};

impl Debug for Dom {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for child in self.children() {
            debug_node(fmt, &child, 0)?;
            write!(fmt, "\n")?;
        }

        Ok(())
    }
}

impl Debug for DomNode<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_node(fmt, self, 0)
    }
}

impl Debug for DomNodeMut<'_> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug_node(fmt, &self.as_ref(), 0)
    }
}

#[allow(clippy::write_with_newline)]
fn debug_node(fmt: &mut std::fmt::Formatter, node: &DomNode, indent_by: usize) -> std::fmt::Result {
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

    write!(fmt, ">")?;

    for child in node.children() {
        write!(fmt, "\n")?;
        debug_node(fmt, &child, indent_by + INDENT_STEP)?;
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
