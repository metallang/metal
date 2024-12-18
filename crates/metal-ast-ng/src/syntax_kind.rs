// SPDX-License-Identifier: MIT

//! This file is @generated by the build script, do not edit by hand.
#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SyntaxKind {
    /// Corresponds to [BlockNode].
    BLOCK_NODE,
    /// Corresponds to [BlockItemsNode].
    BLOCK_ITEMS_NODE,
    /// Corresponds to [ItemNode].
    ITEM_NODE,
    /// Corresponds to [NameNode].
    NAME_NODE,
    /// Corresponds to [VisNode].
    VIS_NODE,
    /// Corresponds to [MutnessNode].
    MUTNESS_NODE,
    /// Corresponds to [ExprSpecNode].
    EXPR_SPEC_NODE,
    /// Corresponds to [ExprNode].
    EXPR_NODE,
    /// Corresponds to [TypeQualNode].
    TYPE_QUAL_NODE,
    /// Corresponds to [TypeNode].
    TYPE_NODE,
    /// Corresponds to [AbstractItemNode].
    ABSTRACT_ITEM_NODE,
    /// Corresponds to [ConstItemNode].
    CONST_ITEM_NODE,
    /// Corresponds to [EnumItemNode].
    ENUM_ITEM_NODE,
    /// Corresponds to [FnItemNode].
    FN_ITEM_NODE,
    /// Corresponds to [ImportItemNode].
    IMPORT_ITEM_NODE,
    /// Corresponds to [ReturnItemNode].
    RETURN_ITEM_NODE,
    /// Corresponds to [StructItemNode].
    STRUCT_ITEM_NODE,
    /// Corresponds to [TypeAliasItemNode].
    TYPE_ALIAS_ITEM_NODE,
    /// Corresponds to [AbstractBodyNode].
    ABSTRACT_BODY_NODE,
    /// Corresponds to [AbstractFnItemNode].
    ABSTRACT_FN_ITEM_NODE,
    /// Corresponds to [FnSignatureNode].
    FN_SIGNATURE_NODE,
    /// Corresponds to [EnumBodyNode].
    ENUM_BODY_NODE,
    /// Corresponds to [EnumBodyItemNode].
    ENUM_BODY_ITEM_NODE,
    /// Corresponds to [EnumVariantNode].
    ENUM_VARIANT_NODE,
    /// Corresponds to [EnumVariantDataTypeNode].
    ENUM_VARIANT_DATA_TYPE_NODE,
    /// Corresponds to [FnInputsNode].
    FN_INPUTS_NODE,
    /// Corresponds to [FnInputNode].
    FN_INPUT_NODE,
    /// Corresponds to [ImportTreeNode].
    IMPORT_TREE_NODE,
    /// Corresponds to [ImportLeafNode].
    IMPORT_LEAF_NODE,
    /// Corresponds to [ImportSegmentNode].
    IMPORT_SEGMENT_NODE,
    /// Corresponds to [ImportBranchNode].
    IMPORT_BRANCH_NODE,
    /// Corresponds to [ImportBranchSubtreesNode].
    IMPORT_BRANCH_SUBTREES_NODE,
    /// Corresponds to [StructBodyNode].
    STRUCT_BODY_NODE,
    /// Corresponds to [StructBodyItemNode].
    STRUCT_BODY_ITEM_NODE,
    /// Corresponds to [StructFieldNode].
    STRUCT_FIELD_NODE,
    /// Corresponds to [NameTypeNode].
    NAME_TYPE_NODE,
    /// Corresponds to [RefTypeNode].
    REF_TYPE_NODE,
    /// Corresponds to [BinaryTypeNode].
    BINARY_TYPE_NODE,
    /// Corresponds to [NameTypeMaybeGenericsNode].
    NAME_TYPE_MAYBE_GENERICS_NODE,
    /// Corresponds to [NameTypeGenericsNode].
    NAME_TYPE_GENERICS_NODE,
    /// Corresponds to [BinaryTypeOpNode].
    BINARY_TYPE_OP_NODE,
    /// Corresponds to [PrefixExprNode].
    PREFIX_EXPR_NODE,
    /// Corresponds to [BinaryExprNode].
    BINARY_EXPR_NODE,
    /// Corresponds to [CallExprNode].
    CALL_EXPR_NODE,
    /// Corresponds to [LitExprNode].
    LIT_EXPR_NODE,
    /// Corresponds to [PrefixExprOpNode].
    PREFIX_EXPR_OP_NODE,
    /// Corresponds to [BinaryExprOpNode].
    BINARY_EXPR_OP_NODE,
    /// Corresponds to [CallExprArgsNode].
    CALL_EXPR_ARGS_NODE,
    /// Corresponds to [NumLitNode].
    NUM_LIT_NODE,
    /// Corresponds to [StrLitNode].
    STR_LIT_NODE,
    /// Don't try to remember this! Use [`T![{]`](T) instead.
    L_BRACE_TOKEN,
    /// Don't try to remember this! Use [`T![}]`](T) instead.
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
    /// Don't try to remember this! Use [`T![abstract]`](T) instead.
    ABSTRACT_TOKEN,
    /// Don't try to remember this! Use [`T![def]`](T) instead.
    DEF_TOKEN,
    /// Don't try to remember this! Use [`T![;]`](T) instead.
    SEMICOLON_TOKEN,
    /// Don't try to remember this! Use [`T![const]`](T) instead.
    CONST_TOKEN,
    /// Don't try to remember this! Use [`T![enum]`](T) instead.
    ENUM_TOKEN,
    /// Don't try to remember this! Use [`T![(]`](T) instead.
    L_PAREN_TOKEN,
    /// Don't try to remember this! Use [`T![)]`](T) instead.
    R_PAREN_TOKEN,
    /// Don't try to remember this! Use [`T![,]`](T) instead.
    COMMA_TOKEN,
    /// Don't try to remember this! Use [`T![import]`](T) instead.
    IMPORT_TOKEN,
    /// Don't try to remember this! Use [`T![.]`](T) instead.
    DOT_TOKEN,
    /// Don't try to remember this! Use [`T![return]`](T) instead.
    RETURN_TOKEN,
    /// Don't try to remember this! Use [`T![struct]`](T) instead.
    STRUCT_TOKEN,
    /// Don't try to remember this! Use [`T![type]`](T) instead.
    TYPE_TOKEN,
    /// Don't try to remember this! Use [`T![[]`](T) instead.
    L_BRACKET_TOKEN,
    /// Don't try to remember this! Use [`T![]]`](T) instead.
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
    SHIFTL_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![>>=]`](T) instead.
    SHIFTR_EQ_TOKEN,
    /// Don't try to remember this! Use [`T![/]`](T) instead.
    SLASH_TOKEN,
    /// Don't try to remember this! Use [`T![*]`](T) instead.
    STAR_TOKEN,
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
    /// A special syntax kind used for transmute safety checks. You shouldn't worry
    /// about (and even less rely on) this.
    __LAST,
}
/// Returns the [SyntaxKind] variants corresponding to the provided token
/// as written in the grammar.
///
/// Note that certain tokens such as parentheses, braces, and brackets need
/// to be wrapped in single quotes (like you would spell a character), e.g.
/// `T!['{']`. This is a limitation imposed by Rust.
///
/// # Example
///
/// ```no_run,
/// # use metal_ast_ng::{T, AstToken, BinaryExprOpNode};
/// # fn example(binary_op_node: BinaryExprOpNode) {
/// if binary_op_node.token().is_some_and(|token| token.syntax().kind() == T![+]) {
///     // The binary operator is plus!
/// }
/// # }
/// ```
pub macro T {
    ['{'] => { $crate ::SyntaxKind::L_BRACE_TOKEN }, ['}'] => { $crate
    ::SyntaxKind::R_BRACE_TOKEN }, [@ ident] => { $crate ::SyntaxKind::LIT_IDENT_TOKEN },
    [pub] => { $crate ::SyntaxKind::PUB_TOKEN }, [mut] => { $crate
    ::SyntaxKind::MUT_TOKEN }, [=] => { $crate ::SyntaxKind::EQ_TOKEN }, [:] => { $crate
    ::SyntaxKind::COLON_TOKEN }, [abstract] => { $crate ::SyntaxKind::ABSTRACT_TOKEN },
    [def] => { $crate ::SyntaxKind::DEF_TOKEN }, [;] => { $crate
    ::SyntaxKind::SEMICOLON_TOKEN }, [const] => { $crate ::SyntaxKind::CONST_TOKEN },
    [enum] => { $crate ::SyntaxKind::ENUM_TOKEN }, ['('] => { $crate
    ::SyntaxKind::L_PAREN_TOKEN }, [')'] => { $crate ::SyntaxKind::R_PAREN_TOKEN }, [,]
    => { $crate ::SyntaxKind::COMMA_TOKEN }, [import] => { $crate
    ::SyntaxKind::IMPORT_TOKEN }, [.] => { $crate ::SyntaxKind::DOT_TOKEN }, [return] =>
    { $crate ::SyntaxKind::RETURN_TOKEN }, [struct] => { $crate
    ::SyntaxKind::STRUCT_TOKEN }, [type] => { $crate ::SyntaxKind::TYPE_TOKEN }, ['['] =>
    { $crate ::SyntaxKind::L_BRACKET_TOKEN }, [']'] => { $crate
    ::SyntaxKind::R_BRACKET_TOKEN }, [&] => { $crate ::SyntaxKind::AMP_TOKEN }, [+] => {
    $crate ::SyntaxKind::PLUS_TOKEN }, [-] => { $crate ::SyntaxKind::MINUS_TOKEN }, [!]
    => { $crate ::SyntaxKind::BANG_TOKEN }, [~] => { $crate ::SyntaxKind::TILDE_TOKEN },
    [+=] => { $crate ::SyntaxKind::PLUS_EQ_TOKEN }, [-=] => { $crate
    ::SyntaxKind::MINUS_EQ_TOKEN }, [/=] => { $crate ::SyntaxKind::SLASH_EQ_TOKEN }, [*=]
    => { $crate ::SyntaxKind::STAR_EQ_TOKEN }, [**=] => { $crate
    ::SyntaxKind::STAR2_EQ_TOKEN }, [%=] => { $crate ::SyntaxKind::PERCENT_EQ_TOKEN },
    [^=] => { $crate ::SyntaxKind::CARET_EQ_TOKEN }, [&=] => { $crate
    ::SyntaxKind::AMP_EQ_TOKEN }, [|=] => { $crate ::SyntaxKind::PIPE_EQ_TOKEN }, [<<=]
    => { $crate ::SyntaxKind::SHIFTL_EQ_TOKEN }, [>>=] => { $crate
    ::SyntaxKind::SHIFTR_EQ_TOKEN }, [/] => { $crate ::SyntaxKind::SLASH_TOKEN }, [*] =>
    { $crate ::SyntaxKind::STAR_TOKEN }, [**] => { $crate ::SyntaxKind::STAR2_TOKEN },
    [%] => { $crate ::SyntaxKind::PERCENT_TOKEN }, [&&] => { $crate
    ::SyntaxKind::AMP2_TOKEN }, [||] => { $crate ::SyntaxKind::PIPE2_TOKEN }, [==] => {
    $crate ::SyntaxKind::EQ2_TOKEN }, [!=] => { $crate ::SyntaxKind::BANG_EQ_TOKEN }, [>]
    => { $crate ::SyntaxKind::GT_TOKEN }, [>=] => { $crate ::SyntaxKind::GT_EQ_TOKEN },
    [<] => { $crate ::SyntaxKind::LT_TOKEN }, [<=] => { $crate ::SyntaxKind::LT_EQ_TOKEN
    }, [^] => { $crate ::SyntaxKind::CARET_TOKEN }, [|] => { $crate
    ::SyntaxKind::PIPE_TOKEN }, [<<] => { $crate ::SyntaxKind::LT2_TOKEN }, [>>] => {
    $crate ::SyntaxKind::GT2_TOKEN }, [..] => { $crate ::SyntaxKind::DOT2_TOKEN }, [@
    number] => { $crate ::SyntaxKind::LIT_NUM_TOKEN }, [@ string] => { $crate
    ::SyntaxKind::LIT_STR_TOKEN },
}