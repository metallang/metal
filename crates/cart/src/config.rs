// SPDX-License-Identifier: MIT

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::Error;

pub trait VerifyConfig {
    fn verify_config(&self) -> Result<(), Error>;
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub pkg: Package,
    pub deps: HashMap<String, Dependency>,
}

impl VerifyConfig for Config {
    fn verify_config(&self) -> Result<(), Error> {
        self.pkg.verify_config()?;
        self.deps.values().try_for_each(|d| d.verify_config())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
}

impl VerifyConfig for Package {
    fn verify_config(&self) -> Result<(), Error> {
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

impl VerifyConfig for Dependency {
    fn verify_config(&self) -> Result<(), Error> {
        if let None = self.git
            && let Some(_) = self.tag
        {
            return Err(Error::TagIncluded);
        }

        Ok(())
    }
}
