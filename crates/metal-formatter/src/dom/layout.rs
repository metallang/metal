use crate::{
    dom::BreakIf, utils::PositionTracker, Dom, DomNodeKind, DomNodeMut, RenderIf, INDENT, NEWLINE,
};

impl Dom {
    pub fn layout_and_render<W: std::fmt::Write>(
        &mut self,
        mut to: W,
    ) -> Result<W, std::fmt::Error> {
        let mut pos = PositionTracker::default();

        for mut child in self.children_mut() {
            measure(&mut child);
            layout_and_render(&mut child, &mut pos /*, &mut to, false */)?;
        }

        Ok(to)
    }
}

fn layout_and_render(
    node: &mut DomNodeMut,
    pos: &mut PositionTracker, /*, fmt: &mut impl std::fmt::Write, is_parent_broken: bool */
) -> std::fmt::Result {
    match &node.kind {
        DomNodeKind::Group | DomNodeKind::Indent | DomNodeKind::Text(_) | DomNodeKind::Token(_) => {
            pos.column += node.flat_width
        }
        DomNodeKind::IndentSlot => pos.column += INDENT.len(),
        DomNodeKind::Newline => pos.column = pos.indent_level * INDENT.len(),
    }

    match &node.break_if {
        BreakIf::ExceedsColumnLimit => node.broken = pos.is_overboard(node.flat_width),
        BreakIf::Never => node.broken = false,
        BreakIf::Always => node.broken = true,
        BreakIf::SameAsParent => {} // filled by the parent
    }

    // render here

    if node.kind == DomNodeKind::Indent {
        pos.indent_level += 1;
    }

    let node_broken = node.broken;
    for mut child in node.children_mut() {
        layout_and_render(&mut child, pos /*, fmt, node_broken */)?;

        if child.break_if == BreakIf::SameAsParent {
            child.broken = node_broken;
        }
    }

    if node.kind == DomNodeKind::Indent {
        pos.indent_level -= 1;
    }

    Ok(())
}

fn measure(node: &mut DomNodeMut) {
    match &node.render_if {
        RenderIf::Always | RenderIf::Flat => {
            match &node.kind {
                DomNodeKind::Newline => node.flat_width = NEWLINE.len(),
                DomNodeKind::Text(text) => node.flat_width = text.len(),
                DomNodeKind::Token(token) => node.flat_width = token.text().len(),
                DomNodeKind::Group | DomNodeKind::Indent | DomNodeKind::IndentSlot => {}
            }

            node.flat_width += node
                .children_mut()
                .map(|mut child| {
                    measure(&mut child);

                    child.flat_width
                })
                .sum::<usize>();
        }
        RenderIf::Broken => {}
    }
}

// pub fn render(&mut self) -> Result<String, std::fmt::Error> {
//     let mut buf = String::new();

//     for child in self.dom.children() {
//         self.render_node(child, &mut buf, &tribool::no)?;
//     }

//     self.node_ids.reset();

//     Ok(buf)
// }

// fn render_node(
//     &mut self,
//     node: DomNode,
//     fmt: &mut impl std::fmt::Write,
//     parent_broken: &tribool,
// ) -> std::fmt::Result {
//     let node_id = self.node_ids.next();

//     let should_render = match &node.render_if {
//         RenderIf::Always => true,
//         RenderIf::Broken => parent_broken == &tribool::yes,
//         RenderIf::Flat => parent_broken == &tribool::no,
//     };

//     if should_render {
//         match &node.kind {
//             DomNodeKind::Group | DomNodeKind::Indent => {}
//             DomNodeKind::IndentSlot => write!(fmt, "{}", self.indent.get())?,
//             DomNodeKind::Text(text) => write!(fmt, "{}", text)?,
//             DomNodeKind::Token(token) => write!(fmt, "{}", token.text())?,
//         }
//     }

//     if let DomNodeKind::Indent = &node.kind {
//         self.indent.increment()
//     }

//     let node_should_break = self.node_layouts.get(&node_id).unwrap().should_break;

//     for child in node.children() {
//         self.render_node(child, fmt, &node_should_break)?;
//     }

//     match &node.kind {
//         DomNodeKind::Indent => self.indent.decrement(),
//         _ => {}
//     }

//     Ok(())
// }
