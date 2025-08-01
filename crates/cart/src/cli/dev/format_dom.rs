// SPDX-License-Identifier: MIT

use metal_formatter::{BreakIf, RenderIf};

use crate::error::Error;

pub struct DevFormatDomCommand {
    path: String,
}

impl tapcli::Command for DevFormatDomCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let Some(tapcli::Arg::Value(path)) = parser.next() else {
            panic!("expected a file path");
        };

        Ok(Self { path })
    }

    fn run(self) -> Result<Self::Output, Self::Error> {
        let contents = std::fs::read_to_string(self.path).unwrap();

        let tokens = metal_lexer::Lexer::new(&contents).collect();
        let mut parser = metal_parser::Parser::new(tokens, &contents);

        metal_parser::parse_root(&mut parser);

        let dom = metal_formatter::Dom::try_from(parser.finish()).unwrap();

        // let mut layout = metal_formatter::LayoutState::default();

        // layout.compute_flat_widths(&dom);
        // layout.compute_should_breaks(&dom);

        // layout.dbg_nodes();

        // let mut dom = metal_formatter::Dom::default();

        // dom.group()
        //     .break_if(BreakIf::Never)
        //     .render_if(RenderIf::Flat)
        //     .children(|dom| {
        //         dom.text("1");
        //         dom.group().children(|dom| {
        //             dom.text("2");
        //             dom.text("3");

        //             Ok(())
        //         })?;
        //         dom.text("4");

        //         Ok(())
        //     })
        //     .unwrap();

        // dom.text("hi");

        let mut layout = metal_formatter::Layout::new(&dom);

        layout.compute_flat_widths();
        layout.compute_should_breaks();

        dbg!(&layout);

        std::fs::write("./test_out.mt", layout.render().unwrap()).unwrap();

        Ok(())
    }
}
