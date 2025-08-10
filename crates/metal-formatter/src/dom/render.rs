// SPDX-License-Identifier: MIT

use crate::{
    dom::BreakIf,
    utils::{PositionTracker, INDENT, NEWLINE},
    Dom, DomNode, DomNodeKind, DomNodeMut, RenderIf,
};

impl Dom {
    pub fn render(&mut self, to: &mut impl std::fmt::Write) -> std::fmt::Result {
        for mut child in self.children_mut() {
            measure(&mut child);
            break_(&mut child, &mut PositionTracker::default(), false);
            render(child.as_ref(), to, &mut PositionTracker::default(), false)?;
        }

        Ok(())
    }
}

fn measure(node: &mut DomNodeMut) {
    match &node.render_if {
        RenderIf::Always | RenderIf::Flat => {
            match &node.kind {
                DomNodeKind::Newline => node.width = NEWLINE.len(),
                DomNodeKind::Text(text) => node.width = text.len(),
                DomNodeKind::Token(token) => node.width = token.text().len(),
                DomNodeKind::Group | DomNodeKind::Indent | DomNodeKind::IndentSlot => {}
            }

            node.width += node
                .children_mut()
                .map(|mut child| {
                    measure(&mut child);

                    child.width
                })
                .sum::<usize>();
        }
        RenderIf::Broken => {}
    }
}

fn break_(node: &mut DomNodeMut, pos: &mut PositionTracker, is_parent_broken: bool) {
    match &node.kind {
        DomNodeKind::Group | DomNodeKind::Indent | DomNodeKind::Text(_) | DomNodeKind::Token(_) => {
            pos.column += node.width;
        }
        DomNodeKind::IndentSlot => pos.column += INDENT.len(),
        DomNodeKind::Newline => pos.column = pos.indent_level * INDENT.len(),
    }

    match &node.break_if {
        BreakIf::ExceedsColumnLimit => node.broken = pos.is_overboard(node.width),
        BreakIf::Never => node.broken = false,
        BreakIf::Always => node.broken = true,
        BreakIf::SameAsParent => node.broken = is_parent_broken,
    }

    if node.kind == DomNodeKind::Indent {
        pos.indent_level += 1;
    }

    let node_broken = node.broken;
    for mut child in node.children_mut() {
        break_(&mut child, pos, node_broken);
    }

    if node.kind == DomNodeKind::Indent {
        pos.indent_level -= 1;
    }
}

fn render(
    node: DomNode,
    to: &mut impl std::fmt::Write,
    pos: &mut PositionTracker,
    is_parent_broken: bool,
) -> std::fmt::Result {
    let should_render = match node.render_if {
        RenderIf::Always => true,
        RenderIf::Broken => is_parent_broken,
        RenderIf::Flat => !is_parent_broken,
    };

    if !should_render {
        return Ok(());
    }

    match &node.kind {
        DomNodeKind::Group | DomNodeKind::Indent => {}
        DomNodeKind::IndentSlot => write!(to, "{}", pos.indent_string())?,
        DomNodeKind::Newline => write!(to, "{}", NEWLINE)?,
        DomNodeKind::Text(text) => write!(to, "{}", text)?,
        DomNodeKind::Token(token) => write!(to, "{}", token.text())?,
    }

    if node.kind == DomNodeKind::Indent {
        pos.indent_level += 1;
    }

    for child in node.children() {
        render(child, to, pos, node.broken)?;
    }

    if node.kind == DomNodeKind::Indent {
        pos.indent_level -= 1;
    }

    Ok(())
}
