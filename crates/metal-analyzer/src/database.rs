use std::{fs, path::PathBuf};

use rkyv::rancor;
use salsa::Storage;

use crate::module::{ArchivedModule, Module};

#[salsa::db]
pub trait Db: salsa::Database {}

#[salsa::db]
#[derive(Clone)]
pub struct AnalyzerDatabase {
    pub storage: Storage<Self>,
    pub modules: Vec<Module>,
}

#[salsa::db]
impl salsa::Database for AnalyzerDatabase {
    fn salsa_event(&self, _event: &dyn Fn() -> salsa::Event) {}
}

#[salsa::db]
impl Db for AnalyzerDatabase {}

impl AnalyzerDatabase {
    pub fn load_from_directory(path: PathBuf) -> Self {
        let mut modules = Vec::new();
        for dir in fs::read_dir(path).unwrap().flatten() {
            // NOTE: the analyzer cache dir should only be one directory large
            if dir.metadata().unwrap().is_dir() | dir.metadata().unwrap().is_symlink() {
                continue;
            }

            let contents = fs::read_to_string(dir.path()).unwrap();
            let archived =
                rkyv::access::<ArchivedModule, rancor::Error>(contents.as_bytes()).unwrap();
            let module = rkyv::deserialize::<Module, rancor::Error>(archived).unwrap();
            modules.push(module);
        }
        Self {
            storage: Storage::default(),
            modules,
        }
    }
}
