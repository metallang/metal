//! This file is @generated by the build script, do not edit by hand.
use crate::{AstNode, SyntaxKind, SyntaxNode, SyntaxNodeExt, SyntaxToken};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `Name` node.
pub struct NameNode {
    syntax: SyntaxNode,
}
impl AstNode for NameNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NAME_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl NameNode {
    /// Find a child token of variant [SyntaxKind::LIT_IDENT_TOKEN].
    pub fn lit_ident_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::LIT_IDENT_TOKEN)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `Visibility` node.
pub struct VisNode {
    syntax: SyntaxNode,
}
impl AstNode for VisNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::VIS_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl VisNode {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `Mutability` node.
pub struct MutnessNode {
    syntax: SyntaxNode,
}
impl AstNode for MutnessNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::MUTNESS_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl MutnessNode {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `ExprSpecifier` node.
pub struct ExprSpecNode {
    syntax: SyntaxNode,
}
impl AstNode for ExprSpecNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::EXPR_SPEC_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ExprSpecNode {
    /// Find a child token of variant [SyntaxKind::EQ_TOKEN].
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::EQ_TOKEN)
    }
    /// Find a child node of type [ExprNode].
    pub fn expr_node(&self) -> Option<ExprNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `Expr` node.
pub enum ExprNode {
    /// See [NameNode].
    Name(NameNode),
    /// See [PrefixExprNode].
    PrefixExpr(PrefixExprNode),
    /// See [BinaryExprNode].
    BinaryExpr(BinaryExprNode),
    /// See [CallExprNode].
    CallExpr(CallExprNode),
    /// See [LitExprNode].
    LitExpr(LitExprNode),
}
impl AstNode for ExprNode {
    #[allow(clippy::match_like_matches_macro)]
    #[allow(clippy::wildcard_enum_match_arm)]
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::NAME_NODE => true,
            SyntaxKind::PREFIX_EXPR_NODE => true,
            SyntaxKind::BINARY_EXPR_NODE => true,
            SyntaxKind::CALL_EXPR_NODE => true,
            SyntaxKind::LIT_EXPR_NODE => true,
            _ => false,
        }
    }
    #[allow(clippy::wildcard_enum_match_arm)]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::NAME_NODE => Some(ExprNode::Name(NameNode::cast(syntax)?)),
            SyntaxKind::PREFIX_EXPR_NODE => {
                Some(ExprNode::PrefixExpr(PrefixExprNode::cast(syntax)?))
            }
            SyntaxKind::BINARY_EXPR_NODE => {
                Some(ExprNode::BinaryExpr(BinaryExprNode::cast(syntax)?))
            }
            SyntaxKind::CALL_EXPR_NODE => {
                Some(ExprNode::CallExpr(CallExprNode::cast(syntax)?))
            }
            SyntaxKind::LIT_EXPR_NODE => {
                Some(ExprNode::LitExpr(LitExprNode::cast(syntax)?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ExprNode::Name(it) => it.syntax(),
            ExprNode::PrefixExpr(it) => it.syntax(),
            ExprNode::BinaryExpr(it) => it.syntax(),
            ExprNode::CallExpr(it) => it.syntax(),
            ExprNode::LitExpr(it) => it.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `TypeQualifier` node.
pub struct TypeQualNode {
    syntax: SyntaxNode,
}
impl AstNode for TypeQualNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TYPE_QUAL_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TypeQualNode {
    /// Find a child token of variant [SyntaxKind::COLON_TOKEN].
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::COLON_TOKEN)
    }
    /// Find a child node of type [TypeNode].
    pub fn type_node(&self) -> Option<TypeNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `Type` node.
pub struct TypeNode {
    syntax: SyntaxNode,
}
impl AstNode for TypeNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::TYPE_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl TypeNode {
    /// Find a child node of type [NameNode].
    pub fn name_node(&self) -> Option<NameNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `Block` node.
pub struct BlockNode {
    syntax: SyntaxNode,
}
impl AstNode for BlockNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BLOCK_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl BlockNode {
    /// Find a child token of variant [SyntaxKind::L_BRACE_TOKEN].
    pub fn l_brace_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::L_BRACE_TOKEN)
    }
    /// Find a child node of type [ItemNode].
    pub fn item_node(&self) -> Option<ItemNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::R_BRACE_TOKEN].
    pub fn r_brace_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::R_BRACE_TOKEN)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `Item` node.
pub enum ItemNode {
    /// See [ConstItemNode].
    ConstItem(ConstItemNode),
    /// See [EnumItemNode].
    EnumItem(EnumItemNode),
    /// See [ExprNode].
    Expr(ExprNode),
    /// See [FnItemNode].
    FnItem(FnItemNode),
    /// See [ImportItemNode].
    ImportItem(ImportItemNode),
    /// See [StructItemNode].
    StructItem(StructItemNode),
}
impl AstNode for ItemNode {
    #[allow(clippy::match_like_matches_macro)]
    #[allow(clippy::wildcard_enum_match_arm)]
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::CONST_ITEM_NODE => true,
            SyntaxKind::ENUM_ITEM_NODE => true,
            SyntaxKind::EXPR_NODE => true,
            SyntaxKind::FN_ITEM_NODE => true,
            SyntaxKind::IMPORT_ITEM_NODE => true,
            SyntaxKind::STRUCT_ITEM_NODE => true,
            _ => false,
        }
    }
    #[allow(clippy::wildcard_enum_match_arm)]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::CONST_ITEM_NODE => {
                Some(ItemNode::ConstItem(ConstItemNode::cast(syntax)?))
            }
            SyntaxKind::ENUM_ITEM_NODE => {
                Some(ItemNode::EnumItem(EnumItemNode::cast(syntax)?))
            }
            SyntaxKind::EXPR_NODE => Some(ItemNode::Expr(ExprNode::cast(syntax)?)),
            SyntaxKind::FN_ITEM_NODE => Some(ItemNode::FnItem(FnItemNode::cast(syntax)?)),
            SyntaxKind::IMPORT_ITEM_NODE => {
                Some(ItemNode::ImportItem(ImportItemNode::cast(syntax)?))
            }
            SyntaxKind::STRUCT_ITEM_NODE => {
                Some(ItemNode::StructItem(StructItemNode::cast(syntax)?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ItemNode::ConstItem(it) => it.syntax(),
            ItemNode::EnumItem(it) => it.syntax(),
            ItemNode::Expr(it) => it.syntax(),
            ItemNode::FnItem(it) => it.syntax(),
            ItemNode::ImportItem(it) => it.syntax(),
            ItemNode::StructItem(it) => it.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `ConstItem` node.
pub struct ConstItemNode {
    syntax: SyntaxNode,
}
impl AstNode for ConstItemNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CONST_ITEM_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ConstItemNode {
    /// Find a child node of type [VisNode].
    pub fn vis_node(&self) -> Option<VisNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::CONST_TOKEN].
    pub fn const_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::CONST_TOKEN)
    }
    /// Find a child node of type [NameNode].
    pub fn name_node(&self) -> Option<NameNode> {
        self.syntax.child()
    }
    /// Find a child node of type [ExprSpecNode].
    pub fn value_node(&self) -> Option<ExprSpecNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `EnumItem` node.
pub struct EnumItemNode {
    syntax: SyntaxNode,
}
impl AstNode for EnumItemNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENUM_ITEM_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl EnumItemNode {
    /// Find a child node of type [VisNode].
    pub fn vis_node(&self) -> Option<VisNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::ENUM_TOKEN].
    pub fn enum_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::ENUM_TOKEN)
    }
    /// Find a child node of type [NameNode].
    pub fn name_node(&self) -> Option<NameNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::L_BRACE_TOKEN].
    pub fn l_brace_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::L_BRACE_TOKEN)
    }
    /// Find a child token of variant [SyntaxKind::R_BRACE_TOKEN].
    pub fn r_brace_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::R_BRACE_TOKEN)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `FnItem` node.
pub struct FnItemNode {
    syntax: SyntaxNode,
}
impl AstNode for FnItemNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_ITEM_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnItemNode {
    /// Find a child node of type [VisNode].
    pub fn vis_node(&self) -> Option<VisNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::DEF_TOKEN].
    pub fn def_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::DEF_TOKEN)
    }
    /// Find a child node of type [NameNode].
    pub fn name_node(&self) -> Option<NameNode> {
        self.syntax.child()
    }
    /// Find a child node of type [FnSignatureNode].
    pub fn sig_node(&self) -> Option<FnSignatureNode> {
        self.syntax.child()
    }
    /// Find a child node of type [BlockNode].
    pub fn body_node(&self) -> Option<BlockNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `ImportItem` node.
pub struct ImportItemNode {
    syntax: SyntaxNode,
}
impl AstNode for ImportItemNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IMPORT_ITEM_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ImportItemNode {
    /// Find a child node of type [VisNode].
    pub fn vis_node(&self) -> Option<VisNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::IMPORT_TOKEN].
    pub fn import_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::IMPORT_TOKEN)
    }
    /// Find a child node of type [ImportTreeNode].
    pub fn tree_node(&self) -> Option<ImportTreeNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::SEMICOLON_TOKEN].
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::SEMICOLON_TOKEN)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `StructItem` node.
pub struct StructItemNode {
    syntax: SyntaxNode,
}
impl AstNode for StructItemNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STRUCT_ITEM_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl StructItemNode {
    /// Find a child node of type [VisNode].
    pub fn vis_node(&self) -> Option<VisNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::STRUCT_TOKEN].
    pub fn struct_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::STRUCT_TOKEN)
    }
    /// Find a child node of type [NameNode].
    pub fn name_node(&self) -> Option<NameNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::L_BRACE_TOKEN].
    pub fn l_brace_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::L_BRACE_TOKEN)
    }
    /// Find a child node of type [StructBodyNode].
    pub fn body_node(&self) -> Option<StructBodyNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::R_BRACE_TOKEN].
    pub fn r_brace_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::R_BRACE_TOKEN)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `EnumBody` node.
pub struct EnumBodyNode {
    syntax: SyntaxNode,
}
impl AstNode for EnumBodyNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENUM_BODY_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl EnumBodyNode {
    /// Find a child node of type [EnumBodyItemNode].
    pub fn enum_body_item_node(&self) -> Option<EnumBodyItemNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `EnumBodyItem` node.
pub enum EnumBodyItemNode {
    /// See [EnumVariantNode].
    EnumVariant(EnumVariantNode),
    /// See [FnItemNode].
    FnItem(FnItemNode),
}
impl AstNode for EnumBodyItemNode {
    #[allow(clippy::match_like_matches_macro)]
    #[allow(clippy::wildcard_enum_match_arm)]
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::ENUM_VARIANT_NODE => true,
            SyntaxKind::FN_ITEM_NODE => true,
            _ => false,
        }
    }
    #[allow(clippy::wildcard_enum_match_arm)]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::ENUM_VARIANT_NODE => {
                Some(EnumBodyItemNode::EnumVariant(EnumVariantNode::cast(syntax)?))
            }
            SyntaxKind::FN_ITEM_NODE => {
                Some(EnumBodyItemNode::FnItem(FnItemNode::cast(syntax)?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            EnumBodyItemNode::EnumVariant(it) => it.syntax(),
            EnumBodyItemNode::FnItem(it) => it.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `EnumVariant` node.
pub struct EnumVariantNode {
    syntax: SyntaxNode,
}
impl AstNode for EnumVariantNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENUM_VARIANT_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl EnumVariantNode {
    /// Find a child node of type [NameNode].
    pub fn name_node(&self) -> Option<NameNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `EnumVariantType` node.
pub struct EnumVariantTypeNode {
    syntax: SyntaxNode,
}
impl AstNode for EnumVariantTypeNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::ENUM_VARIANT_TYPE_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl EnumVariantTypeNode {
    /// Find a child token of variant [SyntaxKind::L_PAREN_TOKEN].
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::L_PAREN_TOKEN)
    }
    /// Find a child node of type [TypeNode].
    pub fn type_node(&self) -> Option<TypeNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::R_PAREN_TOKEN].
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::R_PAREN_TOKEN)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `FnSignature` node.
pub struct FnSignatureNode {
    syntax: SyntaxNode,
}
impl AstNode for FnSignatureNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_SIGNATURE_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnSignatureNode {
    /// Find a child token of variant [SyntaxKind::L_PAREN_TOKEN].
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::L_PAREN_TOKEN)
    }
    /// Find a child token of variant [SyntaxKind::R_PAREN_TOKEN].
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::R_PAREN_TOKEN)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `FnInput` node.
pub struct FnInputNode {
    syntax: SyntaxNode,
}
impl AstNode for FnInputNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::FN_INPUT_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl FnInputNode {
    /// Find a child node of type [MutnessNode].
    pub fn mutness_node(&self) -> Option<MutnessNode> {
        self.syntax.child()
    }
    /// Find a child node of type [NameNode].
    pub fn name_node(&self) -> Option<NameNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `ImportTree` node.
pub enum ImportTreeNode {
    /// See [ImportLeafNode].
    ImportLeaf(ImportLeafNode),
    /// See [ImportSegmentNode].
    ImportSegment(ImportSegmentNode),
    /// See [ImportBranchNode].
    ImportBranch(ImportBranchNode),
}
impl AstNode for ImportTreeNode {
    #[allow(clippy::match_like_matches_macro)]
    #[allow(clippy::wildcard_enum_match_arm)]
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::IMPORT_LEAF_NODE => true,
            SyntaxKind::IMPORT_SEGMENT_NODE => true,
            SyntaxKind::IMPORT_BRANCH_NODE => true,
            _ => false,
        }
    }
    #[allow(clippy::wildcard_enum_match_arm)]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::IMPORT_LEAF_NODE => {
                Some(ImportTreeNode::ImportLeaf(ImportLeafNode::cast(syntax)?))
            }
            SyntaxKind::IMPORT_SEGMENT_NODE => {
                Some(ImportTreeNode::ImportSegment(ImportSegmentNode::cast(syntax)?))
            }
            SyntaxKind::IMPORT_BRANCH_NODE => {
                Some(ImportTreeNode::ImportBranch(ImportBranchNode::cast(syntax)?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ImportTreeNode::ImportLeaf(it) => it.syntax(),
            ImportTreeNode::ImportSegment(it) => it.syntax(),
            ImportTreeNode::ImportBranch(it) => it.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `ImportLeaf` node.
pub struct ImportLeafNode {
    syntax: SyntaxNode,
}
impl AstNode for ImportLeafNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IMPORT_LEAF_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ImportLeafNode {
    /// Find a child node of type [NameNode].
    pub fn name_node(&self) -> Option<NameNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `ImportSegment` node.
pub struct ImportSegmentNode {
    syntax: SyntaxNode,
}
impl AstNode for ImportSegmentNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IMPORT_SEGMENT_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ImportSegmentNode {
    /// Find a child node of type [NameNode].
    pub fn segment_node(&self) -> Option<NameNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::DOT_TOKEN].
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::DOT_TOKEN)
    }
    /// Find a child node of type [ImportTreeNode].
    pub fn rest_node(&self) -> Option<ImportTreeNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `ImportBranch` node.
pub struct ImportBranchNode {
    syntax: SyntaxNode,
}
impl AstNode for ImportBranchNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::IMPORT_BRANCH_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl ImportBranchNode {
    /// Find a child token of variant [SyntaxKind::L_BRACE_TOKEN].
    pub fn l_brace_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::L_BRACE_TOKEN)
    }
    /// Find a child token of variant [SyntaxKind::R_BRACE_TOKEN].
    pub fn r_brace_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::R_BRACE_TOKEN)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `StructBody` node.
pub struct StructBodyNode {
    syntax: SyntaxNode,
}
impl AstNode for StructBodyNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STRUCT_BODY_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl StructBodyNode {
    /// Find a child node of type [StructBodyItemNode].
    pub fn struct_body_item_node(&self) -> Option<StructBodyItemNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `StructBodyItem` node.
pub enum StructBodyItemNode {
    /// See [StructFieldNode].
    StructField(StructFieldNode),
    /// See [FnItemNode].
    FnItem(FnItemNode),
}
impl AstNode for StructBodyItemNode {
    #[allow(clippy::match_like_matches_macro)]
    #[allow(clippy::wildcard_enum_match_arm)]
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::STRUCT_FIELD_NODE => true,
            SyntaxKind::FN_ITEM_NODE => true,
            _ => false,
        }
    }
    #[allow(clippy::wildcard_enum_match_arm)]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::STRUCT_FIELD_NODE => {
                Some(StructBodyItemNode::StructField(StructFieldNode::cast(syntax)?))
            }
            SyntaxKind::FN_ITEM_NODE => {
                Some(StructBodyItemNode::FnItem(FnItemNode::cast(syntax)?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            StructBodyItemNode::StructField(it) => it.syntax(),
            StructBodyItemNode::FnItem(it) => it.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `StructField` node.
pub struct StructFieldNode {
    syntax: SyntaxNode,
}
impl AstNode for StructFieldNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STRUCT_FIELD_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl StructFieldNode {
    /// Find a child node of type [VisNode].
    pub fn vis_node(&self) -> Option<VisNode> {
        self.syntax.child()
    }
    /// Find a child node of type [NameNode].
    pub fn name_node(&self) -> Option<NameNode> {
        self.syntax.child()
    }
    /// Find a child node of type [TypeQualNode].
    pub fn ty_node(&self) -> Option<TypeQualNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `PrefixExpr` node.
pub struct PrefixExprNode {
    syntax: SyntaxNode,
}
impl AstNode for PrefixExprNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PREFIX_EXPR_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl PrefixExprNode {
    /// Find a child node of type [PrefixOpNode].
    pub fn op_node(&self) -> Option<PrefixOpNode> {
        self.syntax.child()
    }
    /// Find a child node of type [ExprNode].
    pub fn expr_node(&self) -> Option<ExprNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `BinaryExpr` node.
pub struct BinaryExprNode {
    syntax: SyntaxNode,
}
impl AstNode for BinaryExprNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BINARY_EXPR_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl BinaryExprNode {
    /// Find a child node of type [ExprNode].
    pub fn lhs_node(&self) -> Option<ExprNode> {
        self.syntax.child()
    }
    /// Find a child node of type [BinaryOpNode].
    pub fn op_node(&self) -> Option<BinaryOpNode> {
        self.syntax.child()
    }
    /// Find a child node of type [ExprNode].
    pub fn rhs_node(&self) -> Option<ExprNode> {
        self.syntax.child()
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `CallExpr` node.
pub struct CallExprNode {
    syntax: SyntaxNode,
}
impl AstNode for CallExprNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::CALL_EXPR_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl CallExprNode {
    /// Find a child node of type [ExprNode].
    pub fn expr_node(&self) -> Option<ExprNode> {
        self.syntax.child()
    }
    /// Find a child token of variant [SyntaxKind::L_PAREN_TOKEN].
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::L_PAREN_TOKEN)
    }
    /// Find a child token of variant [SyntaxKind::R_PAREN_TOKEN].
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::R_PAREN_TOKEN)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `LitExpr` node.
pub enum LitExprNode {
    /// See [NumLitNode].
    NumLit(NumLitNode),
    /// See [StrLitNode].
    StrLit(StrLitNode),
}
impl AstNode for LitExprNode {
    #[allow(clippy::match_like_matches_macro)]
    #[allow(clippy::wildcard_enum_match_arm)]
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SyntaxKind::NUM_LIT_NODE => true,
            SyntaxKind::STR_LIT_NODE => true,
            _ => false,
        }
    }
    #[allow(clippy::wildcard_enum_match_arm)]
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        match syntax.kind() {
            SyntaxKind::NUM_LIT_NODE => {
                Some(LitExprNode::NumLit(NumLitNode::cast(syntax)?))
            }
            SyntaxKind::STR_LIT_NODE => {
                Some(LitExprNode::StrLit(StrLitNode::cast(syntax)?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            LitExprNode::NumLit(it) => it.syntax(),
            LitExprNode::StrLit(it) => it.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `PrefixOp` node.
pub struct PrefixOpNode {
    syntax: SyntaxNode,
}
impl AstNode for PrefixOpNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::PREFIX_OP_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `BinaryOp` node.
pub struct BinaryOpNode {
    syntax: SyntaxNode,
}
impl AstNode for BinaryOpNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BINARY_OP_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `NumLit` node.
pub struct NumLitNode {
    syntax: SyntaxNode,
}
impl AstNode for NumLitNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::NUM_LIT_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl NumLitNode {
    /// Find a child token of variant [SyntaxKind::LIT_NUM_TOKEN].
    pub fn lit_num_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::LIT_NUM_TOKEN)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Represents the `StrLit` node.
pub struct StrLitNode {
    syntax: SyntaxNode,
}
impl AstNode for StrLitNode {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::STR_LIT_NODE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl StrLitNode {
    /// Find a child token of variant [SyntaxKind::LIT_STR_TOKEN].
    pub fn lit_str_token(&self) -> Option<SyntaxToken> {
        self.syntax.find_child_token(SyntaxKind::LIT_STR_TOKEN)
    }
}
