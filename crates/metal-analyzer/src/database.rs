// SPDX-License-Identifier: MIT

use std::{collections::HashMap, fs, path::PathBuf};

use metal_mir::parcel::Module;
use salsa::Storage;

use crate::error::AnalyzerError;

#[salsa::db]
pub trait Db: salsa::Database {}

#[salsa::db]
#[derive(Clone)]
pub struct AnalyzerDatabase {
    pub storage: Storage<Self>,
    pub modules: HashMap<String, Module>,
}

#[salsa::db]
impl salsa::Database for AnalyzerDatabase {
    fn salsa_event(&self, _event: &dyn Fn() -> salsa::Event) {}
}

#[salsa::db]
impl Db for AnalyzerDatabase {}

impl AnalyzerDatabase {
    pub fn load_from_directory(path: PathBuf) -> Result<Self, AnalyzerError> {
        let mut modules = HashMap::new();
        for dir in fs::read_dir(path)?.flatten() {
            // NOTE: the analyzer cache dir should only be one directory large
            if dir.metadata()?.is_dir() | dir.metadata()?.is_symlink() {
                continue;
            }

            let contents = fs::read_to_string(dir.path())?;
            let module: Module = bincode::deserialize(contents.as_bytes())?;
            modules.insert(module.name.clone(), module);
        }
        Ok(Self {
            storage: Storage::default(),
            modules,
        })
    }
}
