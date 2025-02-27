// SPDX-License-Identifier: MIT

use std::{
    fs::{self, File},
    path::PathBuf,
};

use llvm_sys::target::{
    LLVM_InitializeNativeAsmParser, LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget,
};
use metal_codegen_llvm::{
    core::compile_module,
    linking::{get_lld_dir, link},
    llc::{get_llc_dir, ir_llc},
};
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
    pub name: Option<String>,
    pub input_type: InputType,
    pub relative_paths: Vec<PathBuf>,
    pub triple: Option<String>,
}

impl tapcli::Command for BuildCommand {
    type Error = Error;

    fn parse(parser: &mut tapcli::Parser) -> Result<Self, Self::Error> {
        let mut cmd: BuildCommand = Self {
            name: None,
            input_type: InputType::Metal,
            relative_paths: Vec::new(),
            triple: None,
        };

        while let Some(arg) = parser.next() {
            match arg.as_ref() {
                tapcli::ArgRef::Long("input-type") | tapcli::ArgRef::Short('i') => {
                    let value = parser.next().ok_or(Error::InsufficientArguments)?;

                    cmd.input_type = InputType::try_from(value)?;
                }
                tapcli::ArgRef::Long("triple") => {
                    let value = parser.next().ok_or(Error::InsufficientArguments)?;

                    cmd.triple = Some(value.to_string());
                }
                tapcli::ArgRef::Long("name") | tapcli::ArgRef::Short('n') => {
                    let value = parser.next().ok_or(Error::InsufficientArguments)?;

                    cmd.name = Some(value.to_string());
                }
                tapcli::ArgRef::Value(val) => {
                    cmd.relative_paths.push(PathBuf::from(val));
                }
                _ => return Err(Error::UnrecognizedArgument(arg)),
            }
        }

        Ok(cmd)
    }

    fn run<'a>(self) -> Result<Self::Output, Self::Error> {
        Target::setup()?;

        match self.input_type {
            InputType::MIR => {
                let mut modules = Vec::new();

                for path in self.relative_paths.iter() {
                    if fs::metadata(path)?.is_dir() {
                        let read_dir = fs::read_dir(path)?;
                        for entry in read_dir.into_iter().flatten() {
                            // TODO: recursive folder searching.
                            if entry.metadata()?.is_dir() {
                                continue;
                            };

                            let p = entry.path();

                            let file = File::open(p)?;

                            let module: Module = from_reader(file)?;
                            modules.push(module);
                        }
                    } else {
                        let file = File::open(path)?;

                        let module: Module = from_reader(file)?;
                        modules.push(module);
                    }
                }

                let mut object_files = Vec::new();
                Target::reset_llvm()?;

                let mut dll = true;
                for module in modules.iter() {
                    // TODO: move this into LLVM codegen once rykv comes around.
                    unsafe {
                        assert_ne!(1, LLVM_InitializeNativeTarget());
                        assert_ne!(1, LLVM_InitializeNativeAsmPrinter());
                        assert_ne!(1, LLVM_InitializeNativeAsmParser());
                    }

                    let llvm_ir = compile_module(module, true, &self.triple);

                    let input_dir = Target::write_llvm_ir(&module.name, &llvm_ir)?;

                    let obj_fn = format!("{}/{}.o", "./target/.llvm-linking-cache", &module.name);

                    let llc = get_llc_dir();
                    ir_llc(
                        &llc,
                        &input_dir,
                        &obj_fn,
                        metal_codegen_llvm::llc::Optimization::O0,
                        metal_codegen_llvm::llc::LLCFormat::NativeObject,
                    );
                    object_files.push(obj_fn);

                    if !module.library {
                        dll = false;
                    }
                }

                let lld = get_lld_dir();

                let output = if cfg!(target_os = "windows") {
                    if dll {
                        format!(
                            "./target/build/{}.dll",
                            self.name.unwrap_or("output".to_string())
                        )
                    } else {
                        format!(
                            "./target/build/{}.exe",
                            self.name.unwrap_or("output".to_string())
                        )
                    }
                } else {
                    format!(
                        "./target/build/{}",
                        self.name.unwrap_or("output".to_string())
                    )
                };

                Target::reset_build()?;

                link(lld, object_files, &output, &vec!["/DEBUG".to_string()], dll);

                println!("Compilation results have been written to target");
            }
            _ => return Err(Error::Unimplemented("metal".to_string())),
        }

        Ok(())
    }
}
