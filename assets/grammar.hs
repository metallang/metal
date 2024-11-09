-- A block of items
block = "{" ~ item* ~ "}";
-- An identifier
ident = XID_Start ~ XID_Continue*;
-- An optional visibility qualifier
maybe_vis = "pub"?;
-- An optional mutability qualifier
maybe_mut = "mut"?;
-- A default variable value specifier
expr_specifier = "=" ~ expr;
-- A type specifier, also called a type annotation
ty_specifier = ":" ~ ty;
-- One or more `$rule`s separated by `$del`s with an optional trailing `$del`
delimited($del, $rule) = $rule ~ ($del ~ $rule)* ~ $del?;

-- A constant definition
const_item = maybe_vis ~ "const" ~ ident ~ ty_specifier? ~ expr_specifier;
-- An enum definition
enum_item = maybe_vis ~ "enum" ~ ident ~ "{" ~ enum_body ~ "}";
-- A standalone expression
expr_item = expr ~ ";";
-- A function definition
fn_item = maybe_vis ~ "def" ~ ident ~ "(" ~ fn_inputs? ~ ")" ~ ty_specifier? ~ block;
-- An import declaration
import_item = maybe_vis ~ "import" ~ import_tree ~ ";";
-- A struct definition
struct_item = maybe_vis ~ "struct" ~ ident ~ "{" ~ struct_body ~ "}";

-- An enum's "insides"
enum_body = delimited(",", enum_body_item);
-- An item associated with an enum
enum_body_item = enum_variant | fn_item;
-- An enum variant
enum_variant = ident ~ ( "(" ~ ty? ~ ")" )?;

-- Inputs of a function
fn_inputs = delimited(",", fn_input);
-- An input of a function, also called a parameter
fn_input = maybe_mut ~ ident ~ ty_specifier ~ expr_specifier?;

-- An import declaration's import "tree"
import_tree = import_leaf | import_segment | import_multiple;
-- An import tree "leaf", the final component of any import tree
import_leaf = ident;
-- An import tree "segment", an intermediate component in an import tree
import_segment = ident ~ "." ~ import_tree;
-- A branch in an import tree from which multiple subtrees descend
import_multiple = "{" ~ delimited(",", import_tree)? ~ "}";

-- A struct's "insides"
struct_body = delimited(",", struct_body_item);
-- An item associated with a struct
struct_body_item = struct_field | fn_item;
-- A struct field
struct_field = maybe_vis ~ ident ~ ty_specifier;

-- An item
item = const_item | enum_item | expr_item | fn_item | import_item | struct_item;

-- A type expression
ty = ident;

-- An expression, also called a value
expr = ident;
