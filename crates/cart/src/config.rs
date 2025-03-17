// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::Error;

pub trait Verify {
    fn verify(&self) -> Result<(), Error>;
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub pkg: Package,
    pub deps: HashMap<String, Dependency>,
}

impl Verify for Manifest {
    fn verify(&self) -> Result<(), Error> {
        self.pkg.verify()?;
        self.deps.values().try_for_each(|d| d.verify())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl Verify for Package {
    fn verify(&self) -> Result<(), Error> {
        if semver::Version::parse(&self.version).is_err() {
            return Err(Error::InvalidPackageVersion(self.version.clone()));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Dependency {
    #[serde(default)]
    git: Option<String>,
    #[serde(default)]
    tag: Option<String>,
    #[serde(default)]
    features: Option<Vec<String>>,
}

impl Verify for Dependency {
    fn verify(&self) -> Result<(), Error> {
        if self.git.is_none() && self.tag.is_some() {
            return Err(Error::TagIncluded);
        }

        Ok(())
    }
}
