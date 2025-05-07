use crate::Error;

#[extend::ext(name = AnyIntoResult)]
impl<T> T {
    #[allow(clippy::disallowed_types)]
    fn ok<E>(self) -> Result<T, E> {
        Result::Ok(self)
    }
}

#[extend::ext(name = AstNodeExt)]
pub impl<N: metal_ast::AstNode> N {
    fn try_last_token(&self) -> crate::Result<metal_ast::SyntaxToken> {
        self.syntax().last_token().ok_or(Error::UnexpectedNone)
    }
}

#[extend::ext(name = RootNodeExt)]
pub impl metal_ast::RootNode {
    fn try_stmts(&self) -> crate::Result<impl Iterator<Item = metal_ast::StmtNode>> {
        self.block_stmts_node()
            .ok_or(Error::UnexpectedNone)?
            .stmt_nodes()
            .ok()
    }
}

#[extend::ext(name = StmtNodeExt)]
pub impl metal_ast::StmtNode {
    fn try_kind_node(&self) -> crate::Result<metal_ast::StmtKindNode> {
        self.kind_node().ok_or(Error::UnexpectedNone)
    }
}

#[extend::ext(name = ItemNodeExt)]
pub impl metal_ast::ItemNode {
    fn try_anns_node(&self) -> crate::Result<impl Iterator<Item = metal_ast::AnnotationNode>> {
        self.anns_node()
            .ok_or(Error::UnexpectedNone)?
            .annotation_nodes()
            .ok()
    }

    fn try_vis_node(&self) -> crate::Result<metal_ast::VisNode> {
        self.vis_node().ok_or(Error::UnexpectedNone)
    }

    fn try_kind_node(&self) -> crate::Result<metal_ast::ItemKindNode> {
        self.kind_node().ok_or(Error::UnexpectedNone)
    }
}

#[extend::ext(name = AnnotationNodeExt)]
pub impl metal_ast::AnnotationNode {
    fn try_expr_node(&self) -> crate::Result<metal_ast::ExprNode> {
        self.expr_node().ok_or(Error::UnexpectedNone)
    }
}

#[extend::ext(name = ImportItemNodeExt)]
pub impl metal_ast::ImportItemNode {
    fn try_tree_node(&self) -> crate::Result<metal_ast::ImportTreeNode> {
        self.tree_node().ok_or(Error::UnexpectedNone)
    }
}

#[extend::ext(name = ImportLeafNodeExt)]
pub impl metal_ast::ImportLeafNode {
    fn try_segment_node(&self) -> crate::Result<metal_ast::NameNode> {
        self.segment_node().ok_or(Error::UnexpectedNone)
    }

    fn try_rest_node(&self) -> crate::Result<metal_ast::ImportLeafRestNode> {
        self.rest_node().ok_or(Error::UnexpectedNone)
    }
}

#[extend::ext(name = NameNodeExt)]
pub impl metal_ast::NameNode {
    fn try_lit_ident_token(&self) -> crate::Result<metal_ast::SyntaxToken> {
        self.lit_ident_token().ok_or(Error::UnexpectedNone)
    }
}

#[extend::ext(name = ImportLeafRestNodeExt)]
pub impl metal_ast::ImportLeafRestNode {
    fn try_subtree_node(&self) -> crate::Result<metal_ast::ImportTreeNode> {
        self.subtree_node().ok_or(Error::UnexpectedNone)
    }
}

#[extend::ext(name = ImportBranchNodeExt)]
pub impl metal_ast::ImportBranchNode {
    fn try_subtrees(&self) -> crate::Result<impl Iterator<Item = metal_ast::ImportTreeNode>> {
        self.subtrees_node()
            .ok_or(Error::UnexpectedNone)?
            .children()
            .ok()
    }
}
