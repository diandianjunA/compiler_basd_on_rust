use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Terminals {
    Integer(i64),//整数
    FloatNumber(f64),//浮点数
    Character(char),//字符
    String(String),//字符串
    Plus,//+
    Minus,//-
    Asterisk,//*
    Slash,// /
    LeftParen,// (
    RightParen,// )
    Eof,//文件结束
    Int,//int
    Float,//float
    Char,//char
    Void,//void
    Identifier(String),//标识符
    Equals,//=
    If,//if
    Else,//else
    GreaterThan,//>
    LessThan,//<
    GreaterThanEquals,//>=
    LessThanEquals,//<=
    EqualsEquals,//==
    BangEquals,// !=
    OpenBrace,//{
    CloseBrace,//}
    True,//true
    False,//false
    While,//while
    Return,//return
    Comma,//,
    SemiColon,//;
    Colon,//:
    And,//&&
    Or,//||
    Not,// !
    Bad,
    Whitespace,
    LeftBracket,//[
    RightBracket,//]
    For,//for
    Switch,//switch
    Case,//case
    Default,//default
    Break,//break
    Continue,//continue
    Do,//do
    PlusPlus,//++
    MinusMinus,//--
    None
}

impl Terminals {
    pub fn to_num(&self) -> usize {
        match self {
            Terminals::Integer(_) => 0,
            Terminals::FloatNumber(_) => 1,
            Terminals::Character(_) => 2,
            Terminals::String(_) => 3,
            Terminals::Plus => 4,
            Terminals::Minus => 5,
            Terminals::Asterisk => 6,
            Terminals::Slash => 7,
            Terminals::LeftParen => 8,
            Terminals::RightParen => 9,
            Terminals::Eof => 10,
            Terminals::Int => 11,
            Terminals::Float => 12,
            Terminals::Char => 13,
            Terminals::Void => 14,
            Terminals::Identifier(_) => 15,
            Terminals::Equals => 16,
            Terminals::If => 17,
            Terminals::Else => 18,
            Terminals::GreaterThan => 19,
            Terminals::LessThan => 20,
            Terminals::GreaterThanEquals => 21,
            Terminals::LessThanEquals => 22,
            Terminals::EqualsEquals => 23,
            Terminals::BangEquals => 24,
            Terminals::OpenBrace => 25,
            Terminals::CloseBrace => 26,
            Terminals::True => 27,
            Terminals::False => 28,
            Terminals::While => 29,
            Terminals::Return => 30,
            Terminals::Comma => 31,
            Terminals::SemiColon => 32,
            Terminals::Colon => 33,
            Terminals::And => 34,
            Terminals::Or => 35,
            Terminals::Not => 36,
            Terminals::Bad => 37,
            Terminals::Whitespace => 38,
            Terminals::LeftBracket => 39,
            Terminals::RightBracket => 40,
            Terminals::For => 41,
            Terminals::Switch => 42,
            Terminals::Case => 43,
            Terminals::Default => 44,
            Terminals::Break => 45,
            Terminals::Continue => 46,
            Terminals::Do => 47,
            Terminals::PlusPlus => 48,
            Terminals::MinusMinus => 49,
            Terminals::None => 50,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Terminals::Integer(_) => "Integer".to_string(),
            Terminals::FloatNumber(_) => "FloatNumber".to_string(),
            Terminals::Character(_) => "Char".to_string(),
            Terminals::String(_) => "String".to_string(),
            Terminals::Plus => "Plus".to_string(),
            Terminals::Minus => "Minus".to_string(),
            Terminals::Asterisk => "Asterisk".to_string(),
            Terminals::Slash => "Slash".to_string(),
            Terminals::LeftParen => "LeftParen".to_string(),
            Terminals::RightParen => "RightParen".to_string(),
            Terminals::Eof => "Eof".to_string(),
            Terminals::Int => "Int".to_string(),
            Terminals::Float => "Float".to_string(),
            Terminals::Char => "Char".to_string(),
            Terminals::Void => "Void".to_string(),
            Terminals::Identifier(_) => "Identifier".to_string(),
            Terminals::Equals => "Equals".to_string(),
            Terminals::If => "If".to_string(),
            Terminals::Else => "Else".to_string(),
            Terminals::GreaterThan => "GreaterThan".to_string(),
            Terminals::LessThan => "LessThan".to_string(),
            Terminals::GreaterThanEquals => "GreaterThanEquals".to_string(),
            Terminals::LessThanEquals => "LessThanEquals".to_string(),
            Terminals::EqualsEquals => "EqualsEquals".to_string(),
            Terminals::BangEquals => "BangEquals".to_string(),
            Terminals::OpenBrace => "OpenBrace".to_string(),
            Terminals::CloseBrace => "CloseBrace".to_string(),
            Terminals::True => "True".to_string(),
            Terminals::False => "False".to_string(),
            Terminals::While => "While".to_string(),
            Terminals::Return => "Return".to_string(),
            Terminals::Comma => "Comma".to_string(),
            Terminals::SemiColon => "SemiColon".to_string(),
            Terminals::Colon => "Colon".to_string(),
            Terminals::And => "And".to_string(),
            Terminals::Or => "Or".to_string(),
            Terminals::Not => "Not".to_string(),
            Terminals::Bad => "Bad".to_string(),
            Terminals::Whitespace => "Whitespace".to_string(),
            Terminals::LeftBracket => "[".to_string(),
            Terminals::RightBracket => "]".to_string(),
            Terminals::For => "For".to_string(),
            Terminals::Switch => "Switch".to_string(),
            Terminals::Case => "Case".to_string(),
            Terminals::Default => "Default".to_string(),
            Terminals::Break => "Break".to_string(),
            Terminals::Continue => "Continue".to_string(),
            Terminals::Do => "Do".to_string(),
            Terminals::PlusPlus => "PlusPlus".to_string(),
            Terminals::MinusMinus => "MinusMinus".to_string(),
            Terminals::None => "None".to_string(),
        }
    }
}

impl Display for Terminals {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Terminals::Integer(i) => write!(f, "Integer: {}", i),
            Terminals::FloatNumber(f_) => write!(f, "FloatNumber: {}", f_),
            Terminals::Character(c) => write!(f, "Char: {}", c),
            Terminals::String(s) => write!(f, "String: {}", s),
            Terminals::Plus => write!(f, "+"),
            Terminals::Minus => write!(f, "-"),
            Terminals::Asterisk => write!(f, "*"),
            Terminals::Slash => write!(f, "/"),
            Terminals::LeftParen => write!(f, "("),
            Terminals::RightParen => write!(f, ")"),
            Terminals::Eof => write!(f, "Eof"),
            Terminals::Int => write!(f, "Int"),
            Terminals::Float => write!(f, "Float"),
            Terminals::Char => write!(f, "Char"),
            Terminals::Void => write!(f, "Void"),
            Terminals::Identifier(i) => write!(f, "Identifier: {}", i),
            Terminals::Equals => write!(f, "="),
            Terminals::If => write!(f, "If"),
            Terminals::Else => write!(f, "Else"),
            Terminals::GreaterThan => write!(f, ">"),
            Terminals::LessThan => write!(f, "<"),
            Terminals::GreaterThanEquals => write!(f, ">="),
            Terminals::LessThanEquals => write!(f, "<="),
            Terminals::EqualsEquals => write!(f, "=="),
            Terminals::BangEquals => write!(f, "!="),
            Terminals::OpenBrace => write!(f, "{{"),
            Terminals::CloseBrace => write!(f, "}}"),
            Terminals::True => write!(f, "True"),
            Terminals::False => write!(f, "False"),
            Terminals::While => write!(f, "While"),
            Terminals::Return => write!(f, "Return"),
            Terminals::Comma => write!(f, "Comma"),
            Terminals::SemiColon => write!(f, "SemiColon"),
            Terminals::Colon => write!(f, "Colon"),
            Terminals::And => write!(f, "&&"),
            Terminals::Or => write!(f, "||"),
            Terminals::Not => write!(f, "!"),
            Terminals::Bad => write!(f, "Bad"),
            Terminals::Whitespace => write!(f, "Whitespace"),
            Terminals::LeftBracket => write!(f, "["),
            Terminals::RightBracket => write!(f, "]"),
            Terminals::For => write!(f, "For"),
            Terminals::Switch => write!(f, "Switch"),
            Terminals::Case => write!(f, "Case"),
            Terminals::Default => write!(f, "Default"),
            Terminals::Break => write!(f, "Break"),
            Terminals::Continue => write!(f, "Continue"),
            Terminals::Do => write!(f, "Do"),
            Terminals::PlusPlus => write!(f, "PlusPlus"),
            Terminals::MinusMinus => write!(f, "MinusMinus"),
            Terminals::None => write!(f, "None"),
        }
    }
}

impl PartialEq<Self> for Terminals {
    fn eq(&self, other: &Self) -> bool {
        self.to_num() == other.to_num()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: Terminals,
    pub span: TextSpan,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: Terminals, span: TextSpan,line: usize,column: usize) -> Self {
        Self {
            kind,
            span,
            line,
            column,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self { start, end, literal }
    }

    pub fn combine(mut spans: Vec<TextSpan>) -> TextSpan {
        spans.sort_by(
            |a, b| a.start.cmp(&b.start)
        );

        let start = spans.first().unwrap().start;
        let end = spans.last().unwrap().end;

        let mut literal = String::new();
        for (index, span) in spans.iter().enumerate() {
            if index > 0 {
                let last = spans.get(index - 1).unwrap();
                let diff = span.start - last.end;
                literal.push_str(&" ".repeat(diff));
            }
            literal.push_str(&span.literal);
        }

        TextSpan::new(start, end, literal)
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}


pub struct Lexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input, position: 0,line:1,column:1 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            let eof_char: char = '\0';
            self.position += 1;
            self.column += 1;
            return Some(Token::new(
                Terminals::Eof,
                TextSpan::new(self.position - 1, self.position, eof_char.to_string()),
                self.line,
                self.column,
            ));
        }
        let c = self.current_char();
        return c.map(|c| {
            let start = self.position;
            let kind;
            if Self::is_number_start(&c) {
                kind = self.consume_number();
            } else if Self::is_whitespace(&c) {
                self.consume().unwrap();
                kind = Terminals::Whitespace;
            }else if Self::is_identifier_start(&c) {
                let identifier = self.consume_identifier();
                kind = match identifier.as_str() {
                    "int" => Terminals::Int,
                    "float" => Terminals::Float,
                    "char" => Terminals::Char,
                    "void" => Terminals::Void,
                    "if" => Terminals::If,
                    "else" => Terminals::Else,
                    "true" => Terminals::True,
                    "false" => Terminals::False,
                    "while" => Terminals::While,
                    "return" => Terminals::Return,
                    "for" => Terminals::For,
                    "switch" => Terminals::Switch,
                    "case" => Terminals::Case,
                    "default" => Terminals::Default,
                    "break" => Terminals::Break,
                    "continue" => Terminals::Continue,
                    "do" => Terminals::Do,
                    _ => Terminals::Identifier(identifier),
                };
            }else if Self::is_char_start(&c){
                self.consume();
                let c = self.consume().unwrap();
                if self.current_char() == Some('\'') {
                    self.consume();
                    kind = Terminals::Character(c);
                }else {
                    kind = Terminals::Bad;
                }
            }else if Self::is_string_start(&c){
                self.consume();
                let mut string = String::new();
                while let Some(c) = self.current_char() {
                    if c == '"' {
                        self.consume();
                        break;
                    }
                    string.push(c);
                    self.consume();
                }
                kind = Terminals::String(string);
            } else {
                kind = self.consume_punctuation();
            }
            let end = self.position;
            let span = TextSpan::new(start, end, self.input[start..end].to_string());
            Token::new(kind, span, self.line,self.line)
        });
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic()
    }

    fn is_identifier_part(c: &char) -> bool {
        c.is_alphanumeric() || c == &'_'
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn is_char_start(c: &char) -> bool {
        c == &'\''
    }

    fn is_string_start(c: &char) -> bool {
        c == &'"'
    }

    fn consume_number(&mut self) -> Terminals {
        let mut number = String::new();
        while let Some(c) = self.current_char() {
            if Self::is_number_start(&c) {
                self.consume().unwrap();
                number.push(c);
            } else {
                break;
            }
        }
        if let Some(c) = self.current_char() {
            if c == '.' {
                self.consume().unwrap();
                number.push(c);
                while let Some(c) = self.current_char() {
                    if Self::is_number_start(&c) {
                        self.consume().unwrap();
                        number.push(c);
                    } else {
                        break;
                    }
                }
                return Terminals::FloatNumber(number.parse::<f64>().unwrap());
            }
        }
        Terminals::Integer(number.parse::<i64>().unwrap())
    }

    fn consume(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.position += 1;
        if c == Some('\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        c
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if Self::is_identifier_part(&c) {
                self.consume().unwrap();
                identifier.push(c);
            } else {
                break;
            }
        }
        identifier
    }

    fn consume_punctuation(&mut self) -> Terminals {
        let c = self.consume().unwrap();
        match c {
            '+' => {
                if self.current_char() == Some('+') {
                    self.consume();
                    Terminals::PlusPlus
                } else {
                    Terminals::Plus
                }
            },
            '-' => {
                if self.current_char() == Some('-') {
                    self.consume();
                    Terminals::MinusMinus
                } else {
                    Terminals::Minus
                }
            },
            '*' => Terminals::Asterisk,
            '/' => {
                if self.current_char() == Some('/') {
                    self.consume();
                    while let Some(c) = self.current_char() {
                        if c == '\n' {
                            self.consume();
                            break;
                        }
                        self.consume();
                    }
                    Terminals::Whitespace
                }else if self.current_char() == Some('*') {
                    self.consume();
                    while let Some(c) = self.consume() {
                        if c == '*' && self.current_char() == Some('/') {
                            self.consume();
                            break;
                        }
                        self.consume();
                    }
                    Terminals::Whitespace
                } else {
                    Terminals::Slash
                }
            },
            '(' => Terminals::LeftParen,
            ')' => Terminals::RightParen,
            '{' => Terminals::OpenBrace,
            '}' => Terminals::CloseBrace,
            ';' => Terminals::SemiColon,
            ',' => Terminals::Comma,
            '=' => {
                if self.current_char() == Some('=') {
                    self.consume();
                    Terminals::EqualsEquals
                } else {
                    Terminals::Equals
                }
            }
            '>' => {
                if self.current_char() == Some('=') {
                    self.consume();
                    Terminals::GreaterThanEquals
                } else {
                    Terminals::GreaterThan
                }
            }
            '<' => {
                if self.current_char() == Some('=') {
                    self.consume();
                    Terminals::LessThanEquals
                } else {
                    Terminals::LessThan
                }
            }
            '!' => {
                if self.current_char() == Some('=') {
                    self.consume();
                    Terminals::BangEquals
                } else {
                    Terminals::Not
                }
            }
            '&' => {
                if self.current_char() == Some('&') {
                    self.consume();
                    Terminals::And
                } else {
                    panic!("词法错误： 不支持的字符: {}", c)
                }
            }
            '|' => {
                if self.current_char() == Some('|') {
                    self.consume();
                    Terminals::Or
                } else {
                    panic!("词法错误： 不支持的字符: {}", c);
                }
            }
            '[' => Terminals::LeftBracket,
            ']' => Terminals::RightBracket,
            ':' => Terminals::Colon,
            _ => {
                panic!("词法错误： 不支持的字符: {}", c)
            },
        }
    }
}
