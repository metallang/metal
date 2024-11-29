// SPDX-License-Identifier: MIT

use std::{
    fs::{self, File},
    path::PathBuf,
    str::FromStr,
};

use llvm_sys::target::{
    LLVM_InitializeNativeAsmParser, LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget,
};
use metal_codegen_llvm::core::compile_module;
use metal_mir::parcel::Module;
use ron::de::from_reader;

use crate::error::Error;

pub enum InputType {
    Metal,
    MIR,
}

impl From<tapcli::Arg> for InputType {
    fn from(value: tapcli::Arg) -> Self {
        match value {
            tapcli::Arg::Value(val) => match val.as_str() {
                "metal" => Self::Metal,
                "mir" => Self::MIR,
                _ => panic!("input type is unsupported"),
            },
            _ => panic!("unexpected next argument"),
        }
    }
}

pub struct BuildCommand {
    pub input_type: InputType,
    pub relative_paths: Vec<PathBuf>,
}

impl tapcli::Command for BuildCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let mut cmd: BuildCommand = Self {
            input_type: InputType::Metal,
            relative_paths: Vec::new(),
        };

        while let Some(arg) = parser.next() {
            match arg.as_ref() {
                tapcli::ArgRef::Long("input-type") | tapcli::ArgRef::Short('i') => {
                    let value = parser.next().ok_or(Error::InsufficientArguments)?;

                    cmd.input_type = InputType::from(value);
                }
                tapcli::ArgRef::Value(val) => {
                    println!("{}", val);
                    cmd.relative_paths.push(PathBuf::from_str(val).unwrap());
                }
                _ => return Err(Error::UnrecognizedArgument(arg)),
            }
        }

        Ok(cmd)
    }

    fn run<'a>(self) -> Result<Self::Output, Self::Error> {
        fs::create_dir_all(PathBuf::from_str("./target/ice").unwrap()).unwrap();
        if let InputType::MIR = self.input_type {
            let mut modules = Vec::new();

            for path in self.relative_paths.iter() {
                let file = File::open(path).unwrap();

                let module: Module = from_reader(file).unwrap();
                modules.push(module);
            }

            // TODO: move this into LLVM codegen once rykv comes around.
            unsafe {
                assert_ne!(1, LLVM_InitializeNativeTarget());
                assert_ne!(1, LLVM_InitializeNativeAsmPrinter());
                assert_ne!(1, LLVM_InitializeNativeAsmParser());
            }

            for module in modules.iter() {
                let llvm_ir = compile_module(module, true);

                fs::write(
                    &("./target/ice/".to_string() + &module.name + ".ll"),
                    &llvm_ir,
                )
                .unwrap();
            }

            println!("Success! Relevant modules have been loaded into the target ICE");
        }
        Ok(())
    }
}
