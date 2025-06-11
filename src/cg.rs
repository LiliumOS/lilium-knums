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
        pub fn do_cg(files: &IndexMap<Path, File>, name: &str, output: &std::path::Path, mut rand: SipHasher::<2,4>, fileset: &[impl AsRef<std::path::Path>]) -> std::io::Result<()> {
            use core::hash::{Hash, Hasher};
            use std::io::Write;
            name.hash(&mut rand);
            output.hash(&mut rand);

            let stamp_file = match name {
                $(::core::stringify!($name) => {
                    std::fs::create_dir_all(output)?;
                    $name::write_misc(output, files.keys())?;
                    for (path, file) in files {
                        let mut state = rand;
                        path.hash(&mut state);
                        let mut visitor = $name::create_file_visitor(path, state.finish());
                        visit_file(&mut visitor, file);
                        $name::write_output(visitor, output, path)?;
                    }
                    $name::make_stamp(output)?
                })*
                _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Codegen type `{name}` not recognized")))?
            };

            let mut file = std::fs::File::create("knums.d")?;

            write!(file, "{}:", stamp_file.display())?;

            for input in fileset {
                write!(file, " {}", input.as_ref().display())?;
            }
            writeln!(file)

        }
    }
}

def_cg_types! {
    cg cheader;
    cg markdown;
}
