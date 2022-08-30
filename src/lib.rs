use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'a> {
    // Arithmetic
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("++")]
    Mul,
    #[token("--")]
    Div,
    #[token("+++")]
    Pow,
    #[token("---")]
    Mod,

    // Comparison
    #[token(">:")]
    GE,
    #[token(">")]
    GT,
    #[token("<:")]
    LE,
    #[token("<")]
    LT,
    #[token("::")]
    Eq,
    #[token(":::")]
    NE,

    // Logical and Membership
    #[token("&&")]
    And,
    #[token("->?")]
    In,
    #[token("~~")]
    Not,
    #[token("||")]
    Or,
    #[token("^^")]
    Xor,

    // Bitwise
    #[token("&")]
    BAnd,
    #[token("~")]
    BNot,
    #[token("|")]
    BOr,
    #[token("^")]
    BXor,

    // Parens, Brackets and Braces
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,

    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,

    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,

    #[token("{{")]
    TableOpen,
    #[token("}}")]
    TableClose,

    // Control Flow
    #[token("!!")]
    Catch,
    #[token(",,")]
    Else,
    #[token("...")]
    For,
    #[token("<-")]
    From,
    #[token("?")]
    If,
    #[token("!!!")]
    Throw,
    #[token("->")]
    To,
    #[token("??")]
    Try,
    #[token("..")]
    While,

    // Comments
    #[token("==")]
    Comment,
    #[token("==<")]
    CommentOpen,
    #[token(">==")]
    CommentClose,

    // OOP / Functions
    #[token("@")]
    Class,
    #[token("<>")]
    Default,
    #[token("*")]
    Function,
    #[token("'")]
    Instance,
    #[token("=>")]
    Entry,
    #[token("**")]
    Yield,

    // Slicing
    #[token("<<")]
    SliceOpen,
    #[token(">>")]
    SliceClose,

    // Object Manipulation
    #[token("%")]
    Cast,
    #[token("$")]
    Special,
    #[token("=>!")]
    Exit,
    #[token("##")]
    Hash,
    #[token("!?")]
    Parent,
    #[token("?!")]
    Type,
    #[token("???")]
    Readline,
    #[token("!")]
    Print,

    // File I/O
    #[token("?~>")]
    FileCreate,
    #[token("&~~>")]
    FileAppend,
    #[token("<~~")]
    FileRead,
    #[token("~~>")]
    FileWrite,
    #[token("<~>")]
    FileReadWrite,
    #[token("&%~>")]
    FileBinaryAppend,
    #[token("<~%")]
    FileBinaryRead,
    #[token("%~>")]
    FileBinaryWrite,
    #[token("<%>")]
    FileBinaryReadWrite,
    #[token("&~>")]
    FileQuickAppend,
    #[token("<~")]
    FileQuickRead,
    #[token("~>")]
    FileQuickWrite,
    #[token("&%>")]
    FileQuickBinaryAppend,
    #[token("<%")]
    FileQuickBinaryRead,
    #[token("%>")]
    FileQuickBinaryWrite,

    // Other
    #[token("#")]
    Enum,
    #[token(":")]
    Assign,
    #[token(".")]
    Attr,
    #[token("@@")]
    UnixStmp,
    #[token("@@@")]
    ArrStmp,
    #[token(";")]
    End,
    #[token(",")]
    Sep,
    #[token(",.,")]
    Sleep,

    // Other
    #[regex(r"[\\/]+")]
    Integer(&'a str),
    #[regex(r#""((\\")|[^"])*""#)]
    String(&'a str),
    #[regex(r"[\w\d_]+", priority=1)]
    Indentifier(&'a str),
    #[regex(r"[\n\t ]+")]
    Whitespace,

    // Misc
    #[error]
    Error,
}
