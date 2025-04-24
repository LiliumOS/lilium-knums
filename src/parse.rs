use crate::{
    ast::{
        File,
        expr::{BinaryOp, Expression, UnaryOp},
        item::{
            Item, ItemBody, ItemConst, ItemFn, ItemTypeAlias, ItemUse, Padding, Path, StructBody,
            StructBodyFields, StructField, StructKind, StructProperties,
        },
        ty::{FnParam, FnSignature, IntType, IntWidth, PointerKind, Type},
    },
    lexer::Token,
};
use core::iter::Peekable;

use crate::nzlit::nzlit;

fn check_exact<I: Iterator<Item = Token>>(iter: &mut Peekable<I>, tok: Token) {
    match iter.next() {
        Some(val) if val == tok => {}
        Some(val) => panic!("Expected {tok:?} got {val:?}"),
        None => panic!("Expected {tok:?} got EOF"),
    }
}

fn check_ident<I: Iterator<Item = Token>>(iter: &mut Peekable<I>) -> String {
    match iter.next() {
        Some(Token::Ident(id)) => id,
        Some(tok) => panic!("Expected an identifier, got {tok:?}"),
        None => panic!("Expected an identifier"),
    }
}

pub fn parse_simple_expr<I: Iterator<Item = Token>>(iter: &mut Peekable<I>) -> Expression {
    match iter.next().unwrap() {
        Token::OpenParen => {
            let expr = parse_expr(iter);
            check_exact(iter, Token::CloseParen);
            expr
        }
        Token::Ident(id) => Expression::Id(id),
        Token::IntLit(id) => Expression::Integer(id),
        Token::Not => {
            let nested = parse_simple_expr(iter);
            Expression::Unary(UnaryOp::Not, Box::new(nested))
        }
        Token::Sub => {
            let nested = parse_simple_expr(iter);
            Expression::Unary(UnaryOp::Neg, Box::new(nested))
        }
        Token::Add => {
            let nested = parse_simple_expr(iter);
            Expression::Unary(UnaryOp::Plus, Box::new(nested))
        }
        Token::Uuid(uuid) => Expression::UuidLit(uuid),
        tok => panic!("Expected an expression, got {tok:?}"),
    }
}

fn pratt(t: &Token) -> Option<(BinaryOp, u32)> {
    match t {
        Token::Add => Some((BinaryOp::Add, 1)),
        Token::Sub => Some((BinaryOp::Sub, 1)),
        Token::Star => Some((BinaryOp::Mul, 2)),
        Token::BitAnd => Some((BinaryOp::BitAnd, 3)),
        Token::BitOr => Some((BinaryOp::BitOr, 3)),
        Token::BitXor => Some((BinaryOp::BitXor, 3)),
        Token::ShiftLeft => Some((BinaryOp::ShiftLeft, 4)),
        Token::ShiftRight => Some((BinaryOp::ShiftRight, 4)),
        _ => None,
    }
}

pub fn parse_binary_expr<I: Iterator<Item = Token>>(
    iter: &mut Peekable<I>,
    precedence: u32,
) -> Expression {
    let mut base = parse_simple_expr(iter);
    loop {
        let Some(tok) = iter.peek() else { break };

        let Some((op, power)) = pratt(tok) else { break };
        iter.next();

        let lbp = power * 2;
        let rbp = power * 2 + 1;

        if lbp < precedence {
            break;
        }

        let second = parse_binary_expr(iter, rbp);

        base = Expression::Binary(op, Box::new(base), Box::new(second));
    }
    base
}

pub fn parse_expr<I: Iterator<Item = Token>>(iter: &mut Peekable<I>) -> Expression {
    parse_binary_expr(iter, 0)
}

fn id_to_ty(id: String) -> Type {
    match &*id {
        "void" => Type::Void,
        "char" => Type::Char,
        x if x.starts_with('u') || x.starts_with('i') => {
            let signed = x.starts_with('i');
            let width = match &x[1..] {
                "long" => IntWidth::Long,
                "8" => IntWidth::Bits(nzlit!(8)),
                "16" => IntWidth::Bits(nzlit!(16)),
                "32" => IntWidth::Bits(nzlit!(32)),
                "64" => IntWidth::Bits(nzlit!(64)),
                _ => panic!("Expected a valid integer type, got {x}"),
            };

            let intty = IntType { signed, width };

            Type::Integer(intty)
        }
        _ => Type::Named(id),
    }
}

pub fn parse_type<I: Iterator<Item = Token>>(iter: &mut Peekable<I>) -> Type {
    match iter.next().unwrap() {
        Token::Ident(id) => id_to_ty(id),
        Token::OpenBracket => {
            let inner = parse_type(iter);
            check_exact(iter, Token::Semi);
            let len = parse_expr(iter);
            check_exact(iter, Token::CloseBracket);

            Type::Array(Box::new(inner), len)
        }
        Token::Fn => {
            let sig = parse_fn_sig(iter);
            Type::Fn(sig)
        }
        Token::Star => {
            let kind = match iter.next().unwrap() {
                Token::Const => PointerKind::Const,
                Token::Mut => PointerKind::Mut,
                Token::Handle => PointerKind::Handle,
                Token::SharedHandle => PointerKind::Shared,
                tok => panic!(
                    "Expected one of `const`, `mut`, `handle`, or `shared_handle`, got {tok:?}"
                ),
            };

            let inner = parse_type(iter);

            Type::Pointer(kind, Box::new(inner))
        }
        Token::Not => Type::Never,
        tok => panic!("Expected a type, got {tok:?}"),
    }
}

pub fn parse_fn_sig<I: Iterator<Item = Token>>(iter: &mut Peekable<I>) -> FnSignature {
    check_exact(iter, Token::OpenParen);
    let mut params = Vec::new();

    loop {
        let (name, ty) = match iter.peek().unwrap() {
            Token::CloseParen => {
                iter.next();
                break;
            }
            Token::Ident(_) => {
                let id = check_ident(iter);
                match iter.peek().unwrap() {
                    Token::Colon => {
                        iter.next();

                        let ty = parse_type(iter);

                        (Some(id), ty)
                    }
                    _ => (None, id_to_ty(id)),
                }
            }
            _ => (None, parse_type(iter)),
        };
        params.push(FnParam { name, ty });
        match iter.next().unwrap() {
            Token::CloseParen => break,
            Token::Comma => continue,
            tok => panic!("Expected `,`, got {tok:?}"),
        }
    }

    check_exact(iter, Token::Arrow);

    let ret_ty = parse_type(iter);

    FnSignature {
        params,
        ret_ty: Box::new(ret_ty),
    }
}

pub fn parse_item<I: Iterator<Item = Token>>(iter: &mut Peekable<I>) -> Item {
    let mut doc = Vec::new();

    let body = loop {
        match iter.next().unwrap() {
            Token::DocString(c) => doc.push(c),
            Token::Const => {
                let name = check_ident(iter);
                check_exact(iter, Token::Colon);
                let ty = parse_type(iter);
                check_exact(iter, Token::Equal);
                let value = parse_expr(iter);
                check_exact(iter, Token::Semi);
                break ItemBody::Const(ItemConst { ty, name, value });
            }
            Token::Ident(id) if id == "inline" => {
                check_exact(iter, Token::Use);
                let mut components = Vec::new();

                break loop {
                    components.push(check_ident(iter));
                    match iter.next().unwrap() {
                        Token::ColonColon => continue,
                        Token::Semi => {
                            break ItemBody::Use(ItemUse {
                                inline: true,
                                path: Path {
                                    has_leading: false,
                                    components,
                                },
                            });
                        }
                        tok => panic!("Expected a `;` or `::`, got {tok:?}"),
                    }
                };
            }
            Token::Use => {
                let mut components = Vec::new();

                break loop {
                    components.push(check_ident(iter));
                    match iter.next().unwrap() {
                        Token::ColonColon => continue,
                        Token::Semi => {
                            break ItemBody::Use(ItemUse {
                                inline: false,
                                path: Path {
                                    has_leading: false,
                                    components,
                                },
                            });
                        }
                        tok => panic!("Expected a `;` or `::`, got {tok:?}"),
                    }
                };
            }
            Token::Type => {
                let name = check_ident(iter);
                check_exact(iter, Token::Equal);
                let def = parse_type(iter);
                check_exact(iter, Token::Semi);

                break ItemBody::Type(ItemTypeAlias { name, def });
            }
            x @ (Token::Struct | Token::Union) => {
                let kind = match x {
                    Token::Struct => StructKind::Struct,
                    Token::Union => StructKind::Union,
                    _ => unreachable!(),
                };
                let name = check_ident(iter);
                let mut generics = Vec::new();
                match iter.peek().unwrap() {
                    Token::OpenAngle => {
                        iter.next();
                        loop {
                            match iter.next().unwrap() {
                                Token::Ident(id) => generics.push(id),
                                Token::CloseAngle => break,
                                tok => panic!("Expected an identifier or `>`, got {tok:?}"),
                            }

                            match iter.next().unwrap() {
                                Token::Comma => continue,
                                Token::CloseAngle => break,
                                tok => panic!("Expected `>` or `,`, got {tok:?}"),
                            }
                        }
                    }
                    _ => {}
                }

                let mut properties = Vec::new();
                let is_opaque = match iter.next().unwrap() {
                    Token::Colon => loop {
                        match iter.next().unwrap() {
                            Token::Ident(id) if id == "align" => {
                                check_exact(iter, Token::OpenParen);
                                let expr = parse_expr(iter);
                                check_exact(iter, Token::CloseParen);
                                properties.push(StructProperties::Align(expr))
                            }
                            Token::Ident(id) if id == "opaque" => break true,
                            Token::OpenBrace => break false,
                            x => panic!("Expected `align`, got `{x:?}`"),
                        }

                        match iter.next().unwrap() {
                            Token::Comma => continue,
                            Token::OpenBrace => break false,
                            Token::Ident(id) if id == "opaque" => break true,
                            tok => panic!("Expected `,` got {tok:?}"),
                        }
                    },
                    Token::OpenBrace => false,
                    Token::Ident(id) if id == "opaque" => true,
                    tok => panic!("Expected `:` or a struct body, got {tok:?}"),
                };

                let body = if is_opaque {
                    let ty = match iter.next().unwrap() {
                        Token::OpenParen => {
                            let ty = parse_type(iter);
                            check_exact(iter, Token::CloseParen);
                            check_exact(iter, Token::Semi);
                            Some(ty)
                        }
                        Token::Semi => None,
                        tok => panic!("Expected an opaque body, got {tok:?}"),
                    };
                    StructBody::Opaque(ty)
                } else {
                    let mut fields = Vec::new();
                    let padding = loop {
                        let id = match iter.next().unwrap() {
                            Token::Ident(id) => id,
                            Token::CloseBrace => break None,
                            tok => panic!("Expected an identifier, got {tok:?}"),
                        };

                        match iter.next().unwrap() {
                            Token::CloseBrace => break None,
                            Token::Colon => {
                                let ty = parse_type(iter);
                                fields.push(StructField { name: id, ty });
                                match iter.next().unwrap() {
                                    Token::Comma => continue,
                                    Token::CloseBrace => break None,
                                    tok => panic!("Expected `,` or `}}`, got {tok:?}"),
                                }
                            }
                            Token::OpenParen => {
                                let pad = match &*id {
                                    "pad" => {
                                        let ty = parse_type(iter);

                                        Padding::Pad(ty)
                                    }
                                    "padto" => {
                                        let expr = parse_expr(iter);
                                        Padding::PadTo(expr)
                                    }
                                    x => panic!("Expected `pad` or `padto`, got {x}"),
                                };
                                check_exact(iter, Token::CloseParen);
                                check_exact(iter, Token::CloseBrace);
                                break Some(pad);
                            }
                            tok => panic!("Expected a struct field, got {tok:?}"),
                        }
                    };
                    StructBody::Fields(StructBodyFields { fields, padding })
                };

                break ItemBody::Struct(crate::ast::item::ItemStructy {
                    kind,
                    name,
                    generics,
                    properties,
                    body,
                });
            }
            Token::Fn => {
                let name = check_ident(iter);

                let params = parse_fn_sig(iter);

                check_exact(iter, Token::Equal);

                let sysno = parse_expr(iter);

                check_exact(iter, Token::Semi);

                break ItemBody::Fn(ItemFn {
                    name,
                    params,
                    sysno,
                });
            }
            Token::Directive(dir) => break ItemBody::Directive(dir),
            tok => panic!("Expected an item, got {tok:?}"),
        }
    };
    Item { doc, body }
}

pub fn parse_file<I: Iterator<Item = Token>>(iter: &mut Peekable<I>) -> File {
    let mut file_doc = Vec::new();
    let mut items = Vec::new();
    while let Some(Token::InnerDoc(n)) = iter.peek() {
        file_doc.push(n.clone());
        iter.next();
    }

    while let Some(_) = iter.peek() {
        items.push(parse_item(iter));
    }

    File { file_doc, items }
}
