// Rule 0: `Rule::Labeled { label, rule }` is to be handled in the same way as it's `rule`
// Rule 1: `Rule::Opt(rule)` is to be handled in the same way as it's `rule` if that `rule` is either `Rule::Token` or a `Rule::Node`, except that the (return) type of such a rule is marked as `Option` (if not already)

// Step 0. Starting point.
Name           = Rule::Token("@ident")
Visibility     = Rule::Opt(Rule::Token("pub"))
Mutability     = Rule::Opt(Rule::Token("mut"))
ExprSpecifier  = Rule::Seq([Rule::Token("="), Rule::Node("Expr")])
Expr           = Rule::Alt([Rule::Node("Name"), Rule::Node("PrefixExpr"), Rule::Node("BinaryExpr"), Rule::Node("CallExpr"), Rule::Node("LitExpr")])
TypeQualifier  = Rule::Seq([Rule::Token(":"), Rule::Node("Type")])
Type           = Rule::Node("Name")
Block          = Rule::Seq([Rule::Token("{"), Rule::Node("Item"), Rule::Token("}")])
Item           = Rule::Alt([Rule::Node("ConstItem"), Rule::Node("EnumItem"), Rule::Node("Expr"), Rule::Node("FnItem"), Rule::Node("ImportItem"), Rule::Node("StructItem")])
ConstItem      = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("const"), Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }, Rule::Labeled { label: "value", rule: Rule::Node("ExprSpecifier") }])
EnumItem        = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("enum"), Rule::Node("Name"), Rule::Token("{"), Rule::Labeled { label: "body", rule: Rule::Opt(Rule::Node("EnumBody")) }, Rule::Token("}")])
FnItem         = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("def"), Rule::Node("Name"), Rule::Labeled { label: "sig", rule: Rule::Node("FnSignature") }, Rule::Labeled { label: "body", rule: Rule::Node("Block") }])
ImportItem     = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("import"), Rule::Labeled { label: "tree", rule: Rule::Node("ImportTree") }, Rule::Token(";")])
StructItem     = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("struct"), Rule::Node("Name"), Rule::Token("{"), Rule::Labeled { label: "body", rule: Rule::Node("StructBody") }, Rule::Token("}")])
EnumBody        = Rule::Seq([Rule::Node("EnumBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("EnumBodyItem")])), Rule::Opt(Rule::Token(","))])
EnumBodyItem   = Rule::Alt([Rule::Node("EnumVariant"), Rule::Node("FnItem")])
EnumVariant     = Rule::Seq([Rule::Node("Name"), Rule::Labeled { label: "data_ty", rule: Rule::Opt(Rule::Node("EnumVariantType")) }])
EnumVariantType = Rule::Seq([Rule::Token("("), Rule::Node("Type"), Rule::Token(")")])
FnSignature     = Rule::Seq([Rule::Token("("), Rule::Labeled { label: "inputs", rule: Rule::Opt(Rule::Seq([Rule::Node("FnInput"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("FnInput")])), Rule::Opt(Rule::Token(","))])) }, Rule::Token(")"), Rule::Labeled { label: "return_ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }])
FnInput        = Rule::Seq([Rule::Labeled { label: "mutness", rule: Rule::Node("Mutability") }, Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }, Rule::Labeled { label: "default", rule: Rule::Opt(Rule::Node("ExprSpecifier")) }])
ImportTree     = Rule::Alt([Rule::Node("ImportLeaf"), Rule::Node("ImportSegment"), Rule::Node("ImportBranch")])
ImportLeaf     = Rule::Node("Name")
ImportSegment  = Rule::Seq([Rule::Labeled { label: "segment", rule: Rule::Node("Name") }, Rule::Token("."), Rule::Labeled { label: "rest", rule: Rule::Node("ImportTree") }])
ImportBranch    = Rule::Seq([Rule::Token("{"), Rule::Labeled { label: "subtrees", rule: Rule::Opt(Rule::Seq([Rule::Node("ImportTree"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("ImportTree")])), Rule::Opt(Rule::Token(","))])) }, Rule::Token("}")])
StructBody      = Rule::Seq([Rule::Node("StructBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("StructBodyItem")])), Rule::Opt(Rule::Token(","))])
StructBodyItem = Rule::Alt([Rule::Node("StructField"), Rule::Node("FnItem")])
StructField    = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Node("TypeQualifier") }])
PrefixExpr     = Rule::Seq([Rule::Labeled { label: "op", rule: Rule::Node("PrefixOp") }, Rule::Node("Expr")])
BinaryExpr      = Rule::Seq([Rule::Labeled { label: "lhs", rule: Rule::Node("Expr") }, Rule::Labeled { label: "op", rule: Rule::Node("BinaryOp") }, Rule::Labeled { label: "rhs", rule: Rule::Node("Expr") }])
CallExpr        = Rule::Seq([Rule::Node("Expr"), Rule::Token("("), Rule::Rep(Rule::Seq([Rule::Node("Expr"), Rule::Opt(Rule::Token(","))])), Rule::Token(")")])
LitExpr        = Rule::Alt([Rule::Node("NumLit"), Rule::Node("StrLit")])
PrefixOp       = Rule::Alt([Rule::Token("+"), Rule::Token("-"), Rule::Token("!"), Rule::Token("~")])
BinaryOp       = Rule::Alt([Rule::Token("="), Rule::Token("+="), Rule::Token("-="), Rule::Token("/="), Rule::Token("*="), Rule::Token("**="), Rule::Token("%="), Rule::Token("^="), Rule::Token("&="), Rule::Token("|="), Rule::Token("<<="), Rule::Token(">>="), Rule::Token("+"), Rule::Token("-"), Rule::Token("/"), Rule::Token("*"), Rule::Token("**"), Rule::Token("%"), Rule::Token("&&"), Rule::Token("||"), Rule::Token("=="), Rule::Token("!="), Rule::Token(">"), Rule::Token(">="), Rule::Token("<"), Rule::Token("<="), Rule::Token("^"), Rule::Token("&"), Rule::Token("|"), Rule::Token("<<"), Rule::Token(">>"), Rule::Seq([Rule::Token(".."), Rule::Token(".")])])
NumLit         = Rule::Token("@number")
StrLit         = Rule::Token("@string")

// Step 1. Codegen top-level Rule::Tokens
Name           = 
Visibility     = 
Mutability     = 
ExprSpecifier  = Rule::Seq([Rule::Token("="), Rule::Node("Expr")])
Expr           = Rule::Alt([Rule::Node("Name"), Rule::Node("PrefixExpr"), Rule::Node("BinaryExpr"), Rule::Node("CallExpr"), Rule::Node("LitExpr")])
TypeQualifier  = Rule::Seq([Rule::Token(":"), Rule::Node("Type")])
Type           = Rule::Node("Name")
Block          = Rule::Seq([Rule::Token("{"), Rule::Node("Item"), Rule::Token("}")])
Item           = Rule::Alt([Rule::Node("ConstItem"), Rule::Node("EnumItem"), Rule::Node("Expr"), Rule::Node("FnItem"), Rule::Node("ImportItem"), Rule::Node("StructItem")])
ConstItem      = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("const"), Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }, Rule::Labeled { label: "value", rule: Rule::Node("ExprSpecifier") }])
EnumItem        = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("enum"), Rule::Node("Name"), Rule::Token("{"), Rule::Labeled { label: "body", rule: Rule::Opt(Rule::Node("EnumBody")) }, Rule::Token("}")])
FnItem         = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("def"), Rule::Node("Name"), Rule::Labeled { label: "sig", rule: Rule::Node("FnSignature") }, Rule::Labeled { label: "body", rule: Rule::Node("Block") }])
ImportItem     = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("import"), Rule::Labeled { label: "tree", rule: Rule::Node("ImportTree") }, Rule::Token(";")])
StructItem     = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("struct"), Rule::Node("Name"), Rule::Token("{"), Rule::Labeled { label: "body", rule: Rule::Node("StructBody") }, Rule::Token("}")])
EnumBody        = Rule::Seq([Rule::Node("EnumBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("EnumBodyItem")])), Rule::Opt(Rule::Token(","))])
EnumBodyItem   = Rule::Alt([Rule::Node("EnumVariant"), Rule::Node("FnItem")])
EnumVariant     = Rule::Seq([Rule::Node("Name"), Rule::Labeled { label: "data_ty", rule: Rule::Opt(Rule::Node("EnumVariantType")) }])
EnumVariantType = Rule::Seq([Rule::Token("("), Rule::Node("Type"), Rule::Token(")")])
FnSignature     = Rule::Seq([Rule::Token("("), Rule::Labeled { label: "inputs", rule: Rule::Opt(Rule::Seq([Rule::Node("FnInput"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("FnInput")])), Rule::Opt(Rule::Token(","))])) }, Rule::Token(")"), Rule::Labeled { label: "return_ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }])
FnInput        = Rule::Seq([Rule::Labeled { label: "mutness", rule: Rule::Node("Mutability") }, Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }, Rule::Labeled { label: "default", rule: Rule::Opt(Rule::Node("ExprSpecifier")) }])
ImportTree     = Rule::Alt([Rule::Node("ImportLeaf"), Rule::Node("ImportSegment"), Rule::Node("ImportBranch")])
ImportLeaf     = Rule::Node("Name")
ImportSegment  = Rule::Seq([Rule::Labeled { label: "segment", rule: Rule::Node("Name") }, Rule::Token("."), Rule::Labeled { label: "rest", rule: Rule::Node("ImportTree") }])
ImportBranch    = Rule::Seq([Rule::Token("{"), Rule::Labeled { label: "subtrees", rule: Rule::Opt(Rule::Seq([Rule::Node("ImportTree"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("ImportTree")])), Rule::Opt(Rule::Token(","))])) }, Rule::Token("}")])
StructBody      = Rule::Seq([Rule::Node("StructBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("StructBodyItem")])), Rule::Opt(Rule::Token(","))])
StructBodyItem = Rule::Alt([Rule::Node("StructField"), Rule::Node("FnItem")])
StructField    = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Node("TypeQualifier") }])
PrefixExpr     = Rule::Seq([Rule::Labeled { label: "op", rule: Rule::Node("PrefixOp") }, Rule::Node("Expr")])
BinaryExpr      = Rule::Seq([Rule::Labeled { label: "lhs", rule: Rule::Node("Expr") }, Rule::Labeled { label: "op", rule: Rule::Node("BinaryOp") }, Rule::Labeled { label: "rhs", rule: Rule::Node("Expr") }])
CallExpr        = Rule::Seq([Rule::Node("Expr"), Rule::Token("("), Rule::Rep(Rule::Seq([Rule::Node("Expr"), Rule::Opt(Rule::Token(","))])), Rule::Token(")")])
LitExpr        = Rule::Alt([Rule::Node("NumLit"), Rule::Node("StrLit")])
PrefixOp       = Rule::Alt([Rule::Token("+"), Rule::Token("-"), Rule::Token("!"), Rule::Token("~")])
BinaryOp       = Rule::Alt([Rule::Token("="), Rule::Token("+="), Rule::Token("-="), Rule::Token("/="), Rule::Token("*="), Rule::Token("**="), Rule::Token("%="), Rule::Token("^="), Rule::Token("&="), Rule::Token("|="), Rule::Token("<<="), Rule::Token(">>="), Rule::Token("+"), Rule::Token("-"), Rule::Token("/"), Rule::Token("*"), Rule::Token("**"), Rule::Token("%"), Rule::Token("&&"), Rule::Token("||"), Rule::Token("=="), Rule::Token("!="), Rule::Token(">"), Rule::Token(">="), Rule::Token("<"), Rule::Token("<="), Rule::Token("^"), Rule::Token("&"), Rule::Token("|"), Rule::Token("<<"), Rule::Token(">>"), Rule::Seq([Rule::Token(".."), Rule::Token(".")])])
NumLit         = 
StrLit         = 

// Step 2. Codegen top-level Rule::Nodes
Name           = 
Visibility     = 
Mutability     = 
ExprSpecifier  = Rule::Seq([Rule::Token("="), Rule::Node("Expr")])
Expr           = Rule::Alt([Rule::Node("Name"), Rule::Node("PrefixExpr"), Rule::Node("BinaryExpr"), Rule::Node("CallExpr"), Rule::Node("LitExpr")])
TypeQualifier  = Rule::Seq([Rule::Token(":"), Rule::Node("Type")])
Type           = 
Block          = Rule::Seq([Rule::Token("{"), Rule::Node("Item"), Rule::Token("}")])
Item           = Rule::Alt([Rule::Node("ConstItem"), Rule::Node("EnumItem"), Rule::Node("Expr"), Rule::Node("FnItem"), Rule::Node("ImportItem"), Rule::Node("StructItem")])
ConstItem      = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("const"), Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }, Rule::Labeled { label: "value", rule: Rule::Node("ExprSpecifier") }])
EnumItem        = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("enum"), Rule::Node("Name"), Rule::Token("{"), Rule::Labeled { label: "body", rule: Rule::Opt(Rule::Node("EnumBody")) }, Rule::Token("}")])
FnItem         = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("def"), Rule::Node("Name"), Rule::Labeled { label: "sig", rule: Rule::Node("FnSignature") }, Rule::Labeled { label: "body", rule: Rule::Node("Block") }])
ImportItem     = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("import"), Rule::Labeled { label: "tree", rule: Rule::Node("ImportTree") }, Rule::Token(";")])
StructItem     = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Token("struct"), Rule::Node("Name"), Rule::Token("{"), Rule::Labeled { label: "body", rule: Rule::Node("StructBody") }, Rule::Token("}")])
EnumBody        = Rule::Seq([Rule::Node("EnumBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("EnumBodyItem")])), Rule::Opt(Rule::Token(","))])
EnumBodyItem   = Rule::Alt([Rule::Node("EnumVariant"), Rule::Node("FnItem")])
EnumVariant     = Rule::Seq([Rule::Node("Name"), Rule::Labeled { label: "data_ty", rule: Rule::Opt(Rule::Node("EnumVariantType")) }])
EnumVariantType = Rule::Seq([Rule::Token("("), Rule::Node("Type"), Rule::Token(")")])
FnSignature     = Rule::Seq([Rule::Token("("), Rule::Labeled { label: "inputs", rule: Rule::Opt(Rule::Seq([Rule::Node("FnInput"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("FnInput")])), Rule::Opt(Rule::Token(","))])) }, Rule::Token(")"), Rule::Labeled { label: "return_ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }])
FnInput        = Rule::Seq([Rule::Labeled { label: "mutness", rule: Rule::Node("Mutability") }, Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }, Rule::Labeled { label: "default", rule: Rule::Opt(Rule::Node("ExprSpecifier")) }])
ImportTree     = Rule::Alt([Rule::Node("ImportLeaf"), Rule::Node("ImportSegment"), Rule::Node("ImportBranch")])
ImportLeaf     = 
ImportSegment  = Rule::Seq([Rule::Labeled { label: "segment", rule: Rule::Node("Name") }, Rule::Token("."), Rule::Labeled { label: "rest", rule: Rule::Node("ImportTree") }])
ImportBranch    = Rule::Seq([Rule::Token("{"), Rule::Labeled { label: "subtrees", rule: Rule::Opt(Rule::Seq([Rule::Node("ImportTree"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("ImportTree")])), Rule::Opt(Rule::Token(","))])) }, Rule::Token("}")])
StructBody      = Rule::Seq([Rule::Node("StructBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("StructBodyItem")])), Rule::Opt(Rule::Token(","))])
StructBodyItem = Rule::Alt([Rule::Node("StructField"), Rule::Node("FnItem")])
StructField    = Rule::Seq([Rule::Labeled { label: "vis", rule: Rule::Node("Visibility") }, Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Node("TypeQualifier") }])
PrefixExpr     = Rule::Seq([Rule::Labeled { label: "op", rule: Rule::Node("PrefixOp") }, Rule::Node("Expr")])
BinaryExpr      = Rule::Seq([Rule::Labeled { label: "lhs", rule: Rule::Node("Expr") }, Rule::Labeled { label: "op", rule: Rule::Node("BinaryOp") }, Rule::Labeled { label: "rhs", rule: Rule::Node("Expr") }])
CallExpr        = Rule::Seq([Rule::Node("Expr"), Rule::Token("("), Rule::Rep(Rule::Seq([Rule::Node("Expr"), Rule::Opt(Rule::Token(","))])), Rule::Token(")")])
LitExpr        = Rule::Alt([Rule::Node("NumLit"), Rule::Node("StrLit")])
PrefixOp       = Rule::Alt([Rule::Token("+"), Rule::Token("-"), Rule::Token("!"), Rule::Token("~")])
BinaryOp       = Rule::Alt([Rule::Token("="), Rule::Token("+="), Rule::Token("-="), Rule::Token("/="), Rule::Token("*="), Rule::Token("**="), Rule::Token("%="), Rule::Token("^="), Rule::Token("&="), Rule::Token("|="), Rule::Token("<<="), Rule::Token(">>="), Rule::Token("+"), Rule::Token("-"), Rule::Token("/"), Rule::Token("*"), Rule::Token("**"), Rule::Token("%"), Rule::Token("&&"), Rule::Token("||"), Rule::Token("=="), Rule::Token("!="), Rule::Token(">"), Rule::Token(">="), Rule::Token("<"), Rule::Token("<="), Rule::Token("^"), Rule::Token("&"), Rule::Token("|"), Rule::Token("<<"), Rule::Token(">>"), Rule::Seq([Rule::Token(".."), Rule::Token(".")])])
NumLit         = 
StrLit         = 

// Step 3. Apply transformations described in steps 1-2 to all top-level Rule::Seqs that only contain Rule::Nodes and Rule::Tokens (rules 0 and 1 apply)
Name           = 
Visibility     = 
Mutability     = 
ExprSpecifier  = 
Expr           = Rule::Alt([Rule::Node("Name"), Rule::Node("PrefixExpr"), Rule::Node("BinaryExpr"), Rule::Node("CallExpr"), Rule::Node("LitExpr")])
TypeQualifier  = 
Type           = 
Block          = 
Item           = Rule::Alt([Rule::Node("ConstItem"), Rule::Node("EnumItem"), Rule::Node("Expr"), Rule::Node("FnItem"), Rule::Node("ImportItem"), Rule::Node("StructItem")])
ConstItem      = 
EnumItem       = 
FnItem         = 
ImportItem     = 
StructItem     = 
EnumBody        = Rule::Seq([Rule::Node("EnumBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("EnumBodyItem")])), Rule::Opt(Rule::Token(","))])
EnumBodyItem   = Rule::Alt([Rule::Node("EnumVariant"), Rule::Node("FnItem")])
EnumVariant     = 
EnumVariantType = 
FnSignature     = Rule::Seq([Rule::Token("("), Rule::Labeled { label: "inputs", rule: Rule::Opt(Rule::Seq([Rule::Node("FnInput"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("FnInput")])), Rule::Opt(Rule::Token(","))])) }, Rule::Token(")"), Rule::Labeled { label: "return_ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }])
FnInput        = Rule::Seq([Rule::Labeled { label: "mutness", rule: Rule::Node("Mutability") }, Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }, Rule::Labeled { label: "default", rule: Rule::Opt(Rule::Node("ExprSpecifier")) }])
ImportTree     = Rule::Alt([Rule::Node("ImportLeaf"), Rule::Node("ImportSegment"), Rule::Node("ImportBranch")])
ImportLeaf     = 
ImportSegment  = 
ImportBranch    = Rule::Seq([Rule::Token("{"), Rule::Labeled { label: "subtrees", rule: Rule::Opt(Rule::Seq([Rule::Node("ImportTree"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("ImportTree")])), Rule::Opt(Rule::Token(","))])) }, Rule::Token("}")])
StructBody      = Rule::Seq([Rule::Node("StructBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("StructBodyItem")])), Rule::Opt(Rule::Token(","))])
StructBodyItem = Rule::Alt([Rule::Node("StructField"), Rule::Node("FnItem")])
StructField    = 
PrefixExpr     = 
BinaryExpr      = 
CallExpr        = Rule::Seq([Rule::Node("Expr"), Rule::Token("("), Rule::Rep(Rule::Seq([Rule::Node("Expr"), Rule::Opt(Rule::Token(","))])), Rule::Token(")")])
LitExpr        = Rule::Alt([Rule::Node("NumLit"), Rule::Node("StrLit")])
PrefixOp       = Rule::Alt([Rule::Token("+"), Rule::Token("-"), Rule::Token("!"), Rule::Token("~")])
BinaryOp       = Rule::Alt([Rule::Token("="), Rule::Token("+="), Rule::Token("-="), Rule::Token("/="), Rule::Token("*="), Rule::Token("**="), Rule::Token("%="), Rule::Token("^="), Rule::Token("&="), Rule::Token("|="), Rule::Token("<<="), Rule::Token(">>="), Rule::Token("+"), Rule::Token("-"), Rule::Token("/"), Rule::Token("*"), Rule::Token("**"), Rule::Token("%"), Rule::Token("&&"), Rule::Token("||"), Rule::Token("=="), Rule::Token("!="), Rule::Token(">"), Rule::Token(">="), Rule::Token("<"), Rule::Token("<="), Rule::Token("^"), Rule::Token("&"), Rule::Token("|"), Rule::Token("<<"), Rule::Token(">>"), Rule::Seq([Rule::Token(".."), Rule::Token(".")])])
NumLit         = 
StrLit         = 

// Step 4. Codegen all Rule::Alts ~~Cunninghams~~ that only contain Rule::Nodes and Rule::Tokens (rules 0 and 1 apply)
Name           = 
Visibility     = 
Mutability     = 
ExprSpecifier  = 
Expr           = 
TypeQualifier  = 
Type           = 
Block          = 
Item           = 
ConstItem      = 
EnumItem       = 
FnItem         = 
ImportItem     = 
StructItem     = 
EnumBody        = Rule::Seq([Rule::Node("EnumBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("EnumBodyItem")])), Rule::Opt(Rule::Token(","))])
EnumBodyItem   = 
EnumVariant     = 
EnumVariantType = 
FnSignature     = Rule::Seq([Rule::Token("("), Rule::Labeled { label: "inputs", rule: Rule::Opt(Rule::Seq([Rule::Node("FnInput"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("FnInput")])), Rule::Opt(Rule::Token(","))])) }, Rule::Token(")"), Rule::Labeled { label: "return_ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }])
FnInput        = Rule::Seq([Rule::Labeled { label: "mutness", rule: Rule::Node("Mutability") }, Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }, Rule::Labeled { label: "default", rule: Rule::Opt(Rule::Node("ExprSpecifier")) }])
ImportTree     = 
ImportLeaf     = 
ImportSegment  = 
ImportBranch    = Rule::Seq([Rule::Token("{"), Rule::Labeled { label: "subtrees", rule: Rule::Opt(Rule::Seq([Rule::Node("ImportTree"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("ImportTree")])), Rule::Opt(Rule::Token(","))])) }, Rule::Token("}")])
StructBody      = Rule::Seq([Rule::Node("StructBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("StructBodyItem")])), Rule::Opt(Rule::Token(","))])
StructBodyItem = 
StructField    = 
PrefixExpr     = 
BinaryExpr      = 
CallExpr        = Rule::Seq([Rule::Node("Expr"), Rule::Token("("), Rule::Rep(Rule::Seq([Rule::Node("Expr"), Rule::Opt(Rule::Token(","))])), Rule::Token(")")])
LitExpr        = 
PrefixOp       = 
BinaryOp       = 
NumLit         = 
StrLit         = 

// Step 5. Recursively look for Rule::Seqs that:
// - contain exactly 3 subrules
// - the first subrule is a Rule::Node (#1)
// - the second subrule is a Rule::Rep(Rule::Seq) that:
//   - contains exactly 2 subrules
//   - the first subrule is a Rule::Token (#2)
//   - the second subrule is a Rule::Node and is equal to the #1 rule
// - the third subrule is a Rule::Opt(Rule::Token), where Rule::Token is equal to #2
// For every such Rule::Seq, codegen it as a vec of #1 nodes, even if it's wrapped in a Rule::Opt

Name           = 
Visibility     = 
Mutability     = 
ExprSpecifier  = 
Expr           = 
TypeQualifier  = 
Type           = 
Block          = 
Item           = 
ConstItem      = 
EnumItem       = 
FnItem         = 
ImportItem     = 
StructItem     = 
EnumBody        = Rule::Seq([Rule::Node("EnumBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("EnumBodyItem")])), Rule::Opt(Rule::Token(","))])
EnumBodyItem   = 
EnumVariant     = 
EnumVariantType = 
FnSignature     = Rule::Seq([Rule::Token("("), Rule::Labeled { label: "inputs", rule: Rule::Opt(Rule::Seq([Rule::Node("FnInput"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("FnInput")])), Rule::Opt(Rule::Token(","))])) }, Rule::Token(")"), Rule::Labeled { label: "return_ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }])
FnInput        = Rule::Seq([Rule::Labeled { label: "mutness", rule: Rule::Node("Mutability") }, Rule::Node("Name"), Rule::Labeled { label: "ty", rule: Rule::Opt(Rule::Node("TypeQualifier")) }, Rule::Labeled { label: "default", rule: Rule::Opt(Rule::Node("ExprSpecifier")) }])
ImportTree     = 
ImportLeaf     = 
ImportSegment  = 
ImportBranch    = Rule::Seq([
    Rule::Token("{"),
    Rule::Labeled {
        label: "subtrees",
        rule: Rule::Opt(
            Rule::Seq([
                Rule::Node("ImportTree"),
                Rule::Rep(Rule::Seq([
                    Rule::Token(","),
                    Rule::Node("ImportTree")
                    ])),
                Rule::Opt(Rule::Token(","))
            ])
        )
    },
    Rule::Token("}")
    ])
StructBody      = Rule::Seq([Rule::Node("StructBodyItem"), Rule::Rep(Rule::Seq([Rule::Token(","), Rule::Node("StructBodyItem")])), Rule::Opt(Rule::Token(","))])
StructBodyItem = 
StructField    = 
PrefixExpr     = 
BinaryExpr      = 
CallExpr        = Rule::Seq([Rule::Node("Expr"), Rule::Token("("), Rule::Rep(Rule::Seq([Rule::Node("Expr"), Rule::Opt(Rule::Token(","))])), Rule::Token(")")])
LitExpr        = 
PrefixOp       = 
BinaryOp       = 
NumLit         = 
StrLit         = 
