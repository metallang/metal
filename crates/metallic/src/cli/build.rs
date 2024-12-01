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

use crate::{error::Error, target::Target};

pub enum InputType {
    Metal,
    MIR,
}

impl TryFrom<tapcli::Arg> for InputType {
    type Error = Error;

    fn try_from(value: tapcli::Arg) -> Result<Self, Self::Error> {
        match value {
            tapcli::Arg::Value(val) => match val.as_str() {
                "metal" => Ok(Self::Metal),
                "mir" => Ok(Self::MIR),
                _ => Err(Error::UnsupportedInputType(val)),
            },
            _ => Err(Error::UnrecognizedArgument(value)),
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

                    cmd.input_type = InputType::try_from(value)?;
                }
                tapcli::ArgRef::Value(val) => {
                    // NOTE: PathBuf returns an Infallible here.
                    // So as the name suggests this unwrap *shouldn't* fail.
                    cmd.relative_paths.push(PathBuf::from_str(val).unwrap());
                }
                _ => return Err(Error::UnrecognizedArgument(arg)),
            }
        }

        Ok(cmd)
    }

    fn run<'a>(self) -> Result<Self::Output, Self::Error> {
        Target::setup().map_err(Error::TargetError)?;

        match self.input_type {
            InputType::MIR => {
                let mut modules = Vec::new();

                for path in self.relative_paths.iter() {
                    if fs::metadata(path).map_err(Error::IOError)?.is_dir() {
                        let read_dir = fs::read_dir(path).map_err(Error::IOError)?;
                        for entry in read_dir.into_iter().flatten() {
                            // TODO: recursive folder searching.
                            if entry.metadata().map_err(Error::IOError)?.is_dir() {
                                continue;
                            };

                            let p = entry.path();

                            let file = File::open(p).map_err(Error::IOError)?;

                            let module: Module =
                                from_reader(file).map_err(Error::DeserializationError)?;
                            modules.push(module);
                        }
                    } else {
                        let file = File::open(path).map_err(Error::IOError)?;

                        let module: Module =
                            from_reader(file).map_err(Error::DeserializationError)?;
                        modules.push(module);
                    }
                }

                // TODO: move this into LLVM codegen once rykv comes around.
                unsafe {
                    assert_ne!(1, LLVM_InitializeNativeTarget());
                    assert_ne!(1, LLVM_InitializeNativeAsmPrinter());
                    assert_ne!(1, LLVM_InitializeNativeAsmParser());
                }

                Target::reset_llvm().map_err(Error::TargetError)?;
                for module in modules.iter() {
                    let llvm_ir = compile_module(module, true);

                    Target::write_llvm_ir(&module.name, &llvm_ir).map_err(Error::TargetError)?;
                }

                println!("Compilation results have been written to target");
            }
            _ => return Err(Error::Unimplemented("metal".to_string())),
        }

        Ok(())
    }
}
