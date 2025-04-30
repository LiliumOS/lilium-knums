use crate::{
    ast::{File, item::Path},
    visit::visit_file,
};
use indexmap::IndexMap;
use lccc_siphash::SipHasher;

macro_rules! def_cg_types {
    {
        $(cg $name:ident;)*
    } => {
        $(
            mod $name;
        )*
        pub fn do_cg(files: &IndexMap<Path, File>, name: &str, output: &std::path::Path, mut rand: SipHasher::<2,4>) -> std::io::Result<()> {
            use core::hash::{Hash, Hasher};
            name.hash(&mut rand);
            output.hash(&mut rand);
            match name {
                $(::core::stringify!($name) => {
                    std::fs::create_dir_all(output)?;
                    $name::write_misc(output)?;
                    for (path, file) in files {
                        let mut state = rand;
                        path.hash(&mut state);
                        let mut visitor = $name::create_file_visitor(path, state.finish());
                        visit_file(&mut visitor, file);
                        $name::write_output(visitor, output, path)?;
                    }
                    Ok(())
                })*
                _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Codegen type `{name}` not recognized")))
            }
        }
    }
}

def_cg_types! {
    cg cheader;
}
