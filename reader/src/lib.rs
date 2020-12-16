#[macro_use]
extern crate strum_macros;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub use scie_code_file::CodeFile;
pub use scie_token_element::TokenElement;

use crate::simple_hir::SirProgram;
use crate::transpiler::{CTranspiler, Transpiler};

pub mod domain;
pub mod retoken;
pub mod scie_code_file;
pub mod scie_token_element;
pub mod simple_hir;
pub mod transpiler;

pub fn read_scie_data(path: &Path) -> Vec<CodeFile> {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    return serde_json::from_str(&data).expect("error file");
}

fn reader(path: &mut PathBuf) -> SirProgram {
    let vec = read_scie_data(&*path);

    let transpiler = CTranspiler {};
    transpiler.transpile(vec)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::reader;

    #[test]
    fn should_build_first_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("../_fixtures/c/hello.c.json");

        let sir_program = reader(&mut path);

        println!("{:?}", sir_program);

        assert_eq!("main", sir_program.name);
    }
}
