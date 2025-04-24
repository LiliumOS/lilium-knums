use logos::Logos;
use logos::skip;

#[derive(Logos, Clone, Debug, Hash, PartialEq, Eq)]
#[logos(skip r"\p{White_Space}")]
pub enum Token {
    #[regex(r"%[[[:alpha:]]_][[[:alnum:]]_]*", |lex| lex.slice().to_owned())]
    Directive(String),
    #[regex(r"[\p{XID_Start}_][\p{XID_Continue}_]*", |lex|lex.slice().to_owned())]
    Ident(String),
    #[regex(r"0x[[[:xdigit:]]_]+", |lex| lex.slice().to_owned())]
    #[regex(r"0o[0-7_]+", |lex| lex.slice().to_owned())]
    #[regex(r"[[:digit:]][[[:digit:]]_]*", |lex| lex.slice().to_owned())]
    IntLit(String),
    #[regex(r"//![^\n]*\n", |lex|lex.slice()[3..].trim().to_owned(), priority=10)]
    InnerDoc(String),
    #[regex(r"///[^\n]*\n", |lex|lex.slice()[3..].trim().to_owned(), priority=10)]
    DocString(String),
    #[regex(r"U\{[[:xdigit:]]{8}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{12}\}", |lex|lex.slice().to_owned())]
    Uuid(String),
    #[regex(r"//[^\n]*\n", skip)]
    Comment,

    #[token("struct")]
    Struct,
    #[token("union")]
    Union,
    #[token("fn")]
    Fn,
    #[token("use")]
    Use,
    #[token("const")]
    Const,
    #[token("mut")]
    Mut,
    #[token("handle")]
    Handle,
    #[token("shared_handle")]
    SharedHandle,
    #[token("type")]
    Type,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
    #[token("::")]
    ColonColon,
    #[token(";")]
    Semi,
    #[token("<")]
    OpenAngle,
    #[token(">")]
    CloseAngle,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("->")]
    Arrow,
    #[token(":")]
    Colon,
    #[token("*")]
    Star,
    #[token(">>")]
    ShiftRight,
    #[token("<<")]
    ShiftLeft,
    #[token("=")]
    Equal,
    #[token(",")]
    Comma,
    #[token("&")]
    BitAnd,
    #[token("|")]
    BitOr,
    #[token("^")]
    BitXor,
    #[token("!")]
    Not,
}
