use std::{ffi::OsStr, fs::File, path::Path};

use indexmap::IndexMap;
use lexer::Token;
use logos::Logos;
use parse::parse_file;

mod ast;
mod lexer;
mod nzlit;
mod parse;

fn main() -> std::io::Result<()> {
    let path = Path::new("defs"); // TODO: Unhardcode this later
    let mut inputs = IndexMap::new();
    for file in walkdir::WalkDir::new(path) {
        let file = file?;
        let name = file.path().strip_prefix(path).unwrap();

        if name.extension().is_some_and(|v| v == OsStr::new("knum")) {
            let file = std::fs::read_to_string(file.path())?;
            let name = name.as_os_str().to_string_lossy();
            let path = ast::item::Path {
                components: name
                    .strip_suffix(".knum")
                    .unwrap()
                    .split('/')
                    .map(String::from)
                    .collect::<Vec<_>>(),
                has_leading: false,
            };

            let mut lexer = Token::lexer(&file).flatten().peekable();
            let file = parse_file(&mut lexer);
            inputs.insert(path, file);
        }
    }

    println!("Files {inputs:#?}");

    Ok(())
}
