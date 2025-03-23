// SPDX-License-Identifier: MIT

//! This file is @generated by the build script, do not edit by hand.
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SyntaxKind {
    /// Don't try to remember this! Use [`N![Root]`](N) instead.
    ROOT_NODE,
    /// Don't try to remember this! Use [`N![BlockStmts]`](N) instead.
    BLOCK_STMTS_NODE,
    /// Don't try to remember this! Use [`N![Block]`](N) instead.
    BLOCK_NODE,
    /// Don't try to remember this! Use [`N![Stmt]`](N) instead.
    STMT_NODE,
    /// Don't try to remember this! Use [`N![Name]`](N) instead.
    NAME_NODE,
    /// Don't try to remember this! Use [`N![Visibility]`](N) instead.
    VIS_NODE,
    /// Don't try to remember this! Use [`N![Mutability]`](N) instead.
    MUTNESS_NODE,
    /// Don't try to remember this! Use [`N![ExprSpecifier]`](N) instead.
    EXPR_SPEC_NODE,
    /// Don't try to remember this! Use [`N![Expr]`](N) instead.
    EXPR_NODE,
    /// Don't try to remember this! Use [`N![TypeQualifier]`](N) instead.
    TYPE_QUAL_NODE,
    /// Don't try to remember this! Use [`N![Type]`](N) instead.
    TYPE_NODE,
    /// Don't try to remember this! Use [`N![TypeSpecifier]`](N) instead.
    TYPE_SPECIFIER_NODE,
    /// Don't try to remember this! Use [`N![StmtKind]`](N) instead.
    STMT_KIND_NODE,
    /// Don't try to remember this! Use [`N![Item]`](N) instead.
    ITEM_NODE,
    /// Don't try to remember this! Use [`N![Annotations]`](N) instead.
    ANNOTATIONS_NODE,
    /// Don't try to remember this! Use [`N![ItemKind]`](N) instead.
    ITEM_KIND_NODE,
    /// Don't try to remember this! Use [`N![AbstractItem]`](N) instead.
    ABSTRACT_ITEM_NODE,
    /// Don't try to remember this! Use [`N![ConstItem]`](N) instead.
    CONST_ITEM_NODE,
    /// Don't try to remember this! Use [`N![EnumItem]`](N) instead.
    ENUM_ITEM_NODE,
    /// Don't try to remember this! Use [`N![FnItem]`](N) instead.
    FN_ITEM_NODE,
    /// Don't try to remember this! Use [`N![ImportItem]`](N) instead.
    IMPORT_ITEM_NODE,
    /// Don't try to remember this! Use [`N![StructItem]`](N) instead.
    STRUCT_ITEM_NODE,
    /// Don't try to remember this! Use [`N![TypeAliasItem]`](N) instead.
    TYPE_ALIAS_ITEM_NODE,
    /// Don't try to remember this! Use [`N![Annotation]`](N) instead.
    ANNOTATION_NODE,
    /// Don't try to remember this! Use [`N![GenericParamList]`](N) instead.
    GENERIC_PARAM_LIST_NODE,
    /// Don't try to remember this! Use [`N![AbstractBody]`](N) instead.
    ABSTRACT_BODY_NODE,
    /// Don't try to remember this! Use [`N![AbstractFnItem]`](N) instead.
    ABSTRACT_FN_ITEM_NODE,
    /// Don't try to remember this! Use [`N![FnSignature]`](N) instead.
    FN_SIGNATURE_NODE,
    /// Don't try to remember this! Use [`N![EnumBody]`](N) instead.
    ENUM_BODY_NODE,
    /// Don't try to remember this! Use [`N![EnumBodyItem]`](N) instead.
    ENUM_BODY_ITEM_NODE,
    /// Don't try to remember this! Use [`N![EnumVariant]`](N) instead.
    ENUM_VARIANT_NODE,
    /// Don't try to remember this! Use [`N![EnumFn]`](N) instead.
    ENUM_FN_NODE,
    /// Don't try to remember this! Use [`N![EnumVariantDataType]`](N) instead.
    ENUM_VARIANT_DATA_TYPE_NODE,
    /// Don't try to remember this! Use [`N![FnInputs]`](N) instead.
    FN_INPUTS_NODE,
    /// Don't try to remember this! Use [`N![FnInput]`](N) instead.
    FN_INPUT_NODE,
    /// Don't try to remember this! Use [`N![ImportTree]`](N) instead.
    IMPORT_TREE_NODE,
    /// Don't try to remember this! Use [`N![ImportLeaf]`](N) instead.
    IMPORT_LEAF_NODE,
    /// Don't try to remember this! Use [`N![ImportBranch]`](N) instead.
    IMPORT_BRANCH_NODE,
    /// Don't try to remember this! Use [`N![ImportSegment]`](N) instead.
    IMPORT_SEGMENT_NODE,
    /// Don't try to remember this! Use [`N![ImportBranchSubtrees]`](N) instead.
    IMPORT_BRANCH_SUBTREES_NODE,
    /// Don't try to remember this! Use [`N![StructBody]`](N) instead.
    STRUCT_BODY_NODE,
    /// Don't try to remember this! Use [`N![StructBodyItem]`](N) instead.
    STRUCT_BODY_ITEM_NODE,
    /// Don't try to remember this! Use [`N![StructField]`](N) instead.
    STRUCT_FIELD_NODE,
    /// Don't try to remember this! Use [`N![StructFn]`](N) instead.
    STRUCT_FN_NODE,
    /// Don't try to remember this! Use [`N![NameType]`](N) instead.
    NAME_TYPE_NODE,
    /// Don't try to remember this! Use [`N![RefType]`](N) instead.
    REF_TYPE_NODE,
    /// Don't try to remember this! Use [`N![BinaryType]`](N) instead.
    BINARY_TYPE_NODE,
    /// Don't try to remember this! Use [`N![NameTypeGenerics]`](N) instead.
    NAME_TYPE_GENERICS_NODE,
    /// Don't try to remember this! Use [`N![NameTypeGenericsInner]`](N) instead.
    NAME_TYPE_GENERICS_INNER_NODE,
    /// Don't try to remember this! Use [`N![BinaryTypeOp]`](N) instead.
    BINARY_TYPE_OP_NODE,
    /// Don't try to remember this! Use [`N![PrefixExpr]`](N) instead.
    PREFIX_EXPR_NODE,
    /// Don't try to remember this! Use [`N![BinaryExpr]`](N) instead.
    BINARY_EXPR_NODE,
    /// Don't try to remember this! Use [`N![CallExpr]`](N) instead.
    CALL_EXPR_NODE,
    /// Don't try to remember this! Use [`N![LitExpr]`](N) instead.
    LIT_EXPR_NODE,
    /// Don't try to remember this! Use [`N![ParenExpr]`](N) instead.
    PAREN_EXPR_NODE,
    /// Don't try to remember this! Use [`N![ReturnExpr]`](N) instead.
    RETURN_EXPR_NODE,
    /// Don't try to remember this! Use [`N![IfExpr]`](N) instead.
    IF_EXPR_NODE,
    /// Don't try to remember this! Use [`N![DeferExpr]`](N) instead.
    DEFER_EXPR_NODE,
    /// Don't try to remember this! Use [`N![StructExpr]`](N) instead.
    STRUCT_EXPR_NODE,
    /// Don't try to remember this! Use [`N![PrefixExprOp]`](N) instead.
    PREFIX_EXPR_OP_NODE,
    /// Don't try to remember this! Use [`N![BinaryExprOp]`](N) instead.
    BINARY_EXPR_OP_NODE,
    /// Don't try to remember this! Use [`N![CallExprArgs]`](N) instead.
    CALL_EXPR_ARGS_NODE,
    /// Don't try to remember this! Use [`N![IfExprElseClause]`](N) instead.
    IF_EXPR_ELSE_CLAUSE_NODE,
    /// Don't try to remember this! Use [`N![StructExprFields]`](N) instead.
    STRUCT_EXPR_FIELDS_NODE,
    /// Don't try to remember this! Use [`N![StructExprField]`](N) instead.
    STRUCT_EXPR_FIELD_NODE,
    /// Don't try to remember this! Use [`N![GenericParams]`](N) instead.
    GENERIC_PARAMS_NODE,
    /// Don't try to remember this! Use [`N![GenericParam]`](N) instead.
    GENERIC_PARAM_NODE,
    /// Don't try to remember this! Use [`T!['{']`](T) instead.
    L_BRACE_TOKEN,
    /// Don't try to remember this! Use [`T!['}']`](T) instead.
    R_BRACE_TOKEN,
    /// Don't try to remember this! Use [`T![@ident]`](T) instead.
    LIT_IDENT_TOKEN,
    /// Don't try to remember this! Use [`T![pub]`](T) instead.
    PUB_TOKEN,
    /// Don't try to remember this! Use [`T![mut]`](T) instead.
    MUT_TOKEN,
    /// Don't try to remember this! Use [`T![=]`](T) instead.
    EQ_TOKEN,
    /// Don't try to remember this! Use [`T![:]`](T) instead.
    COLON_TOKEN,
    /// Don't try to remember this! Use [`T![;]`](T) instead.
    SEMICOLON_TOKEN,
    /// Don't try to remember this! Use [`T![@]`](T) instead.
    AT_TOKEN,
    /// Don't try to remember this! Use [`T![abstract]`](T) instead.
    ABSTRACT_TOKEN,
    /// Don't try to remember this! Use [`T![def]`](T) instead.
    DEF_TOKEN,
    /// Don't try to remember this! Use [`T![const]`](T) instead.
    CONST_TOKEN,
    /// Don't try to remember this! Use [`T![enum]`](T) instead.
    ENUM_TOKEN,
    /// Don't try to remember this! Use [`T!['(']`](T) instead.
    L_PAREN_TOKEN,
    /// Don't try to remember this! Use [`T![')']`](T) instead.
    R_PAREN_TOKEN,
    /// Don't try to remember this! Use [`T![,]`](T) instead.
    COMMA_TOKEN,
    /// Don't try to remember this! Use [`T![import]`](T) instead.
    IMPORT_TOKEN,
    /// Don't try to remember this! Use [`T![.]`](T) instead.
    DOT_TOKEN,
    /// Don't try to remember this! Use [`T![struct]`](T) instead.
    STRUCT_TOKEN,
    /// Don't try to remember this! Use [`T![type]`](T) instead.
    TYPE_TOKEN,
    /// Don't try to remember this! Use [`T!['[']`](T) instead.
    L_BRACKET_TOKEN,
    /// Don't try to remember this! Use [`T![']']`](T) instead.
    R_BRACKET_TOKEN,
    /// Don't try to remember this! Use [`T![&]`](T) instead.
    AMP_TOKEN,
    /// Don't try to remember this! Use [`T![+]`](T) instead.
    PLUS_TOKEN,
    /// Don't try to remember this! Use [`T![-]`](T) instead.
    MINUS_TOKEN,
    /// Don't try to remember this! Use [`T![!]`](T) instead.
    BANG_TOKEN,
    /// Don't try to remember this! Use [`T![~]`](T) instead.
    TILDE_TOKEN,
    /// Don't try to remember this! Use [`T![*]`](T) instead.
    STAR_TOKEN,
    /// Don't try to remember this! Use [`T![+=]`](T) instead.
    PLUS_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![-=]`](T) instead.
    MINUS_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![/=]`](T) instead.
    SLASH_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![*=]`](T) instead.
    STAR_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![**=]`](T) instead.
    STAR2_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![%=]`](T) instead.
    PERCENT_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![^=]`](T) instead.
    CARET_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![&=]`](T) instead.
    AMP_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![|=]`](T) instead.
    PIPE_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![<<=]`](T) instead.
    LT2_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![>>=]`](T) instead.
    GT2_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![/]`](T) instead.
    SLASH_TOKEN,
    /// Don't try to remember this! Use [`T![**]`](T) instead.
    STAR2_TOKEN,
    /// Don't try to remember this! Use [`T![%]`](T) instead.
    PERCENT_TOKEN,
    /// Don't try to remember this! Use [`T![&&]`](T) instead.
    AMP2_TOKEN,
    /// Don't try to remember this! Use [`T![||]`](T) instead.
    PIPE2_TOKEN,
    /// Don't try to remember this! Use [`T![==]`](T) instead.
    EQ2_TOKEN,
    /// Don't try to remember this! Use [`T![!=]`](T) instead.
    BANG_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![>]`](T) instead.
    GT_TOKEN,
    /// Don't try to remember this! Use [`T![>=]`](T) instead.
    GT_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![<]`](T) instead.
    LT_TOKEN,
    /// Don't try to remember this! Use [`T![<=]`](T) instead.
    LT_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![^]`](T) instead.
    CARET_TOKEN,
    /// Don't try to remember this! Use [`T![|]`](T) instead.
    PIPE_TOKEN,
    /// Don't try to remember this! Use [`T![<<]`](T) instead.
    LT2_TOKEN,
    /// Don't try to remember this! Use [`T![>>]`](T) instead.
    GT2_TOKEN,
    /// Don't try to remember this! Use [`T![..]`](T) instead.
    DOT2_TOKEN,
    /// Don't try to remember this! Use [`T![@number]`](T) instead.
    LIT_NUM_TOKEN,
    /// Don't try to remember this! Use [`T![@string]`](T) instead.
    LIT_STR_TOKEN,
    /// Don't try to remember this! Use [`T![return]`](T) instead.
    RETURN_TOKEN,
    /// Don't try to remember this! Use [`T![if]`](T) instead.
    IF_TOKEN,
    /// Don't try to remember this! Use [`T![else]`](T) instead.
    ELSE_TOKEN,
    /// Don't try to remember this! Use [`T![defer]`](T) instead.
    DEFER_TOKEN,
    /// Represents a multi- or single-line comment.
    COMMENT_TOKEN,
    /// Represents a whitespace token, such as a space or a tab, among others.
    WHITESPACE_TOKEN,
    /// A special token representing an unknown token.
    UNKNOWN_TOKEN,
    /// A special syntax kind used for transmute safety checks. You shouldn't worry
    /// about (and even less rely on) this.
    __LAST,
}
impl From<rowan::SyntaxKind> for SyntaxKind {
    fn from(value: rowan::SyntaxKind) -> Self {
        let d = value.0 as u8;
        assert!(d <= (SyntaxKind::__LAST as u8));
        unsafe { std::mem::transmute::<u8, SyntaxKind>(d) }
    }
}
impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(val: SyntaxKind) -> Self {
        rowan::SyntaxKind(val as u16)
    }
}
impl SyntaxKind {
    pub fn is_whitespace(&self) -> bool {
        matches!(self, T![@ comment] | T![@ whitespace] | T![@ unknown])
    }
    pub fn is_item_start(&self) -> bool {
        matches!(
            self,
            T![abstract]
            | T![const]
            | T![enum]
            | T![def]
            | T![import]
            | T![struct]
            | T![type]
            | T![pub]
            | T![@]
        )
    }
    pub fn is_prefix_op(&self) -> bool {
        matches!(self, T![+] | T![-] | T![!] | T![~] | T![*])
    }
    pub fn is_expr_start(&self) -> bool {
        matches!(
            self,
            T![@ ident]
            | T!['{']
            | T![@ number]
            | T![@ string]
            | T!['(']
            | T![return]
            | T![if]
            | T![defer]
        ) || self.is_prefix_op()
    }
}
/// Returns the [SyntaxKind] variant corresponding to the provided token
/// as written in the grammar.
///
/// Note that certain tokens such as parentheses, braces, and brackets need
/// to be wrapped in single quotes (like you would spell a character), e.g.
/// `T!['{']`. This is a limitation imposed by Rust.
///
/// # Example
///
/// ```no_run,
/// # use metal_ast::{T, AstToken, BinaryExprOpNode};
/// # fn example(binary_op_node: BinaryExprOpNode) {
/// if binary_op_node.token().is_some_and(|token| token.syntax().kind() == T![+]) {
///     // The binary operator is plus!
/// }
/// # }
/// ```
pub macro T {
    ['{'] => { $crate::SyntaxKind::L_BRACE_TOKEN },
    ['}'] => { $crate::SyntaxKind::R_BRACE_TOKEN },
    [@ident] => { $crate::SyntaxKind::LIT_IDENT_TOKEN },
    [pub] => { $crate::SyntaxKind::PUB_TOKEN },
    [mut] => { $crate::SyntaxKind::MUT_TOKEN },
    [=] => { $crate::SyntaxKind::EQ_TOKEN },
    [:] => { $crate::SyntaxKind::COLON_TOKEN },
    [;] => { $crate::SyntaxKind::SEMICOLON_TOKEN },
    [@] => { $crate::SyntaxKind::AT_TOKEN },
    [abstract] => { $crate::SyntaxKind::ABSTRACT_TOKEN },
    [def] => { $crate::SyntaxKind::DEF_TOKEN },
    [const] => { $crate::SyntaxKind::CONST_TOKEN },
    [enum] => { $crate::SyntaxKind::ENUM_TOKEN },
    ['('] => { $crate::SyntaxKind::L_PAREN_TOKEN },
    [')'] => { $crate::SyntaxKind::R_PAREN_TOKEN },
    [,] => { $crate::SyntaxKind::COMMA_TOKEN },
    [import] => { $crate::SyntaxKind::IMPORT_TOKEN },
    [.] => { $crate::SyntaxKind::DOT_TOKEN },
    [struct] => { $crate::SyntaxKind::STRUCT_TOKEN },
    [type] => { $crate::SyntaxKind::TYPE_TOKEN },
    ['['] => { $crate::SyntaxKind::L_BRACKET_TOKEN },
    [']'] => { $crate::SyntaxKind::R_BRACKET_TOKEN },
    [&] => { $crate::SyntaxKind::AMP_TOKEN },
    [+] => { $crate::SyntaxKind::PLUS_TOKEN },
    [-] => { $crate::SyntaxKind::MINUS_TOKEN },
    [!] => { $crate::SyntaxKind::BANG_TOKEN },
    [~] => { $crate::SyntaxKind::TILDE_TOKEN },
    [*] => { $crate::SyntaxKind::STAR_TOKEN },
    [+=] => { $crate::SyntaxKind::PLUS_EQ_TOKEN },
    [-=] => { $crate::SyntaxKind::MINUS_EQ_TOKEN },
    [/=] => { $crate::SyntaxKind::SLASH_EQ_TOKEN },
    [*=] => { $crate::SyntaxKind::STAR_EQ_TOKEN },
    [**=] => { $crate::SyntaxKind::STAR2_EQ_TOKEN },
    [%=] => { $crate::SyntaxKind::PERCENT_EQ_TOKEN },
    [^=] => { $crate::SyntaxKind::CARET_EQ_TOKEN },
    [&=] => { $crate::SyntaxKind::AMP_EQ_TOKEN },
    [|=] => { $crate::SyntaxKind::PIPE_EQ_TOKEN },
    [<<=] => { $crate::SyntaxKind::LT2_EQ_TOKEN },
    [>>=] => { $crate::SyntaxKind::GT2_EQ_TOKEN },
    [/] => { $crate::SyntaxKind::SLASH_TOKEN },
    [**] => { $crate::SyntaxKind::STAR2_TOKEN },
    [%] => { $crate::SyntaxKind::PERCENT_TOKEN },
    [&&] => { $crate::SyntaxKind::AMP2_TOKEN },
    [||] => { $crate::SyntaxKind::PIPE2_TOKEN },
    [==] => { $crate::SyntaxKind::EQ2_TOKEN },
    [!=] => { $crate::SyntaxKind::BANG_EQ_TOKEN },
    [>] => { $crate::SyntaxKind::GT_TOKEN },
    [>=] => { $crate::SyntaxKind::GT_EQ_TOKEN },
    [<] => { $crate::SyntaxKind::LT_TOKEN },
    [<=] => { $crate::SyntaxKind::LT_EQ_TOKEN },
    [^] => { $crate::SyntaxKind::CARET_TOKEN },
    [|] => { $crate::SyntaxKind::PIPE_TOKEN },
    [<<] => { $crate::SyntaxKind::LT2_TOKEN },
    [>>] => { $crate::SyntaxKind::GT2_TOKEN },
    [..] => { $crate::SyntaxKind::DOT2_TOKEN },
    [@number] => { $crate::SyntaxKind::LIT_NUM_TOKEN },
    [@string] => { $crate::SyntaxKind::LIT_STR_TOKEN },
    [return] => { $crate::SyntaxKind::RETURN_TOKEN },
    [if] => { $crate::SyntaxKind::IF_TOKEN },
    [else] => { $crate::SyntaxKind::ELSE_TOKEN },
    [defer] => { $crate::SyntaxKind::DEFER_TOKEN },
    [@comment] => { $crate::SyntaxKind::COMMENT_TOKEN },
    [@whitespace] => { $crate::SyntaxKind::WHITESPACE_TOKEN },
    [@unknown] => { $crate::SyntaxKind::UNKNOWN_TOKEN
    },
}
/// Returns the [SyntaxKind] variant corresponding to the provided node
/// as written in the grammar.
///
/// # Example
///
/// ```no_run,
/// # use metal_ast::{N, AstNode, BinaryExprOpNode};
/// # fn example(binary_op_node: BinaryExprOpNode) {
/// assert!(binary_op_node.syntax().kind() == N![BinaryExprOp]);
/// # }
/// ```
pub macro N {
    [Root] => { $crate::SyntaxKind::ROOT_NODE },
    [BlockStmts] => { $crate::SyntaxKind::BLOCK_STMTS_NODE },
    [Block] => { $crate::SyntaxKind::BLOCK_NODE },
    [Stmt] => { $crate::SyntaxKind::STMT_NODE },
    [Name] => { $crate::SyntaxKind::NAME_NODE },
    [Visibility] => { $crate::SyntaxKind::VIS_NODE },
    [Mutability] => { $crate::SyntaxKind::MUTNESS_NODE },
    [ExprSpecifier] => { $crate::SyntaxKind::EXPR_SPEC_NODE },
    [Expr] => { $crate::SyntaxKind::EXPR_NODE },
    [TypeQualifier] => { $crate::SyntaxKind::TYPE_QUAL_NODE },
    [Type] => { $crate::SyntaxKind::TYPE_NODE },
    [TypeSpecifier] => { $crate::SyntaxKind::TYPE_SPECIFIER_NODE },
    [StmtKind] => { $crate::SyntaxKind::STMT_KIND_NODE },
    [Item] => { $crate::SyntaxKind::ITEM_NODE },
    [Annotations] => { $crate::SyntaxKind::ANNOTATIONS_NODE },
    [ItemKind] => { $crate::SyntaxKind::ITEM_KIND_NODE },
    [AbstractItem] => { $crate::SyntaxKind::ABSTRACT_ITEM_NODE },
    [ConstItem] => { $crate::SyntaxKind::CONST_ITEM_NODE },
    [EnumItem] => { $crate::SyntaxKind::ENUM_ITEM_NODE },
    [FnItem] => { $crate::SyntaxKind::FN_ITEM_NODE },
    [ImportItem] => { $crate::SyntaxKind::IMPORT_ITEM_NODE },
    [StructItem] => { $crate::SyntaxKind::STRUCT_ITEM_NODE },
    [TypeAliasItem] => { $crate::SyntaxKind::TYPE_ALIAS_ITEM_NODE },
    [Annotation] => { $crate::SyntaxKind::ANNOTATION_NODE },
    [GenericParamList] => { $crate::SyntaxKind::GENERIC_PARAM_LIST_NODE },
    [AbstractBody] => { $crate::SyntaxKind::ABSTRACT_BODY_NODE },
    [AbstractFnItem] => { $crate::SyntaxKind::ABSTRACT_FN_ITEM_NODE },
    [FnSignature] => { $crate::SyntaxKind::FN_SIGNATURE_NODE },
    [EnumBody] => { $crate::SyntaxKind::ENUM_BODY_NODE },
    [EnumBodyItem] => { $crate::SyntaxKind::ENUM_BODY_ITEM_NODE },
    [EnumVariant] => { $crate::SyntaxKind::ENUM_VARIANT_NODE },
    [EnumFn] => { $crate::SyntaxKind::ENUM_FN_NODE },
    [EnumVariantDataType] => { $crate::SyntaxKind::ENUM_VARIANT_DATA_TYPE_NODE },
    [FnInputs] => { $crate::SyntaxKind::FN_INPUTS_NODE },
    [FnInput] => { $crate::SyntaxKind::FN_INPUT_NODE },
    [ImportTree] => { $crate::SyntaxKind::IMPORT_TREE_NODE },
    [ImportLeaf] => { $crate::SyntaxKind::IMPORT_LEAF_NODE },
    [ImportBranch] => { $crate::SyntaxKind::IMPORT_BRANCH_NODE },
    [ImportSegment] => { $crate::SyntaxKind::IMPORT_SEGMENT_NODE },
    [ImportBranchSubtrees] => { $crate::SyntaxKind::IMPORT_BRANCH_SUBTREES_NODE },
    [StructBody] => { $crate::SyntaxKind::STRUCT_BODY_NODE },
    [StructBodyItem] => { $crate::SyntaxKind::STRUCT_BODY_ITEM_NODE },
    [StructField] => { $crate::SyntaxKind::STRUCT_FIELD_NODE },
    [StructFn] => { $crate::SyntaxKind::STRUCT_FN_NODE },
    [NameType] => { $crate::SyntaxKind::NAME_TYPE_NODE },
    [RefType] => { $crate::SyntaxKind::REF_TYPE_NODE },
    [BinaryType] => { $crate::SyntaxKind::BINARY_TYPE_NODE },
    [NameTypeGenerics] => { $crate::SyntaxKind::NAME_TYPE_GENERICS_NODE },
    [NameTypeGenericsInner] => { $crate::SyntaxKind::NAME_TYPE_GENERICS_INNER_NODE },
    [BinaryTypeOp] => { $crate::SyntaxKind::BINARY_TYPE_OP_NODE },
    [PrefixExpr] => { $crate::SyntaxKind::PREFIX_EXPR_NODE },
    [BinaryExpr] => { $crate::SyntaxKind::BINARY_EXPR_NODE },
    [CallExpr] => { $crate::SyntaxKind::CALL_EXPR_NODE },
    [LitExpr] => { $crate::SyntaxKind::LIT_EXPR_NODE },
    [ParenExpr] => { $crate::SyntaxKind::PAREN_EXPR_NODE },
    [ReturnExpr] => { $crate::SyntaxKind::RETURN_EXPR_NODE },
    [IfExpr] => { $crate::SyntaxKind::IF_EXPR_NODE },
    [DeferExpr] => { $crate::SyntaxKind::DEFER_EXPR_NODE },
    [StructExpr] => { $crate::SyntaxKind::STRUCT_EXPR_NODE },
    [PrefixExprOp] => { $crate::SyntaxKind::PREFIX_EXPR_OP_NODE },
    [BinaryExprOp] => { $crate::SyntaxKind::BINARY_EXPR_OP_NODE },
    [CallExprArgs] => { $crate::SyntaxKind::CALL_EXPR_ARGS_NODE },
    [IfExprElseClause] => { $crate::SyntaxKind::IF_EXPR_ELSE_CLAUSE_NODE },
    [StructExprFields] => { $crate::SyntaxKind::STRUCT_EXPR_FIELDS_NODE },
    [StructExprField] => { $crate::SyntaxKind::STRUCT_EXPR_FIELD_NODE },
    [GenericParams] => { $crate::SyntaxKind::GENERIC_PARAMS_NODE },
    [GenericParam] => { $crate::SyntaxKind::GENERIC_PARAM_NODE },
}
