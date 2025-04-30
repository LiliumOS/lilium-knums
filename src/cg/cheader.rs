use lilium_sys::uuid::Uuid;

use crate::{
    ast::{
        expr::{BinaryOp, UnaryOp},
        item::{Path, StructKind},
    },
    parse,
    visit::*,
};
use core::fmt::Write as _;

pub struct CBuilder {
    file: String,
    guard_var: String,
}

impl FileVisitor for CBuilder {
    #[allow(unused_variables)]
    fn visit_header(&mut self) {
        let _ = writeln!(self.file, "#ifndef {}", self.guard_var);
        let _ = writeln!(self.file, "#define {}", self.guard_var);

        let _ = writeln!(self.file, "#ifdef __cplusplus");
        let _ = writeln!(self.file, "extern \"C\"{{");
        let _ = writeln!(self.file, "#endif /* __cplusplus */");
    }

    #[allow(unused_variables)]
    fn visit_file_doc(&mut self, _: &str) {}

    #[allow(unused_variables)]
    fn visit_item(&mut self) -> impl ItemVisitor + '_ {
        ItemWriter {
            inner: &mut self.file,
        }
    }

    #[allow(unused_variables)]
    fn visit_footer(&mut self) {
        let _ = writeln!(self.file, "#ifdef __cplusplus");
        let _ = writeln!(self.file, "}}");
        let _ = writeln!(self.file, "#endif /* __cplusplus */");

        let _ = writeln!(self.file, "#endif /* {} */", self.guard_var);
    }
}

pub fn write_misc(output: &std::path::Path) -> std::io::Result<()> {
    Ok(())
}

pub fn create_file_visitor(path: &Path, cookie: u64) -> CBuilder {
    let mut guard_var = String::with_capacity(((path.components.len() * 101) >> 4) + 26);
    guard_var.push_str("__LILIUM__");

    for comp in &path.components {
        guard_var.push_str(&comp.to_ascii_uppercase());
        guard_var.push_str("_");
    }

    let _ = writeln!(guard_var, "{cookie:016X}");

    CBuilder {
        file: String::new(),
        guard_var,
    }
}

pub fn write_output(
    visitor: CBuilder,
    output: &std::path::Path,
    fpath: &Path,
) -> std::io::Result<()> {
    let mut buf = output.to_path_buf();
    buf.push("lilium-sci");
    for comp in &fpath.components {
        buf.push(comp);
    }
    buf.set_extension("h");

    std::fs::create_dir_all(buf.parent().unwrap())?;

    std::fs::write(buf, visitor.file)
}

pub struct ItemWriter<'a> {
    inner: &'a mut String,
}

impl<'a> ItemVisitor for ItemWriter<'a> {
    #[allow(unused_variables)]
    fn visit_doc_line(&mut self, _: &str) {}

    #[allow(unused_variables)]
    fn visit_use(&mut self, path: &[impl AsRef<str>]) {
        self.inner.push_str("#include <lilium-sci/");
        let mut sep = "";
        for p in path {
            self.inner.push_str(sep);
            sep = "/";
            self.inner.push_str(p.as_ref());
        }
        self.inner.push_str(".h>\n");
    }

    #[allow(unused_variables)]
    fn visit_directive(&mut self, dir: &str) {
        match dir {
            "%define_int_types" => {
                let _ = writeln!(self.inner, "typedef signed char __i8;");
                let _ = writeln!(self.inner, "typedef unsigned char __u8;");
                let _ = writeln!(self.inner, "typedef signed short __i16;");
                let _ = writeln!(self.inner, "typedef unsigned short __u16;");
                let _ = writeln!(self.inner, "typedef signed int __i32;");
                let _ = writeln!(self.inner, "typedef unsigned int __u32;");
                let _ = writeln!(self.inner, "typedef signed long __i64;");
                let _ = writeln!(self.inner, "typedef unsigned long __u64;");

                let _ = writeln!(self.inner, "typedef signed long __ilong;");
                let _ = writeln!(self.inner, "typedef unsigned long __ulong;");

                let _ = writeln!(
                    self.inner,
                    "#define __LILIUM_SIZEOF_POINTER__ (sizeof(void*))"
                );
            }
            "%define_handle_types" => {
                self.inner.push_str("#ifndef __HAS_LILIUM_HANDLE_DEF__\n");
                self.inner.push_str("#define __handle\n");
                self.inner.push_str("#define __shared_handle\n");
                self.inner.push_str("#define __HAS_LILIUM_HANDLE_DEF__\n");
                self.inner
                    .push_str("#endif /*__HAS_LILIUM_HANDLE_DEF__*/\n");
            }
            "%def_sysno" => {
                self.inner.push_str("#ifndef __LILIUM_WANT_SYSNO\n");
                self.inner.push_str("#define __LILIUM_WANT_SYSNO 1\n");
                self.inner.push_str("#endif /* __LILIUM_WANT_SYSNO */\n");
            }
            "%def_sys_proto" => {
                self.inner.push_str("#ifndef __LILIUM_WANT_SYSPROTO\n");
                self.inner.push_str("#define __LILIUM_WANT_SYSPROTO 1\n");
                self.inner.push_str("#endif /* __LILIUM_WANT_SYSPROTO */\n");
            }
            "%def_syscall_num_helpers" => {
                self.inner.push_str(
                    "#define __LILIUM_SYSNO(__subsys, __sysno) ((__subsys << 12) | __sysno)\n",
                );
                self.inner.push_str(
                    "#define __LILIUM_ERRNO(__subsys, __errno) (-((__subsys << 8) | (-__errno)))\n",
                )
            }
            x => todo!("directive {x}"),
        }
    }

    #[allow(unused_variables)]
    fn visit_type_alias(&mut self) -> impl TypeAliasVisitor + '_ {
        TypeAliasWriter::new(&mut self.inner)
    }

    #[allow(unused_variables)]
    fn visit_struct(&mut self) -> impl StructVisitor + '_ {
        StructItemWriter::new(&mut self.inner)
    }

    #[allow(unused_variables)]
    fn visit_const(&mut self) -> impl ConstItemVisitor + '_ {
        ConstItemWriter::new(&mut self.inner)
    }

    #[allow(unused_variables)]
    fn visit_fn(&mut self) -> impl FnItemVisitor + '_ {
        FnItemWriter::new(&mut self.inner)
    }
}

pub struct ConstItemWriter<'a> {
    file: &'a mut String,
    name: String,
    val: String,
}

impl<'a> ConstItemWriter<'a> {
    pub fn new(file: &'a mut String) -> Self {
        Self {
            file,
            name: String::new(),
            val: String::new(),
        }
    }
}

impl<'a> ConstItemVisitor for ConstItemWriter<'a> {
    fn visit_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn visit_ty(&mut self) -> impl TypeVisitor + '_ {
        ()
    }
    fn visit_value(&mut self) -> impl ExprVisitor + '_ {
        ExprWriter::new(&mut self.val)
    }
}

impl<'a> Drop for ConstItemWriter<'a> {
    fn drop(&mut self) {
        let _ = writeln!(self.file, "#define {} ({})", self.name, self.val);
    }
}

pub struct FnItemWriter<'a> {
    file: &'a mut String,
    name: NameAndType,
    sysno: String,
}

impl<'a> FnItemWriter<'a> {
    fn new(file: &'a mut String) -> Self {
        Self {
            file,
            name: NameAndType::default(),
            sysno: String::new(),
        }
    }
}

impl<'a> Drop for FnItemWriter<'a> {
    fn drop(&mut self) {
        let _ = writeln!(self.file, "#if __LILIUM_WANT_SYSNO");
        let _ = writeln!(
            self.file,
            "#define __SYS_{} ({})",
            self.name.name, self.sysno
        );
        let _ = writeln!(self.file, "#endif /* __LILIUM_WANT_SYSNO */");
        let _ = writeln!(self.file, "#if __LILIUM_WANT_SYSPROTO");
        let _ = writeln!(self.file, "extern {};", self.name);
        let _ = writeln!(self.file, "#endif /* __LILIUM_WANT_SYSPROTO */");
    }
}

impl<'a> FnItemVisitor for FnItemWriter<'a> {
    fn visit_name(&mut self, name: &str) {
        self.name.name = name.to_string();
    }

    fn visit_sig(&mut self) -> impl SignatureVisitor + '_ {
        SignatureWriter::new(&mut self.name)
    }

    fn visit_sysno(&mut self) -> impl ExprVisitor + '_ {
        ExprWriter::new(&mut self.sysno)
    }
}

pub struct ExprWriter<'a>(&'a mut String);

impl<'a> ExprWriter<'a> {
    pub fn new(expr: &'a mut String) -> Self {
        Self(expr)
    }
}

impl<'a> ExprVisitor for ExprWriter<'a> {
    fn visit_constant(&mut self, name: &str) {
        self.0.push_str(name);
    }
    fn visit_int_literal(&mut self, val: &str) {
        let _ = write!(self.0, "{}ULL", parse_int_literal(val));
    }

    fn visit_uuid_literal(&mut self, uuid: lilium_sys::uuid::Uuid) {
        let Uuid { minor, major } = uuid;
        let _ = write!(self.0, "((Uuid){{.minor = {minor}, .major = {major}}})");
    }
    fn visit_unary_expr(&mut self) -> impl UnaryExprVisitor + '_ {
        UnaryExprWriter::new(&mut self.0)
    }
    fn visit_binary_expr(&mut self) -> impl BinaryExprVisitor + '_ {
        BinaryExprWriter::new(&mut self.0)
    }
}

pub struct UnaryExprWriter<'a> {
    expr: &'a mut String,
    op: UnaryOp,
    inner: String,
}

impl<'a> UnaryExprWriter<'a> {
    pub fn new(expr: &'a mut String) -> Self {
        Self {
            expr,
            op: UnaryOp::Plus,
            inner: String::new(),
        }
    }
}

impl<'a> Drop for UnaryExprWriter<'a> {
    fn drop(&mut self) {
        let Self { op, inner, expr } = self;
        let op = match op {
            UnaryOp::Plus => "+",
            UnaryOp::Neg => "-",
            UnaryOp::Not => "~",
        };
        let _ = write!(expr, "({op} {inner})");
    }
}

impl<'a> UnaryExprVisitor for UnaryExprWriter<'a> {
    fn visit_inner(&mut self) -> impl ExprVisitor + '_ {
        ExprWriter::new(&mut self.inner)
    }

    fn visit_op(&mut self, op: UnaryOp) {
        self.op = op;
    }
}

pub struct BinaryExprWriter<'a> {
    expr: &'a mut String,
    op: BinaryOp,
    left: String,
    right: String,
}

impl<'a> BinaryExprWriter<'a> {
    pub fn new(expr: &'a mut String) -> Self {
        Self {
            expr,
            op: BinaryOp::Add,
            left: String::new(),
            right: String::new(),
        }
    }
}

impl<'a> Drop for BinaryExprWriter<'a> {
    fn drop(&mut self) {
        let Self {
            op,
            left,
            right,
            expr,
        } = self;
        let _ = write!(expr, "({left} {op} {right})");
    }
}

impl<'a> BinaryExprVisitor for BinaryExprWriter<'a> {
    fn visit_op(&mut self, op: BinaryOp) {
        self.op = op;
    }

    fn visit_left(&mut self) -> impl ExprVisitor + '_ {
        ExprWriter::new(&mut self.left)
    }

    fn visit_right(&mut self) -> impl ExprVisitor + '_ {
        ExprWriter::new(&mut self.right)
    }
}

pub struct TypeAliasWriter<'a> {
    inner: &'a mut String,
    name: NameAndType,
}

impl<'a> TypeAliasWriter<'a> {
    pub fn new(inner: &'a mut String) -> Self {
        Self {
            inner,
            name: NameAndType::default(),
        }
    }
}

impl<'a> Drop for TypeAliasWriter<'a> {
    fn drop(&mut self) {
        let _ = writeln!(self.inner, "typedef {};", self.name);
    }
}

impl<'a> TypeAliasVisitor for TypeAliasWriter<'a> {
    fn visit_name(&mut self, name: &str) {
        self.name.name = name.to_string();
    }

    fn visit_def(&mut self) -> impl TypeVisitor + '_ {
        TypeWriter::new(&mut self.name)
    }
}

#[derive(Default)]
struct PointersAndSuffix {
    nesting: usize,
    outer_pointers: Vec<&'static str>,
    maybe_suffix: String,
    nest_one: Option<Box<PointersAndSuffix>>,
}

impl PointersAndSuffix {
    fn is_empty(&self) -> bool {
        self.outer_pointers.is_empty() && self.maybe_suffix.is_empty()
    }
}

#[derive(Default)]
pub struct NameAndType {
    noreturn: String,
    main_type: String,
    pointers_and_suffix: PointersAndSuffix,
    name: String,
}

impl core::fmt::Display for NameAndType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.noreturn)?;
        f.write_str(" ")?;
        f.write_str(&self.main_type)?;

        let mut n = &self.pointers_and_suffix;

        let mut stack = Vec::new();

        loop {
            f.write_str(" ")?;
            for ptr in n.outer_pointers.iter().rev() {
                f.write_str(ptr)?;
            }
            stack.push(n);

            match &n.nest_one {
                Some(inner) => {
                    f.write_str("(")?;
                    n = inner;
                }
                None => {
                    f.write_str(&self.name)?;
                    for (i, r) in stack.iter().enumerate().rev() {
                        if i != 0 {
                            f.write_str(")")?;
                        }

                        f.write_str(&r.maybe_suffix)?;
                    }
                    break Ok(());
                }
            }
        }
    }
}

pub struct TypeWriter<'a>(&'a mut NameAndType);

impl<'a> TypeWriter<'a> {
    pub fn new(name: &'a mut NameAndType) -> Self {
        Self(name)
    }
}

fn bump_suffix(suffix: &mut PointersAndSuffix) {
    if !suffix.is_empty() {
        let inner = core::mem::take(suffix);
        suffix.nesting = inner.nesting + 1;
        suffix.nest_one = Some(Box::new(inner));
    }
}

impl<'a> ParamTypeVisitor for TypeWriter<'a> {
    #[allow(unused_variables)]
    fn visit_char(&mut self) {
        self.0.main_type.push_str("char");
    }

    #[allow(unused_variables)]
    fn visit_int(&mut self) -> impl IntTypeVisitor + '_ {
        SimpleNamedTypeWriter::new(&mut self.0.main_type)
    }

    #[allow(unused_variables)]
    fn visit_named(&mut self) -> impl NamedTypeVisitor + '_ {
        SimpleNamedTypeWriter::new(&mut self.0.main_type)
    }

    #[allow(unused_variables)]
    fn visit_fn_pointer(&mut self) -> impl SignatureVisitor + '_ {
        ()
    }

    #[allow(unused_variables)]
    fn visit_pointer(&mut self) -> impl PointerTypeVisitor + '_ {
        bump_suffix(&mut self.0.pointers_and_suffix);
        PointerTypeWriter::new(&mut self.0)
    }
}

impl<'a> ReturnTypeVisitor for TypeWriter<'a> {
    fn visit_never(&mut self) {
        self.0.main_type.push_str("void");
    }
    fn visit_void(&mut self) {
        self.0.main_type.push_str("void");
    }
}

impl<'a> TypeVisitor for TypeWriter<'a> {
    #[allow(unused_variables)]
    fn visit_void(&mut self) {
        self.0.main_type.push_str("void");
    }

    #[allow(unused_variables)]
    fn visit_byte(&mut self) {
        self.0.main_type.push_str("unsigned char")
    }

    #[allow(unused_variables)]
    fn visit_array(&mut self) -> impl ArrayTypeVisitor + '_ {
        ArrayWriter::new(self.0)
    }
}

pub struct ArrayWriter<'a>(&'a mut NameAndType, usize, String);

impl<'a> ArrayWriter<'a> {
    pub fn new(ty: &'a mut NameAndType) -> Self {
        ty.pointers_and_suffix.maybe_suffix.push('[');
        Self(ty, ty.pointers_and_suffix.nesting, String::new())
    }
}

impl<'a> Drop for ArrayWriter<'a> {
    fn drop(&mut self) {
        let nlevel = self.0.pointers_and_suffix.nesting - self.1;

        let mut n = &mut self.0.pointers_and_suffix;

        for _ in 0..nlevel {
            n = n.nest_one.as_deref_mut().unwrap();
        }
        let _ = write!(n.maybe_suffix, "{}]", self.2);
    }
}

impl<'a> ArrayTypeVisitor for ArrayWriter<'a> {
    fn visit_elem(&mut self) -> impl TypeVisitor + '_ {
        TypeWriter::new(&mut self.0)
    }

    fn visit_len(&mut self) -> impl ExprVisitor + '_ {
        ExprWriter::new(&mut self.2)
    }
}

pub struct SimpleNamedTypeWriter<'a>(&'a mut String);

impl<'a> SimpleNamedTypeWriter<'a> {
    fn new(name: &'a mut String) -> Self {
        Self(name)
    }
}

impl<'a> NamedTypeVisitor for SimpleNamedTypeWriter<'a> {
    fn visit_name(&mut self, name: &str) {
        *self.0 = name.to_string()
    }
    fn visit_arg(&mut self) -> impl TypeVisitor + '_ {
        ()
    }

    #[allow(unused_variables)]
    fn visit_replace(&mut self) -> impl TypeVisitor + '_ {
        *self.0 = String::new();
        self
    }
}

impl<'a> TypeVisitor for SimpleNamedTypeWriter<'a> {
    fn visit_byte(&mut self) {
        self.0.push_str("unsigned char")
    }
    fn visit_void(&mut self) {
        self.0.push_str("void")
    }
    fn visit_array(&mut self) -> impl ArrayTypeVisitor + '_ {
        ()
    }
}

impl<'a> ParamTypeVisitor for SimpleNamedTypeWriter<'a> {
    #[allow(unused_variables)]
    fn visit_char(&mut self) {
        self.0.push_str("char")
    }

    #[allow(unused_variables)]
    fn visit_int(&mut self) -> impl IntTypeVisitor + '_ {
        self
    }

    #[allow(unused_variables)]
    fn visit_named(&mut self) -> impl NamedTypeVisitor + '_ {
        self
    }

    #[allow(unused_variables)]
    fn visit_fn_pointer(&mut self) -> impl SignatureVisitor + '_ {
        ()
    }

    #[allow(unused_variables)]
    fn visit_pointer(&mut self) -> impl PointerTypeVisitor + '_ {
        ()
    }
}

impl<'a> IntTypeVisitor for SimpleNamedTypeWriter<'a> {
    fn visit_signedness(&mut self, signed: bool) {
        if signed {
            self.0.push_str("__i")
        } else {
            self.0.push_str("__u")
        }
    }

    fn visit_width(&mut self, width: crate::ast::ty::IntWidth) {
        let _ = write!(self.0, "{width}");
    }
}

pub struct SignatureWriter<'a>(&'a mut NameAndType, &'static str);

impl<'a> SignatureWriter<'a> {
    pub fn new(ty: &'a mut NameAndType) -> Self {
        ty.pointers_and_suffix.maybe_suffix.push('(');
        Self(ty, "")
    }
}

impl<'a> SignatureVisitor for SignatureWriter<'a> {
    fn visit_arg(&mut self) -> impl FnParamVisitor + '_ {
        let sep = self.1;
        self.1 = ", ";
        FnParamWriter::new(&mut self.0.pointers_and_suffix.maybe_suffix, sep)
    }
    fn visit_return(&mut self) -> impl ReturnTypeVisitor + '_ {
        self.0.pointers_and_suffix.maybe_suffix.push(')');
        TypeWriter::new(self.0)
    }
}

pub struct FnParamWriter<'a>(&'a mut String, &'static str, NameAndType);

impl<'a> FnParamWriter<'a> {
    pub fn new(targ: &'a mut String, sep: &'static str) -> Self {
        Self(targ, sep, NameAndType::default())
    }
}

impl<'a> FnParamVisitor for FnParamWriter<'a> {
    fn visit_name(&mut self, name: &str) {
        self.2.name = name.to_string();
    }

    fn visit_type(&mut self) -> impl ParamTypeVisitor + '_ {
        TypeWriter::new(&mut self.2)
    }
}

impl<'a> Drop for FnParamWriter<'a> {
    fn drop(&mut self) {
        let _ = write!(self.0, "{}{}", self.1, self.2);
    }
}

pub struct PointerTypeWriter<'a>(&'a mut NameAndType);

impl<'a> PointerTypeWriter<'a> {
    pub fn new(ty: &'a mut NameAndType) -> Self {
        Self(ty)
    }
}

impl<'a> PointerTypeVisitor for PointerTypeWriter<'a> {
    fn visit_pointee(&mut self) -> impl TypeVisitor + '_ {
        TypeWriter::new(self.0)
    }

    fn visit_pointer_kind(&mut self, kind: crate::ast::ty::PointerKind) {
        match kind {
            crate::ast::ty::PointerKind::Const => {
                self.0.pointers_and_suffix.outer_pointers.push("const*")
            }
            crate::ast::ty::PointerKind::Mut => self.0.pointers_and_suffix.outer_pointers.push("*"),
            crate::ast::ty::PointerKind::Handle => self
                .0
                .pointers_and_suffix
                .outer_pointers
                .push("* __handle "),
            crate::ast::ty::PointerKind::Shared => self
                .0
                .pointers_and_suffix
                .outer_pointers
                .push("* __shared_handle "),
        }
    }
}

pub struct StructItemWriter<'a> {
    file: &'a mut String,
    kind: StructKind,
    name: String,
    aligned_attr: Option<String>,
    body: String,
    option_body: Vec<String>,
    option_id: Option<String>,
}

impl<'a> StructItemWriter<'a> {
    fn new(file: &'a mut String) -> Self {
        Self {
            file,
            kind: StructKind::Struct,
            name: String::new(),
            aligned_attr: None,
            body: String::new(),
            option_body: Vec::new(),
            option_id: None,
        }
    }
}

impl<'a> Drop for StructItemWriter<'a> {
    fn drop(&mut self) {
        let Self {
            file,
            kind,
            name,
            aligned_attr,
            body,
            option_body,
            option_id,
            ..
        } = self;

        let aligned_attr = aligned_attr
            .as_deref()
            .map(|e| format!("__attribute__((__aligned({e})))"))
            .unwrap_or_default();

        let mut option_body_inner = None;

        for (n, item) in option_body.iter().enumerate() {
            let _ = write!(
                option_body_inner.get_or_insert(String::new()),
                "unsigned char __pad{n}[{item}]; "
            );
        }

        let option_body = option_body_inner
            .as_deref()
            .map(|e| format!("struct{{ ExtendedOptionHead head; union{{{e}}}; }};"))
            .unwrap_or_default();
        let _ = writeln!(
            file,
            "typedef {kind} {name} {body} {option_body}}} {name} {aligned_attr};"
        );

        if let Some(id) = option_id.as_deref() {
            let mut new_name = String::with_capacity((name.len() * 3) >> 1);
            let mut last = 0;
            for (i, _) in name.match_indices(|c: char| c.is_ascii_uppercase()) {
                new_name.push('_');
                new_name.push_str(&name[last..i]);
                last = i;
            }
            new_name.push('_');
            new_name.push_str(&name[last..]);
            new_name.make_ascii_uppercase();
            let _ = writeln!(file, "#define __LILIUM{new_name}_ID ({id})");
        }
    }
}

impl<'a> StructVisitor for StructItemWriter<'a> {
    #[allow(unused_variables)]
    fn visit_kind(&mut self, kind: StructKind) {
        self.kind = kind;
    }

    #[allow(unused_variables)]
    fn visit_name(&mut self, name: &str) {
        self.name = name.to_string()
    }

    #[allow(unused_variables)]
    fn visit_generic_arg(&mut self, name: &str) {
        /* do nothing */
    }

    #[allow(unused_variables)]
    fn visit_option(&mut self) -> impl StructOptionVisitor + '_ {
        self
    }

    #[allow(unused_variables)]
    fn visit_body(&mut self) -> impl StructBodyVisitor + '_ {
        self.body.push('{');
        StructBodyWriter::new(&mut self.body)
    }

    #[allow(unused_variables)]
    fn visit_opaque(&mut self) -> impl OpaqueVisitor + '_ {
        ()
    }
}

impl<'a> StructOptionVisitor for StructItemWriter<'a> {
    fn visit_align(&mut self) -> impl ExprVisitor + '_ {
        ExprWriter::new(self.aligned_attr.insert(String::new()))
    }

    fn visit_option_id(&mut self) -> impl ExprVisitor + '_ {
        ExprWriter::new(self.option_id.insert(String::new()))
    }

    fn visit_option_body_pad(&mut self) -> impl ExprVisitor + '_ {
        self.option_body.push(String::new());
        ExprWriter::new(self.option_body.last_mut().unwrap())
    }
}

pub struct StructBodyWriter<'a> {
    body: &'a mut String,
    sep: &'static str,
    pad_name_and_type: Option<NameAndType>,
}

impl<'a> StructBodyWriter<'a> {
    pub fn new(body: &'a mut String) -> Self {
        Self {
            body,
            sep: "",
            pad_name_and_type: None,
        }
    }
}

impl<'a> Drop for StructBodyWriter<'a> {
    fn drop(&mut self) {
        if let Some(pad_name_and_type) = &self.pad_name_and_type {
            let _ = write!(self.body, "{pad_name_and_type};");
        }
    }
}

impl<'a> StructBodyVisitor for StructBodyWriter<'a> {
    fn visit_field(&mut self) -> impl FieldVisitor + '_ {
        FieldWriter::new(&mut self.body)
    }

    fn visit_pad(&mut self) -> impl TypeVisitor + '_ {
        self.body.push_str(self.sep);

        TypeWriter::new(self.pad_name_and_type.insert(NameAndType {
            name: String::from("__pad"),
            ..Default::default()
        }))
    }

    fn visit_pad_to(&mut self) -> impl ExprVisitor + '_ {
        ()
    }
}

pub struct FieldWriter<'a>(&'a mut String, NameAndType);

impl<'a> FieldWriter<'a> {
    fn new(body: &'a mut String) -> Self {
        Self(body, NameAndType::default())
    }
}

impl<'a> Drop for FieldWriter<'a> {
    fn drop(&mut self) {
        let _ = write!(self.0, "{};", self.1);
    }
}

impl<'a> FieldVisitor for FieldWriter<'a> {
    fn visit_doc_line(&mut self, st: &str) {}
    fn visit_name(&mut self, n: &str) {
        self.1.name = n.to_string();
    }

    fn visit_ty(&mut self) -> impl TypeVisitor + '_ {
        TypeWriter::new(&mut self.1)
    }
}
