#![feature(never_type, macro_metavar_expr)]

use std::{ffi::OsStr, fs::File, hash::Hash, path::Path};

use indexmap::IndexMap;
use lccc_siphash::{RawSipHasher, SipHasher};
use lexer::Token;
use logos::Logos;
use parse::parse_file;
use visit::visit_file;

mod ast;
mod cg;
mod defs;
mod lexer;
mod nzlit;
mod parse;
mod visit;

fn main() -> std::io::Result<()> {
    let (k0, k1) = match std::env::var("KNUMS_SEED") {
        Ok(seed) => {
            let master = seed.parse::<u64>().expect("KNUMS_SEED must be set");

            let mut seed_expander = RawSipHasher::<2, 4>::from_keys(
                master ^ 0x428a2f98d728ae22,
                master ^ 0x7137449123ef65cd,
            );

            // 0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1
            seed_expander.update(0x6a09e667f3bcc908);
            seed_expander.update(0xbb67ae8584caa73b);
            seed_expander.update(0x3c6ef372fe94f82b);
            seed_expander.update(0xa54ff53a5f1d36f1);
            let k0 = seed_expander.finish();
            // 0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179
            seed_expander.update(0x510e527fade682d1);
            seed_expander.update(0x9b05688c2b3e6c1f);
            seed_expander.update(0x1f83d9abfb41bd6b);
            seed_expander.update(0x5be0cd19137e2179);
            let k1 = seed_expander.finish();

            (k0, k1)
        }
        Err(_) => rand::random(),
    };

    let mut rand = SipHasher::<2, 4>::new_with_keys(k0, k1);

    let path = Path::new("defs"); // TODO: Unhardcode this later
    let mut inputs = IndexMap::new();
    let mut fileset = Vec::new();
    for file in walkdir::WalkDir::new(path) {
        let file = file?;

        let name = file.path().strip_prefix(path).unwrap();
        name.hash(&mut rand);

        if name.extension().is_some_and(|v| v == OsStr::new("knum")) {
            let content = std::fs::read_to_string(file.path())?;
            content.hash(&mut rand);
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

            let mut lexer = Token::lexer(&content).flatten().peekable();
            let body = parse_file(&mut lexer);
            inputs.insert(path, body);
            fileset.push(file.into_path());
        }
    }

    cg::do_cg(&inputs, "cheader", Path::new("include"), rand, &fileset)
}
