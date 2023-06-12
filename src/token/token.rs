use std::collections::HashMap;

pub struct Token {
    ttype: String,
    literal: String,
    keywords: HashMap<String, String>,
}

impl Token {
    pub const COMPILER: &'static str = "COMPILER";

    // general
    pub const ILLIGAL: &'static str = "ILLIGAL";
    pub const EOF: &'static str = "EOF";
    pub const ROOT: &'static str = "ROOT";

    // identifiers + literals
    pub const IDENT: &'static str = "IDENT";
    pub const INT: &'static str = "INT";
    pub const STRING: &'static str = "STRING";

    // operators
    pub const ASSIGN: &'static str = "=";
    pub const PLUS: &'static str = "+";
    pub const MINUS: &'static str = "-";
    pub const ASTERISK: &'static str = "*";
    pub const SLASH: &'static str = "/";
    pub const BANG: &'static str = "!";

    pub const LT: &'static str = "<";
    pub const GT: &'static str = ">";
    pub const EQ: &'static str = "==";
    pub const NOT_EQ: &'static str = "!=";

    // sections
    pub const SEC_DECL: &'static str = "--";

    // Delimiters
    pub const COMMA: &'static str = ",";
    pub const SEMICOLON: &'static str = ";";
    pub const COLON: &'static str = ":";
    pub const LPAREN: &'static str = "(";
    pub const RPAREN: &'static str = ")";
    pub const LBRACE: &'static str = "{";
    pub const RBRACE: &'static str = "}";
    pub const LBRACKET: &'static str = "[";
    pub const RBRACKET: &'static str = "]";

    // Keywords
    pub const FUNC: &'static str = "FUNCTION";
    pub const LET: &'static str = "LET";
    pub const TRUE: &'static str = "TRUE";
    pub const FALSE: &'static str = "FALSE";
    pub const IF: &'static str = "IF";
    pub const ELSE: &'static str = "ELSE";
    pub const RETURN: &'static str = "RETURN";
    pub const CONST: &'static str = "CONST";

    pub fn new(_ttype: String, _literal: String) -> Token {
        let mut _keywords = [
            ("fn".to_string(), String::from(Token::FUNC)),
            ("let".to_string(), String::from(Token::LET)),
            ("true".to_string(), String::from(Token::TRUE)),
            ("false".to_string(), String::from(Token::FALSE)),
            ("if".to_string(), String::from(Token::IF)),
            ("else".to_string(), String::from(Token::ELSE)),
            ("return".to_string(), String::from(Token::RETURN)),
            ("const".to_string(), String::from(Token::CONST)),
        ].iter().cloned().collect();
        Token {
            keywords: _keywords,
            ttype: _ttype,
            literal: _literal,
        }
    }

    pub fn rebirth(&mut self, _ttype: String, _literal: String) {
        self.ttype = _ttype.clone();
        self.literal = _literal.clone();
    }

    pub fn look_up_ident(&self, ident: String) -> String {
        if let Some(value) = self.keywords.get(&ident) {
            return value.clone();
        };
        ident
    }

    pub fn show(&self) -> String {
        format!("[TYPE]: {} [LITERAL]: {}", self.ttype, self.literal)
    }

    pub fn get_type(&self) -> String {
        return format!("{}", self.ttype);
    }

    pub fn get_literal(&self) -> String {
        return format!("{}", self.literal);
    }

    pub fn clone(&mut self) -> Token {
        Token::new(
            self.ttype.clone(),
            self.literal.clone()
        )
    }
}



