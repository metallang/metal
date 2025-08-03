use crate::{utils::PositionTracker, Dom, DomNode, DomNodeKind, RenderIf, NEWLINE};

impl Dom {
    pub fn render(&self, to: &mut impl std::fmt::Write) -> std::fmt::Result {
        let mut pos = PositionTracker::default();

        for child in self.children() {
            render_node(child, to, &mut pos, false)?;
        }

        Ok(())
    }
}

fn render_node(
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
        render_node(child, to, pos, node.broken)?;
    }

    if node.kind == DomNodeKind::Indent {
        pos.indent_level -= 1;
    }

    Ok(())
}
