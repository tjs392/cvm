use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Token {
        let token = self.advance();
        if &token != expected {
            panic!("Expected {:?}, got {:?}", expected, token);
        }
        token
    }

    pub fn parse_program(&mut self) -> Program {
        let mut structs = vec![];
        let mut functions = vec![];

        while *self.peek() != Token::EOF {
            match self.peek() {
                Token::Struct => structs.push(self.parse_struct()),
                _             => functions.push(self.parse_function())
            }
        }
        Program {
            structs,
            functions
        }
    }

    fn parse_struct(&mut self) -> StructDef {
        self.expect(&Token::Struct);
        
        let name = match self.advance() {
            Token::Ident(n) => n,
            other => panic!("Expected struct name, got {:?}", other),
        };

        self.expect(&Token::LBrace);

        let mut fields = vec![];
        while *self.peek() != Token::RBrace {
            let field_type = self.parse_type();
            let field_name = match self.advance() {
                Token::Ident(n) => n,
                other => panic!("Expected field name, got {:?}", other),
            };
            self.expect(&Token::Semicolon);
            fields.push((field_name, field_type));
        }

        self.expect(&Token::RBrace);
        
        StructDef {
            name,
            fields,
        }
    }

    fn parse_type(&mut self) -> Type {
        match self.advance() {
            Token::Int  => Type::Int,
            Token::Bool => Type::Bool,
            Token::Ident(name) => Type::Struct(name),
            other => panic!("Expected type, got {:?}", other),
        }
    }

    fn parse_function(&mut self) -> FunctionDef {
        let return_type = match self.peek() {
            Token::Null => {
                self.advance();
                None
            },
            _ => Some(self.parse_type()),
        };

        let name = match self.advance() {
            Token::Ident(n) => n,
            other => panic!("Expected function name, got {:?}", other),
        };

        self.expect(&Token::LParen);
        let mut params = vec![];
        while *self.peek() != Token::RParen {
            let param_type = self.parse_type();
            let param_name = match self.advance() {
                Token::Ident(n) => n,
                other => panic!("Expected parameter name, got {:?}", other),
            };
            params.push((param_name, param_type));

            if *self.peek() == Token::Comma {
                self.advance();
            }
        }
        self.expect(&Token::RParen);

        let body = self.parse_block();

        FunctionDef { 
            name, 
            params, 
            return_type, 
            body 
        }
    }

    fn parse_block(&mut self) -> Vec<Statement> {
        self.expect(&Token::LBrace);
        let mut statements = vec![];
        while *self.peek() != Token::RBrace {
            statements.push(self.parse_statement());
        }
        self.expect(&Token::RBrace);
        statements
    }

    fn parse_statement(&mut self) -> Statement {
        match self.peek() {
            Token::Int | Token::Bool | Token::Ident(_) => {
                let start_pos = self.pos;
                let first = self.advance();

                match self.peek() {
                    Token::Ident(_) => {
                        self.pos = start_pos;
                        self.parse_var_decl()
                    },

                    Token::Assign => {
                        let name = match first {
                            Token::Ident(n) => n,
                            _ => panic!("Expected identifier for assignment"),
                        };
                        self.advance();
                        let expr = self.parse_expression();
                        self.expect(&Token::Semicolon);

                        Statement::Assign(name, expr)
                    },
                    _ => {
                        self.pos = start_pos;
                        let expr = self.parse_expression();
                        self.expect(&Token::Semicolon);
                        
                        Statement::ExprStatement(expr)                    
                    }
                }
            },
            Token::Return => self.parse_return(),

            Token::If => self.parse_if(),

            Token::While => self.parse_while(),

            _ => panic!("Unexpected token in statement: {:?}", self.peek()),
            }
    }

    fn parse_var_decl(&mut self) -> Statement {
        let var_type = self.parse_type();
        let name = match self.advance() {
            Token::Ident(n) => n,
            other => panic!("Expected variable name, got {:?}", other),
        };
        self.expect(&Token::Assign);
        let expr = self.parse_expression();
        self.expect(&Token::Semicolon);

        Statement::VarDec(var_type, name, expr)
    }

    fn parse_return(&mut self) -> Statement {
        self.expect(&Token::Return);
        let expr = self.parse_expression();
        self.expect(&Token::Semicolon);

        Statement::Return(expr)
    }

    fn parse_if(&mut self) -> Statement {
        self.expect(&Token::If);
        self.expect(&Token::LParen);

        let condition = self.parse_expression();

        self.expect(&Token::RParen);

        let then_block = self.parse_block();

        let else_block = if *self.peek() == Token::Else {
            self.advance();
            Some(self.parse_block())
        } else {
            None
        };

        Statement::If(condition, then_block, else_block)
    }

    fn parse_while(&mut self) -> Statement {
        self.expect(&Token::While);
        self.expect(&Token::LParen);

        let condition = self.parse_expression();

        self.expect(&Token::RParen);

        let body = self.parse_block();

        Statement::While(condition, body)
    }

    // https://stackoverflow.com/questions/17369090/operator-precedence-table-for-the-c-programming-language
    // recursive decent parsing of expressions
    fn parse_expression(&mut self) -> Expr {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Expr {
        let mut left = self.parse_and();
        while *self.peek() == Token::Or {
            self.advance();
            let right = self.parse_and();
            left = Expr::BinOp(Box::new(left), BinOp::Or, Box::new(right));
        }
        left
    }

    fn parse_and(&mut self) -> Expr{
        let mut left = self.parse_bitor();
        while *self.peek() == Token::And {
            self.advance();
            let right =  self.parse_bitor();
            left = Expr::BinOp(Box::new(left), BinOp::And, Box::new(right));
        }
        left
    }

    fn parse_bitor(&mut self) -> Expr {
        let mut left = self.parse_bitxor();
        while *self.peek() == Token::Pipe {
            self.advance();
            let right = self.parse_bitxor();
            left = Expr::BinOp(Box::new(left), BinOp::BitOr, Box::new(right));
        }
        left
    }

    fn parse_bitxor(&mut self) -> Expr {
        let mut left = self.parse_bitand();
        while *self.peek() == Token::Caret {
            self.advance();
            let right = self.parse_bitand();
            left = Expr::BinOp(Box::new(left), BinOp::BitXor, Box::new(right));
        }
        left
    }

    fn parse_bitand(&mut self) -> Expr {
        let mut left = self.parse_equality();
        while *self.peek() == Token::Ampersand {
            self.advance();
            let right = self.parse_equality();
            left = Expr::BinOp(Box::new(left), BinOp::BitAnd, Box::new(right));
        }
        left
    }

    fn parse_equality(&mut self) -> Expr {
        let mut left = self.parse_comparison();
        loop {
            let op = match self.peek() {
                Token::Eq => BinOp::Eq,
                Token::NotEq => BinOp::NotEq,
                _ => break,
            };

            self.advance();
            let right = self.parse_comparison();
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        left
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut left = self.parse_bitwise_shift();
        loop {
            let op = match self.peek() {
                Token::Lt => BinOp::Lt,
                Token::Gt => BinOp::Gt,
                _ => break,
            };

            self.advance();
            let right = self.parse_bitwise_shift();
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        left
    }

    fn parse_bitwise_shift(&mut self) -> Expr {
        let mut left = self.parse_additive();
        loop {
            let op = match self.peek() {
                Token::LShift => BinOp::LShift,
                Token::RShift => BinOp::RShift,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive();
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        left
    }

    fn parse_additive(&mut self) -> Expr {
        let mut left = self.parse_multiplicative();
        loop {
            let op = match self.peek() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative();
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        left
    }

    fn parse_multiplicative(&mut self) -> Expr {
        let mut left = self.parse_unary();
        loop {
            let op = match self.peek() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary();
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        left
    }

    fn parse_unary(&mut self) -> Expr {
        match self.peek() {
            Token::Not => {
                self.advance();
                let expr = self.parse_unary();
                Expr::UnaryOp(UnaryOp::Not, Box::new(expr))
            },
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary();
                Expr::UnaryOp(UnaryOp::Neg, Box::new(expr))
            },
            Token::Tilde => {
                self.advance();
                let expr = self.parse_unary();
                Expr::UnaryOp(UnaryOp::BitNot, Box::new(expr))
            },
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Expr {
        let mut expr = self.parse_primary();
        loop {
            match self.peek() {
                Token::Dot => {
                    self.advance();
                    let field = match self.advance() {
                        Token::Ident(name) => name,
                        other => panic!("Expected field name after '.', got {:?}", other),
                    };
                    expr = Expr::FieldAccess(Box::new(expr), field);
                },

                Token::LParen => {
                    let name = match expr {
                        Expr::Identifier(name) => name,
                        _ => panic!("Can only call identifiers"),
                    };

                    self.advance();
                    let mut args = vec![];
                    
                    while *self.peek() != Token::RParen {
                        args.push(self.parse_expression());
                        if *self.peek() == Token::Comma {
                            self.advance();
                        }
                    }
                    self.expect(&Token::RParen);
                    expr = Expr::FunctionCall(name, args);
                },

                _ => break,
            }
        }
        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Token::IntLiteral(n) => Expr::IntLiteral(n),
            Token::BoolLiteral(b) => Expr::BoolLiteral(b),
            Token::Null => Expr::Null,
            Token::Ident(name) => {
                if *self.peek() == Token::LBrace {
                    self.advance();

                    let mut fields = vec![];
                    while *self.peek() != Token::RBrace {
                        let field_name = match self.advance() {
                            Token::Ident(n) => n,
                            other => panic!("Expected field name in struct init, got {:?}", other),
                        };

                        self.expect(&Token::Colon);
                        let field_expr = self.parse_expression();
                        fields.push((field_name, field_expr));
                        
                        if *self.peek() == Token::Comma {
                            self.advance();
                        }
                    }
                    self.expect(&Token::RBrace);
                    
                    Expr::StructInit(name, fields)
                } else {
                    Expr::Identifier(name)
                }
            },

            Token::LParen => {
                let expr = self.parse_expression();
                self.expect(&Token::RParen);
                expr
            },
            other => panic!("Unexpected token in primary expression: {:?}", other),
        }
    }
}