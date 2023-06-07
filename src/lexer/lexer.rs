use crate::token::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(s: String) -> Lexer {
        Lexer {
            input: s,
            position: 0,
            read_position: 0,
            ch: '\0',
        }
    }

    pub fn slice(s: String, l: usize, r : usize) -> String {
        let mut ans: String = "".to_string();
        for i in l..r {
            if let Some(_ch) = s.chars().nth(i) {
                ans.push(_ch);
            }
        }
        return ans;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            if let Some(_ch) = self.input.chars().nth(self.read_position) {
                self.ch = _ch;
            }
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\n' || self.ch == '\t' || self.ch == '\r' {
            self.read_char();
        };
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            if let Some(_ch) = self.input.chars().nth(self.read_position) {
                return _ch;
            } else {
                return '\0';
            }
        };
    }

    pub fn is_letter_char(c: char) -> bool {
        c.is_alphabetic()
    }

    pub fn is_digit_char(c: char) -> bool {
        c.is_digit(10)
    }

    pub fn read_identifier(&mut self) -> String {
        let mut _position = self.position;
        while Lexer::is_letter_char(self.ch) {
            self.read_char();
        }
        return Lexer::slice(self.input.clone(), _position, self.position);
    }

    pub fn read_number(&mut self) -> String {
        let mut _position = self.position;
        while Lexer::is_digit_char(self.ch) {
            self.read_char();
        }
        return Lexer::slice(self.input.clone(), _position, self.position);
    }

    pub fn read_string(&mut self) -> String {
        let mut _position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
        }
        return Lexer::slice(self.input.clone(), _position, self.position);
    }

    pub fn next_token(&mut self) -> Token {
        let mut t = Token::new("".to_string(), "".to_string());
        self.skip_whitespace();
        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let mut _ch = self.ch;
                    self.read_char();
                    t.rebirth(Token::EQ.to_string(), "==".to_string());
                } else {
                    t.rebirth(Token::ASSIGN.to_string(), self.ch.to_string());
                }
            }
            '+' => {
                t.rebirth(Token::PLUS.to_string(), self.ch.to_string());
            }
            '-' => {
                if self.peek_char() == '-' {
                    let mut _ch = self.ch;
                    self.read_char();
                    t.rebirth(Token::SEC_DECL.to_string(), "--".to_string());
                } else {
                    t.rebirth(Token::MINUS.to_string(), self.ch.to_string());
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    let mut _ch = self.ch;
                    self.read_char();
                    t.rebirth(Token::NOT_EQ.to_string(), "!=".to_string());
                } else {
                    t.rebirth(Token::BANG.to_string(), self.ch.to_string());
                }
            }
            '/' => {
                t.rebirth(Token::SLASH.to_string(), self.ch.to_string());
            }
            '*' => {
                t.rebirth(Token::ASTERISK.to_string(), self.ch.to_string());
            }
            '<' => {
                t.rebirth(Token::LT.to_string(), self.ch.to_string());
            }
            '>' => {
                t.rebirth(Token::GT.to_string(), self.ch.to_string());
            }
            ';' => {
                t.rebirth(Token::SEMICOLON.to_string(), self.ch.to_string());
            }
            ':' => {
                t.rebirth(Token::COLON.to_string(), self.ch.to_string());
            }
            ',' => {
                t.rebirth(Token::COMMA.to_string(), self.ch.to_string());
            }
            '{' => {
                t.rebirth(Token::LBRACE.to_string(), self.ch.to_string());
            }
            '}' => {
                t.rebirth(Token::RBRACE.to_string(), self.ch.to_string());
            }
            '(' => {
                t.rebirth(Token::LPAREN.to_string(), self.ch.to_string());
            }
            ')' => {
                t.rebirth(Token::RPAREN.to_string(), self.ch.to_string());
            }
            '"' => {
                t.rebirth(Token::STRING.to_string(), self.read_string());
            }
            '[' => {
                t.rebirth(Token::LBRACKET.to_string(), self.ch.to_string());
            }
            ']' => {
                t.rebirth(Token::RBRACKET.to_string(), self.ch.to_string());
            }
            '\0' => {
                t.rebirth(Token::EOF.to_string(), "\0".to_string());
            }
            _ => {
                if Lexer::is_letter_char(self.ch) {
                    let identifier = self.read_identifier();
                    t.rebirth(t.look_up_ident(identifier.clone()).to_string(), identifier);
                } else if Lexer::is_digit_char(self.ch) {
                    t.rebirth(Token::INT.to_string(), self.read_number());
                } else {
                    t.rebirth(Token::ILLIGAL.to_string(), self.ch.to_string());
                }
            }
        }
        self.read_char();
        return t;
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        let s = self.input.clone();
        let mut _lexer = Lexer::new(s.to_string());
        _lexer.next_token();
        let mut _tokens: Vec<Token> = vec![];
        while _tokens.len() == 0 || &_tokens[_tokens.len() - 1].get_type() != Token::EOF {
            _tokens.push(_lexer.next_token());
        }
        return _tokens;
    }
}




