// SPDX-License-Identifier: MIT

use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use metal_mir::parcel::Module;
use salsa::Storage;

use crate::error::Error;

#[salsa::db]
pub trait Db: salsa::Database {}

#[salsa::db]
#[derive(Clone)]
pub struct AnalyzerDatabase {
    pub storage: Storage<Self>,
    pub logs: Arc<Mutex<Vec<String>>>,
    pub modules: HashMap<String, Module>,
}

#[salsa::db]
impl salsa::Database for AnalyzerDatabase {
    fn salsa_event(&self, event: &dyn Fn() -> salsa::Event) {
        let event = event();

        #[cfg(debug_assertions)]
        eprintln!("Compiler salsa event: {event:?}");

        let logs = &mut *self.logs.lock().unwrap();

        if let salsa::EventKind::WillExecute { .. } = event.kind {
            logs.push(format!("Event: {event:?}"));
        }
    }
}

#[salsa::db]
impl Db for AnalyzerDatabase {}

impl AnalyzerDatabase {
    pub fn load_from_directory(path: PathBuf) -> Result<Self, Error> {
        let mut modules = HashMap::new();
        for dir in fs::read_dir(path)?.flatten() {
            // NOTE: the analyzer cache dir is flat meaning doesn't support child directories.
            if dir.metadata()?.is_dir() | dir.metadata()?.is_symlink() {
                continue;
            }

            let contents = fs::read_to_string(dir.path())?;
            let module: Module = bincode::deserialize(contents.as_bytes())?;
            modules.insert(module.name.clone(), module);
        }
        Ok(Self {
            storage: Storage::default(),
            logs: Arc::default(),
            modules,
        })
    }

    pub fn save_to_directory(&self, path: PathBuf) -> Result<(), Error> {
        let p = path.to_str().unwrap().to_string();
        for module in self.modules.values() {
            let ser = bincode::serialize(module)?;
            fs::write(format!("{}{}", &p, &module.name), ser)?;
        }

        Ok(())
    }
}
