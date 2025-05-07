use metal_ast::T;

use crate::{
    dom::{BreakIf, Dom, RenderIf as Rule},
    helpers::{
        AnnotationNodeExt, AstNodeExt, ImportBranchNodeExt, ImportItemNodeExt, ImportLeafNodeExt,
        ImportLeafRestNodeExt, ItemNodeExt, NameNodeExt, RootNodeExt, StmtNodeExt,
    },
};

impl TryFrom<metal_ast::RootNode> for Dom {
    type Error = crate::Error;

    fn try_from(value: metal_ast::RootNode) -> crate::Result<Self> {
        let mut dom = Dom::default();

        dom.group()
            .break_if(BreakIf::Never)
            .children(|dom| build_root_dom(value, dom))?;

        Ok(dom)
    }
}

fn build_root_dom(node: metal_ast::RootNode, dom: &mut Dom) -> crate::Result {
    for stmt in node.try_stmts()? {
        dom.group().children(|dom| build_stmt_dom(stmt, dom))?;
        dom.text("\n");
    }

    Ok(())
}

fn build_stmt_dom(node: metal_ast::StmtNode, dom: &mut Dom) -> crate::Result {
    match node.try_kind_node()? {
        metal_ast::StmtKindNode::Item(item) => build_item_dom(item, dom)?,
        metal_ast::StmtKindNode::Expr(_expr) => todo!(),
    }

    if node.try_kind_node()?.try_last_token()?.kind() != T!['}'] {
        dom.text(";");
    }

    Ok(())
}

fn build_item_dom(node: metal_ast::ItemNode, dom: &mut Dom) -> crate::Result {
    for ann in node.try_anns_node()? {
        build_annotation_dom(ann, dom)?;
        dom.text("\n").render_if(Rule::Broken);
        dom.text(" ").render_if(Rule::Flat);
    }

    build_vis_dom(node.try_vis_node()?, dom)?;

    match node.try_kind_node()? {
        metal_ast::ItemKindNode::AbstractItem(_node) => todo!(),
        metal_ast::ItemKindNode::ConstItem(_node) => todo!(),
        metal_ast::ItemKindNode::EnumItem(_node) => todo!(),
        metal_ast::ItemKindNode::FnItem(_node) => todo!(),
        metal_ast::ItemKindNode::ImportItem(node) => build_import_item_node(node, dom)?,
        metal_ast::ItemKindNode::StructItem(_node) => todo!(),
        metal_ast::ItemKindNode::TypeAliasItem(_node) => todo!(),
    }

    Ok(())
}

fn build_expr_dom(_node: metal_ast::ExprNode, _dom: &mut Dom) -> crate::Result {
    todo!()
}

fn build_annotation_dom(node: metal_ast::AnnotationNode, dom: &mut Dom) -> crate::Result {
    dom.text("@");
    build_expr_dom(node.try_expr_node()?, dom)?;

    Ok(())
}

fn build_vis_dom(node: metal_ast::VisNode, dom: &mut Dom) -> crate::Result {
    if let Some(token) = node.pub_token() {
        dom.token(token);
    }

    Ok(())
}

fn build_import_item_node(node: metal_ast::ImportItemNode, dom: &mut Dom) -> crate::Result {
    dom.text("import ");

    build_import_tree_node(node.try_tree_node()?, dom)?;

    Ok(())
}

fn build_import_tree_node(node: metal_ast::ImportTreeNode, dom: &mut Dom) -> crate::Result {
    match node {
        metal_ast::ImportTreeNode::ImportLeaf(node) => build_import_leaf_node(node, dom)?,
        metal_ast::ImportTreeNode::ImportBranch(node) => build_import_branch_node(node, dom)?,
    }

    Ok(())
}

fn build_import_leaf_node(node: metal_ast::ImportLeafNode, dom: &mut Dom) -> crate::Result {
    dom.token(node.try_segment_node()?.try_lit_ident_token()?);

    if let Some(rest) = node.rest_node() {
        build_import_leaf_rest_node(rest, dom)?;
    }

    Ok(())
}

fn build_import_leaf_rest_node(
    node: metal_ast::ImportLeafRestNode,
    dom: &mut Dom,
) -> crate::Result {
    dom.text(".");
    build_import_tree_node(node.try_subtree_node()?, dom)?;

    Ok(())
}

fn build_import_branch_node(node: metal_ast::ImportBranchNode, dom: &mut Dom) -> crate::Result {
    dom.group().children(|dom| {
        dom.text("{");
        dom.text("\n").render_if(Rule::Broken);

        dom.indent().children(|dom| {
            let mut subtrees = node.try_subtrees()?;

            if let Some(subtree) = subtrees.next() {
                build_import_tree_node(subtree, dom)?;
            }

            for subtree in subtrees {
                dom.text(",");
                dom.text(" ").render_if(Rule::Flat);
                dom.text("\n").render_if(Rule::Broken);
                build_import_tree_node(subtree, dom)?;
            }

            Ok(())
        })?;

        dom.text("\n").render_if(Rule::Broken);
        dom.text("}");

        Ok(())
    })?;

    Ok(())
}
