use lilium_sys::uuid::Uuid;

use crate::ast::{expr::*, item::*, ty::*, *};

macro_rules! make_visitor {
    {
        |$vid:ident|
        $($vis:vis trait $name:ident ($global_name:ident ) $(: $supertrait:ident)? {
            |$inner:ident: $ast_elem:ty| $block:block
            $(fn $visitor:ident(&mut self $(,$args:ident : $args_ty:ty)* $(,)?) $(-> $ret_ty:ty)?;)*
        })*
    } => {
        $(

            #[allow(unused_mut)]
            $vis fn $global_name(mut $vid: impl $name, $inner: &$ast_elem) {
                if !$name::is_some(&$vid) {
                    return;
                }
                $block
            }

            $vis trait $name $(: $supertrait)? {
                fn is_some(&self) -> bool {
                    true
                }
                $(
                    #[allow(unused_variables)]
                    fn $visitor(&mut self $(,$args : $args_ty)*) $(-> $ret_ty)?;
                )*
            }

            impl $name for () {
                fn is_some(&self) -> bool {
                    false
                }
                $(
                    #[allow(unused_variables)]
                    fn $visitor(&mut self $(,$args : $args_ty)*) $(-> $ret_ty)? {
                        ()
                    }
                )*
            }

            impl $name for ! {
                fn is_some(&self) -> bool {
                    *self
                }
                $(
                    #[allow(unused_variables)]
                    fn $visitor(&mut self $(,_ : $args_ty)*) $(-> $ret_ty)? {
                        *self
                    }
                )*
            }

            impl<V: $name + ?Sized> $name for &mut V {
                fn is_some(&self) -> bool {
                    <V as $name>::is_some(self)
                }

                $(
                    fn $visitor(&mut self $(,$args : $args_ty)*) $(-> $ret_ty)? {
                        <V as $name>::$visitor(self $(, $args)*)
                    }
                )*
            }

            impl<V: $name + ?Sized> $name for Box<V> {
                fn is_some(&self) -> bool {
                    <V as $name>::is_some(self)
                }

                $(
                    fn $visitor(&mut self $(,$args : $args_ty)*) $(-> $ret_ty)? {
                        <V as $name>::$visitor(self $(, $args)*)
                    }
                )*
            }

            impl<V: $name> $name for Option<V> {
                fn is_some(&self) -> bool {
                    self.as_ref().is_some_and(|v| <V as $name>::is_some(v))
                }

                $(
                    fn $visitor(&mut self $(,$args : $args_ty)*) $(-> $ret_ty)? {
                        let _val = self.as_mut().map(move |v| v.$visitor($($args),*));
                        $(${ignore($ret_ty)} _val)?
                    }
                )*
            }

        )*
    }
}

make_visitor! {
    |this|
    pub trait FileVisitor (visit_file){
        |file: File| {
            this.visit_header();
            for doc in &file.file_doc{
                this.visit_file_doc(doc);
            }
            for item in &file.items {
                visit_item(this.visit_item(), item);
            }
            this.visit_footer();
        }
        fn visit_header(&mut self);
        fn visit_file_doc(&mut self, doc: &str);
        fn visit_item(&mut self) -> impl ItemVisitor+'_;
        fn visit_footer(&mut self);
    }

    pub trait ItemVisitor (visit_item) {
        |item: Item| {
            for doc in &item.doc {
                this.visit_doc_line(doc);
            }

            match &item.body {
                ItemBody::Directive(dir)=>this.visit_directive(dir),
                ItemBody::Fn(item_fn) => visit_fn_item(this.visit_fn(), item_fn),
                ItemBody::Const(item_const) => visit_const_item(this.visit_const(), item_const),
                ItemBody::Use(item_use) => this.visit_use(&item_use.path.components),
                ItemBody::Type(item_type_alias) => visit_type_alias(this.visit_type_alias(), item_type_alias),
                ItemBody::Struct(item_structy) => visit_struct(this.visit_struct(), item_structy),
            }
        }
        fn visit_doc_line(&mut self, doc: &str);
        fn visit_use(&mut self, path: &[impl AsRef<str>]);
        fn visit_directive(&mut self, dir: &str);
        fn visit_type_alias(&mut self) -> impl TypeAliasVisitor + '_;
        fn visit_struct(&mut self) -> impl StructVisitor + '_;
        fn visit_const(&mut self) -> impl ConstItemVisitor + '_;
        fn visit_fn(&mut self) -> impl FnItemVisitor + '_;
    }

    pub trait ConstItemVisitor (visit_const_item) {
        |citem: ItemConst| {
            this.visit_name(&citem.name);
            visit_type(this.visit_ty(), &citem.ty);
            visit_expr(this.visit_value(), &citem.value);
        }
        fn visit_name(&mut self, name: &str);
        fn visit_ty(&mut self) -> impl TypeVisitor + '_;
        fn visit_value(&mut self) -> impl ExprVisitor + '_;
    }

    pub trait FnItemVisitor  (visit_fn_item) {
        |fn_item: ItemFn| {
            this.visit_name(&fn_item.name);
            visit_signature(this.visit_sig(), &fn_item.params);
            visit_expr(this.visit_sysno(), &fn_item.sysno);
        }
        fn visit_name(&mut self, name: &str);
        fn visit_sig(&mut self) -> impl SignatureVisitor + '_;
        fn visit_sysno(&mut self) -> impl ExprVisitor + '_;
    }

    pub trait TypeAliasVisitor (visit_type_alias){
        |alias: ItemTypeAlias | {
            this.visit_name(&alias.name);
            visit_type(this.visit_def(), &alias.def)
        }
        fn visit_name(&mut self, name: &str);
        fn visit_def(&mut self) -> impl TypeVisitor + '_;
    }

    pub trait StructVisitor (visit_struct) {
        |structy: ItemStructy| {
            this.visit_kind(structy.kind);
            this.visit_name(&structy.name);
            for name in &structy.generics {
                this.visit_generic_arg(name)
            }
            for property in &structy.properties{
                visit_struct_option(this.visit_option(), property)
            }
            match &structy.body {
                StructBody::Opaque(opaque_body) => visit_opaque(this.visit_opaque(), opaque_body),
                StructBody::Fields(fields) => visit_struct_body(this.visit_body(), fields),
            }
        }
        fn visit_kind(&mut self, kind: StructKind);
        fn visit_name(&mut self, name: &str);
        fn visit_generic_arg(&mut self, name: &str);
        fn visit_option(&mut self) -> impl StructOptionVisitor + '_;
        fn visit_body(&mut self) -> impl StructBodyVisitor + '_;
        fn visit_opaque(&mut self) -> impl OpaqueVisitor + '_;
    }

    pub trait OpaqueVisitor (visit_opaque){
        |opaque: OpaqueBody| {
            if let Some(ty) = &opaque.0 {
                visit_type(this.visit_internal_type(),ty);
            }
        }
        fn visit_internal_type(&mut self) -> impl TypeVisitor + '_;
    }

    pub trait StructOptionVisitor (visit_struct_option){
        |option: StructProperties| {
            match option {
                StructProperties::Align(align) => visit_expr(this.visit_align(), align),
                StructProperties::OptionBody(body) => for item in body {
                    visit_expr(this.visit_option_body_pad(), item)
                },
                StructProperties::Option(id) => visit_expr(this.visit_option_id(), id),
            }
        }
        fn visit_align(&mut self) -> impl ExprVisitor + '_;
        fn visit_option_body_pad(&mut self) -> impl ExprVisitor + '_;
        fn visit_option_id(&mut self) -> impl ExprVisitor + '_;
    }

    pub trait StructBodyVisitor (visit_struct_body){
        |fields: StructBodyFields| {
            for field in &fields.fields {
                visit_field(this.visit_field(), field);
            }
            match &fields.padding {
                Some(Padding::Pad(pad)) => visit_type(this.visit_pad(), pad),
                Some(Padding::PadTo(pad_to)) => visit_expr(this.visit_pad_to(), pad_to),
                None => {}
            }
        }
        fn visit_field(&mut self) -> impl FieldVisitor + '_;
        fn visit_pad(&mut self) -> impl TypeVisitor + '_;
        fn visit_pad_to(&mut self) -> impl ExprVisitor + '_;
    }

    pub trait FieldVisitor (visit_field){
        |field: StructField| {
            for doc in &field.doc {
                this.visit_doc_line(doc);
            }
            this.visit_name(&field.name);
            visit_type(this.visit_ty(), &field.ty);
        }
        fn visit_doc_line(&mut self, st: &str);
        fn visit_name(&mut self, n: &str);
        fn visit_ty(&mut self) -> impl TypeVisitor + '_;
    }

    pub trait ParamTypeVisitor (visit_param_type) {
        |ty: Type| {
            match ty {
                Type::Integer(int_type) => visit_int_type(this.visit_int(), int_type),
                Type::Char => this.visit_char(),
                Type::Fn(fn_signature) => visit_signature(this.visit_fn_pointer(), fn_signature),
                Type::Pointer(pointer_type) => visit_pointer_type(this.visit_pointer(), pointer_type),
                Type::Named(named_type) => visit_named_type(this.visit_named(), named_type),
                ty @ (Type::Byte |
                Type::Array(_) |
                Type::Void |
                Type::Never) => panic!("Invalid type for position: {ty:?}"),
            }
        }
        fn visit_char(&mut self);
        fn visit_int(&mut self) -> impl IntTypeVisitor+'_;
        fn visit_named(&mut self) -> impl NamedTypeVisitor+'_;
        fn visit_fn_pointer(&mut self) -> impl SignatureVisitor+'_;
        fn visit_pointer(&mut self) -> impl PointerTypeVisitor+'_;
    }

    pub trait TypeVisitor (visit_type): ParamTypeVisitor {
        |ty: Type| {
            match ty {
                Type::Never => panic!("invalid type for position: `!`"),
                Type::Void => this.visit_void(),
                Type::Byte => this.visit_byte(),
                Type::Array(arr) => visit_array_type(this.visit_array(), arr),
                ty => visit_param_type(this, ty)
            }
        }
        fn visit_void(&mut self);
        fn visit_byte(&mut self);
        fn visit_array(&mut self) -> impl ArrayTypeVisitor + '_;
    }
    pub trait ReturnTypeVisitor (visit_return_type): ParamTypeVisitor {
        |ty: Type| {
            match ty {
                ty @ (Type::Byte |
                Type::Array(_)) => panic!("Invalid type for position: {ty:?}"),
                Type::Void => this.visit_void(),
                Type::Never => this.visit_never(),
                ty => visit_param_type(this, ty),
            }
        }
        fn visit_never(&mut self);
        fn visit_void(&mut self);
    }
    pub trait NamedTypeVisitor (visit_named_type){
        |name: NamedType| {
            this.visit_name(&name.0);

            match &name.1 {
                Some(NameSuffix::Generics(generics)) => {
                    for arg in generics {
                        visit_type(this.visit_arg(), arg);
                    }
                }
                Some(NameSuffix::ParamReplace(inner)) => {
                    visit_type(this.visit_replace(), inner)
                }
                None => {}
            }
        }
        fn visit_name(&mut self, name: &str);
        fn visit_arg(&mut self) -> impl TypeVisitor+'_;
        fn visit_replace(&mut self) -> impl TypeVisitor+'_;
    }
    pub trait IntTypeVisitor (visit_int_type){
        |intty: IntType| {
            this.visit_signedness(intty.signed);
            this.visit_width(intty.width);
        }
        fn visit_signedness(&mut self, signed: bool);
        fn visit_width(&mut self, width: IntWidth);
    }
    pub trait SignatureVisitor (visit_signature){
        |sig: FnSignature|{
            for arg in &sig.params{
                visit_fn_param(this.visit_arg(), arg);
            }
            visit_return_type(this.visit_return(), &sig.ret_ty);
        }
        fn visit_arg(&mut self) -> impl FnParamVisitor+'_;
        fn visit_return(&mut self) -> impl ReturnTypeVisitor + '_;
    }

    pub trait FnParamVisitor (visit_fn_param){
        |param: FnParam| {
            if let Some(name) = &param.name {
                this.visit_name(name)
            }
            visit_param_type(this.visit_type(), &param.ty);
        }
        fn visit_name(&mut self, name: &str);
        fn visit_type(&mut self) -> impl ParamTypeVisitor+'_;
    }

    pub trait PointerTypeVisitor (visit_pointer_type){
        |ptr: PointerType|{
            this.visit_pointer_kind(ptr.0);

            visit_type(this.visit_pointee(), &ptr.1)
        }
        fn visit_pointer_kind(&mut self, kind: PointerKind);
        fn visit_pointee(&mut self) -> impl TypeVisitor + '_;
    }

    pub trait ArrayTypeVisitor (visit_array_type){
        |arr: ArrayType| {
            visit_type(this.visit_elem(), &arr.0);
            visit_expr(this.visit_len(), &arr.1);
        }
        fn visit_elem(&mut self) -> impl TypeVisitor + '_;
        fn visit_len(&mut self) -> impl ExprVisitor + '_;
    }

    pub trait ExprVisitor (visit_expr){
        |expr: Expression|{
            match expr {
                Expression::Id(name) => this.visit_constant(name),
                Expression::Integer(lit) => this.visit_int_literal(lit),
                Expression::Binary(binary_expr) => visit_binary_expr(this.visit_binary_expr(), binary_expr),
                Expression::Unary(unary_expr) => visit_unary_expr(this.visit_unary_expr(), unary_expr),
                Expression::UuidLit(uuid) => this.visit_uuid_literal(*uuid),
            }
        }
        fn visit_int_literal(&mut self, val: &str);
        fn visit_uuid_literal(&mut self, uuid: Uuid);
        fn visit_constant(&mut self, name: &str);
        fn visit_unary_expr(&mut self) -> impl UnaryExprVisitor + '_;
        fn visit_binary_expr(&mut self) -> impl BinaryExprVisitor + '_;
    }

    pub trait UnaryExprVisitor (visit_unary_expr){
        |unary: UnaryExpr|{
            this.visit_op(unary.0);
            visit_expr(this.visit_inner(), &unary.1);
        }
        fn visit_op(&mut self, op: UnaryOp);
        fn visit_inner(&mut self) -> impl ExprVisitor + '_;
    }
    pub trait BinaryExprVisitor (visit_binary_expr){
        |binary: BinaryExpr|{
            this.visit_op(binary.0);
            visit_expr(this.visit_left(), &binary.1);
            visit_expr(this.visit_right(), &binary.2);
        }
        fn visit_op(&mut self, op: BinaryOp);
        fn visit_left(&mut self) -> impl ExprVisitor + '_;
        fn visit_right(&mut self) -> impl ExprVisitor + '_;
    }
}

/// Parses a given integer literal into a u64 (the largest type that knums supports)
/// panics if `lit` is not a valid integer literal (see [`Token::IntLiteral`][crate::lexer::Token::IntLiteral])
pub fn parse_int_literal(lit: &str) -> u64 {
    let mut n = 0u64;
    let (radix, body) = match lit {
        lit if lit.starts_with("0x") || lit.starts_with("0X") => (16, &lit[2..]),
        lit if lit.starts_with("0o") || lit.starts_with("0O") => (8, &lit[2..]),
        lit => (10u32, lit),
    };

    for v in body.split("_") {
        if v.is_empty() {
            continue;
        }
        n *= (radix as u128).pow(v.len() as u32) as u64;
        n += u64::from_str_radix(v, radix).unwrap()
    }
    n
}
