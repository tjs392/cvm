

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    
    // literals
    IntLiteral(i64),
    FloatLiteral(f64),        
    CharLiteral(char),        
    StringLiteral(String),    
    BoolLiteral(bool),
    Ident(String),

    // types
    Int,
    Char,        
    Short,       
    Long,        
    Float,       
    Double,      
    Void,
    Bool,
    Signed,      
    Unsigned,    

    // declarations
    Struct,
    Union,       
    Enum,        
    Typedef,     
    Const,       
    Static,      
    Extern,      
    Sizeof,      

    // control flows
    Return,
    If,
    Else,
    While,
    Do,          
    For,
    Switch,      
    Case,        
    Default,     
    Break,
    Continue,
    Goto,        
    Null,

    // arithmetic
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    PlusPlus,    // ++
    MinusMinus,  // --

    // comparisons
    Eq,          // ==
    NotEq,       // !=
    Lt,          // 
    Gt,          // >
    Le,          // <=
    Ge,          // >=

    // logicals
    And,         // &&
    Or,          // ||
    Not,         // !

    // bitwise
    Ampersand,   // &
    Pipe,        // |
    Tilde,       // ~
    Caret,       // ^
    LShift,      // 
    RShift,      // >>

    // assignments
    Assign,          // =
    PlusAssign,      // +=
    MinusAssign,     // -=
    StarAssign,      // *=
    SlashAssign,     // /=
    PercentAssign,   // %=   
    AndAssign,       // &=   
    OrAssign,        // |=   
    XorAssign,       // ^=   
    LShiftAssign,    // <<=  
    RShiftAssign,    // >>=  

    // delims
    LParen,      // (
    RParen,      // )
    LBrace,      // {
    RBrace,      // }
    LBracket,    // [   
    RBracket,    // ]   
    Semicolon,   // ;
    Comma,       // ,
    Dot,         // .
    Arrow,       // -> 
    Question,    // ?   
    Colon,       // :

    EOF,
}

// lexer / tokenizer
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while self.pos < self.input.len() {
            while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                self.advance();
            }

            let ch = match self.peek() {
                Some(c) => c,
                None => break,
            };

            // check for comments
            if ch == '/' && self.input.get(self.pos + 1) == Some(&'/') {
                while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                    self.advance();
                }
                continue;
            }

            // check for long comments /*  */
            if ch == '/' && self.input.get(self.pos + 1) == Some(&'*') {
                self.advance();
                self.advance();
                while self.pos < self.input.len() {
                    if self.peek() == Some('*') && self.input.get(self.pos + 1) == Some(&'/') {
                        self.advance();
                        self.advance();
                        break;
                    }
                    self.advance();
                }
                continue;
            }

            // check for numbers (decimal, hex, octal, floats)
            if ch.is_ascii_digit() || (ch == '.' && self.input.get(self.pos + 1).map_or(false, |c| c.is_ascii_digit())) {
                // hex: 0x or 0X
                if ch == '0' && self.input.get(self.pos + 1).map_or(false, |&c| c == 'x' || c == 'X') {
                    self.advance();
                    self.advance();
                    let mut hex = String::new();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_hexdigit() {
                            hex.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    if hex.is_empty() {
                        panic!("Invalid hex literal");
                    }
                    tokens.push(Token::IntLiteral(i64::from_str_radix(&hex, 16).unwrap()));
                    continue;
                }
                
                // octal: starts with 0 and followed by digits
                if ch == '0' && self.input.get(self.pos + 1).map_or(false, |c| c.is_ascii_digit()) {
                    self.advance();
                    let mut octal = String::new();
                    while let Some(c) = self.peek() {
                        if c >= '0' && c <= '7' {
                            octal.push(c);
                            self.advance();
                        } else if c.is_ascii_digit() {
                            panic!("Invalid octal digit: {}", c);
                        } else {
                            break;
                        }
                    }
                    if octal.is_empty() {
                        tokens.push(Token::IntLiteral(0));
                    } else {
                        tokens.push(Token::IntLiteral(i64::from_str_radix(&octal, 8).unwrap()));
                    }
                    continue;
                }

                // decimal or float
                let mut num = String::new();
                let mut is_float = false;

                // integer part
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        num.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }

                // decimal point
                if self.peek() == Some('.') {
                    is_float = true;
                    num.push('.');
                    self.advance();

                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() {
                            num.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                // handle exponents (1e9, 1E9)
                if self.peek() == Some('e') || self.peek() == Some('E') {
                    is_float = true;
                    num.push('e');
                    self.advance();

                    if self.peek() == Some('+') || self.peek() == Some('-') {
                        num.push(self.peek().unwrap());
                        self.advance();
                    }

                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() {
                            num.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }

                // float suffix
                if self.peek() == Some('f') || self.peek() == Some('F') || self.peek() == Some('l') || self.peek() == Some('L') {
                    is_float = true;
                    self.advance();
                }
                    
                if is_float {
                    tokens.push(Token::FloatLiteral(num.parse().unwrap()));
                } else {
                    tokens.push(Token::IntLiteral(num.parse().unwrap()));
                }
                continue;
            }

            // checking for character lits
            // https://en.wikipedia.org/wiki/Escape_sequences_in_C
            if ch == '\'' {
                self.advance();
                let c = match self.peek() {
                    Some('\\') => {
                        self.advance();
                        match self.peek() {
                            Some('n') => { self.advance(); '\n' },
                            Some('t') => { self.advance(); '\t' },
                            Some('r') => { self.advance(); '\r' },
                            Some('\\') => { self.advance(); '\\' },
                            Some('\'') => { self.advance(); '\'' },
                            Some('\"') => { self.advance(); '\"' },
                            Some('0') => { self.advance(); '\0' },
                            Some('x') => {
                                // hex escap, \xFF
                                self.advance();
                                let mut hex = String::new();
                                while let Some(c) = self.peek() {
                                    if c.is_ascii_hexdigit() && hex.len() < 2 {
                                        hex.push(c);
                                        self.advance();
                                    } else {
                                        break;
                                    }
                                }
                                char::from_u32(u32::from_str_radix(&hex, 16).unwrap()).unwrap()
                            },
                            Some(d) if d.is_ascii_digit() => {
                                // octal escape, \077
                                let mut octal = String::new();
                                while let Some(c) = self.peek() {
                                    if c.is_ascii_digit() && octal.len() < 3 {
                                        octal.push(c);
                                        self.advance();
                                    } else {
                                        break;
                                    }
                                }
                                char::from_u32(u32::from_str_radix(&octal, 8).unwrap()).unwrap()
                            },
                            _ => panic!("Invalid escape sequence"),
                        }
                    },
                    Some(c) => {
                        let ch = c;
                        self.advance();
                        ch
                    },
                    None => panic!("Unterminated character literal"),
                };
                
                if self.peek() != Some('\'') {
                    panic!("Expected closing ' for character literal");
                }
                self.advance();
                tokens.push(Token::CharLiteral(c));
                continue;
            }

            // string literals
            if ch == '"' {
                self.advance();
                let mut s = String::new();

                while self.peek() != Some('"') && self.pos < self.input.len() {
                    match self.peek() {
                        Some('\\') => {
                            self.advance();
                            match self.peek() {
                                Some('n') => { s.push('\n'); self.advance(); },
                                Some('t') => { s.push('\t'); self.advance(); },
                                Some('r') => { s.push('\r'); self.advance(); },
                                Some('\\') => { s.push('\\'); self.advance(); },
                                Some('\'') => { s.push('\''); self.advance(); },
                                Some('\"') => { s.push('\"'); self.advance(); },
                                Some('0') => { s.push('\0'); self.advance(); },
                                Some('x') => {
                                    self.advance();
                                    let mut hex = String::new();
                                    while let Some(c) = self.peek() {
                                        if c.is_ascii_hexdigit() && hex.len() < 2 {
                                            hex.push(c);
                                            self.advance();
                                        } else {
                                            break;
                                        }
                                    }
                                    let val = u32::from_str_radix(&hex, 16).unwrap();
                                    s.push(char::from_u32(val).unwrap());
                                },
                                _ => panic!("Invalid escape sequence in string"),
                            }
                        },
                        Some(c) => {
                            s.push(c);
                            self.advance();
                        },
                        None => panic!("Unterminated string literal"),
                    }
                }
                if self.peek() != Some('"') {
                    panic!("Expected closing \" for string literal");
                }
                self.advance();
                tokens.push(Token::StringLiteral(s));
                continue;
            }

            // check identifier
            if ch.is_alphabetic() || ch == '_' {
                let mut word = String::new();
                while let Some(c) = self.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        word.push(c);
                        self.advance();
                    } else {
                        break;
                    }
                }

                let token = match word.as_str() {
                    //types
                    "int"      => Token::Int,
                    "char"     => Token::Char,    
                    "short"    => Token::Short,   
                    "long"     => Token::Long,    
                    "float"    => Token::Float,   
                    "double"   => Token::Double,  
                    "signed"   => Token::Signed,  
                    "unsigned" => Token::Unsigned,
                    "bool"     => Token::Bool,
                    "void"     => Token::Void,
                    
                    // decs
                    "struct"   => Token::Struct,
                    "union"    => Token::Union,   
                    "enum"     => Token::Enum,    
                    "typedef"  => Token::Typedef, 
                    "const"    => Token::Const,   
                    "static"   => Token::Static,  
                    "extern"   => Token::Extern,  
                    "sizeof"   => Token::Sizeof,  
                    
                    // control flows
                    "return"   => Token::Return,
                    "if"       => Token::If,
                    "else"     => Token::Else,
                    "while"    => Token::While,
                    "do"       => Token::Do,      
                    "for"      => Token::For,
                    "switch"   => Token::Switch,  
                    "case"     => Token::Case,    
                    "default"  => Token::Default, 
                    "break"    => Token::Break,
                    "continue" => Token::Continue,
                    "goto"     => Token::Goto,    
                    "null"     => Token::Null,
                    
                    // bools
                    "true"     => Token::BoolLiteral(true),
                    "false"    => Token::BoolLiteral(false),
                    
                    _          => Token::Ident(word),
                };

                tokens.push(token);
                continue;
            }

            // check for triple char tokens like <<= >>=
            if let Some(next) = self.input.get(self.pos + 1) {
                if let Some(next2) = self.input.get(self.pos + 2) {
                    let token = match (ch, next, next2) {
                        ('<', '<', '=') => Some(Token::LShiftAssign),
                        ('>', '>', '=') => Some(Token::RShiftAssign),
                        _ => None,
                    };
                    
                    if let Some(tok) = token {
                        self.advance();
                        self.advance();
                        self.advance();
                        tokens.push(tok);
                        continue;
                    }
                }
            } 

            // check double char tokens like == << && etc.
            if let Some(next) = self.input.get(self.pos + 1) {
                let token = match (ch, next) {
                    ('=', '=') => Some(Token::Eq),
                    ('!', '=') => Some(Token::NotEq),
                    ('<', '=') => Some(Token::Le),
                    ('>', '=') => Some(Token::Ge),
                    ('&', '&') => Some(Token::And),
                    ('|', '|') => Some(Token::Or),
                    ('<', '<') => Some(Token::LShift),
                    ('>', '>') => Some(Token::RShift),
                    ('+', '+') => Some(Token::PlusPlus),
                    ('-', '-') => Some(Token::MinusMinus),
                    ('+', '=') => Some(Token::PlusAssign),
                    ('-', '=') => Some(Token::MinusAssign),
                    ('*', '=') => Some(Token::StarAssign),
                    ('/', '=') => Some(Token::SlashAssign),
                    ('%', '=') => Some(Token::PercentAssign),   
                    ('&', '=') => Some(Token::AndAssign),       
                    ('|', '=') => Some(Token::OrAssign),        
                    ('^', '=') => Some(Token::XorAssign),       
                    ('-', '>') => Some(Token::Arrow),
                    _ => None,
                };

                if let Some(tok) = token {
                    self.advance();
                    self.advance();
                    tokens.push(tok);
                    continue;
                }
            }

            // all other tokens
            self.advance();
            let token = match ch {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,
                '%' => Token::Percent,
                '<' => Token::Lt,
                '>' => Token::Gt,
                '!' => Token::Not,
                '=' => Token::Assign,
                '&' => Token::Ampersand,
                '|' => Token::Pipe,
                '~' => Token::Tilde,
                '^' => Token::Caret,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                ';' => Token::Semicolon,
                ',' => Token::Comma,
                '.' => Token::Dot,
                '[' => Token::LBracket,
                ']' => Token::RBracket,
                '?' => Token::Question,
                ':' => Token::Colon,
                _ => panic!("Unexpected character: {}", ch),
            };
            tokens.push(token);
        }
        tokens.push(Token::EOF);
        tokens
    }
}