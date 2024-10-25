pub use inkwell;

pub trait CodeGen {
    fn codegen(
        &self,
        ctx: &inkwell::context::Context,
        builder: &inkwell::builder::Builder,
        module: &inkwell::module::Module
    );
}