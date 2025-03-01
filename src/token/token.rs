#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub enum TokenType {
    LET,
    FN,
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    STRING,
    ASSIGN,    // =
    PLUS,      // +
    MINUS,     // -
    ASTERISK,  // *
    SLASH,     // /
    COMMA,     // ,
    COLON,     // :
    SEMICOLON, // ;
    LPAREN,    // (
    RPAREN,    // )
    LBRACE,    // {
    RBRACE,    // }
    LBRACKET,  // [
    RBRACKET,  // ]
    LT,        // <
    GT,        // >
    EQ,        // ==
    NOTEQ,     // !=
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
    BANG,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
