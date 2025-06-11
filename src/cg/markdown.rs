use crate::ast::item::Path;
use crate::visit::*;
use std::path::PathBuf;

pub struct MarkdownGen {
    file: String,
}

impl FileVisitor for MarkdownGen {
    #[allow(unused_variables)]
    fn visit_header(&mut self) {
        todo!()
    }

    #[allow(unused_variables)]
    fn visit_file_doc(&mut self, doc: &str) {
        todo!()
    }

    #[allow(unused_variables)]
    fn visit_item(&mut self) -> impl ItemVisitor + '_ {
        todo!()
    }

    #[allow(unused_variables)]
    fn visit_footer(&mut self) {
        todo!()
    }
}

pub fn write_misc(
    output: &std::path::Path,
    paths: impl Iterator<Item: AsRef<Path>>,
) -> std::io::Result<()> {
    let mut summary_path = output.to_path_buf();
    summary_path.push("src/SUMMARY.md");
    let mut summary = std::fs::File::create(path)?;
    Ok(())
}

pub fn create_file_visitor(path: &Path, cookie: u64) -> MarkdownGen {
    MarkdownGen {
        file: String::new(),
    }
}

pub fn make_stamp(output: &std::path::Path) -> std::io::Result<PathBuf> {
    let mut path = output.to_path_buf();
    path.push("stamp");
    std::fs::File::create(&path)?;

    Ok(path)
}

pub fn write_output(
    visitor: MarkdownGen,
    output: &std::path::Path,
    fpath: &Path,
) -> std::io::Result<()> {
    let mut buf = output.to_path_buf();
    buf.push("src");
    for comp in &fpath.components {
        buf.push(comp);
    }
    buf.set_extension("md");

    std::fs::create_dir_all(buf.parent().unwrap())?;

    std::fs::write(buf, visitor.file)
}
