// SPDX-License-Identifier: MIT

use std::fs;

pub struct Target;

const CACHEDIR_CONTENTS: &str = r#"Signature: 8a477f597d28d172789f06886806bc55
# This file is a cache directory tag created by metallic.
# For information about cache directory tags, see:
#	http://www.brynosaurus.com/cachedir/"#;

const GITIGNORE_CONTENTS: &str = "*";

impl Target {
    pub fn setup() {
        // ensure the directories all exist
        fs::create_dir_all("./target/.llvm").unwrap();

        if !fs::exists("./target/CACHEDIR.TAG").unwrap() {
            fs::write("./target/CACHEDIR.TAG", CACHEDIR_CONTENTS).unwrap();
        }
        if !fs::exists("./target/.gitignore").unwrap() {
            fs::write("./target/.gitignore", GITIGNORE_CONTENTS).unwrap();
        }
    }


    pub fn refresh_llvm() {
        fs::remove_dir_all("./target/.llvm").unwrap();
        fs::create_dir_all("./target/.llvm").unwrap();
    }

    pub fn write_llvm_ir(module: &str, contents: &[u8]) {
        fs::write("./target/.llvm/".to_string() + module + ".ll", contents).unwrap();
    }
}
