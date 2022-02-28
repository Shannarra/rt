extern crate core;

use std::collections::HashMap;
use std::env::var;
use std::fmt::{Debug, Display, Formatter};
use std::io::ErrorKind;
use std::ops::Index;
use crate::TokenKind::Word;

#[derive(Clone)]
struct Pos {
    file: String,
    row: u16,
    col: u16
}

impl Pos {
    pub fn new() -> Self { Self { file: String::new(), row: 0, col: 0 } }

    pub fn make(file: String, row: u16, col: u16) -> Self { Self { file, row, col } }

    pub fn from_tuple(src: (String, u16, u16)) -> Self { Self { file: src.0, row: src.1, col: src.2 } }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.row, self.col)
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.row, self.col)
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialOrd, PartialEq)]
enum TokenKind {
    Word,
    Keyword,
    Operator,
    Numeric
}

#[derive(Debug)]
#[derive(Clone)]
enum OperatorKind {
    Eq,
    Plus,
    Minus,
    Mul,
    Div
}

#[derive(Clone)]
struct Token {
    value:    String,
    position: Pos,
    kind:     TokenKind
}

impl Token {
    pub fn new() -> Self {
        Self { value: String::from("empty_token"), position: Pos::new(), kind: TokenKind::Word }
    }

    pub fn make(value: String, position: Pos, kind: TokenKind) -> Self {
        Self {
            value,
            position,
            kind
        }
    }

    pub fn with_formatted_value<T: Display>(
        value: &str,
        sep: &str,
        fmt_val: T,
        pos: Pos,
        kind: TokenKind
    ) -> Self {
        Token::make(format!("{}{}{}", value, sep, fmt_val), pos, kind)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {} ({:?})", self.position, self.value, self.kind)
    }
}

static KEYWORDS: [&str; 3] = [ "let", "be", "fn"];
static OPERATORS: [&str; 5] = ["=", "+", "-", "(", ")"];


pub trait StringUtils {
    fn trim_newlines(&self) -> Self;
}

impl StringUtils for String {
    fn trim_newlines(&self) -> String {
        let mut s = self.clone();

        while s.ends_with('\n') || s.ends_with('\r') {
            if s.ends_with('\n') {
                assert_eq!(s.pop(), Some('\n'));
            }
            if s.ends_with('\r') {
                assert_eq!(s.pop(), Some('\r'));
            }
        }

        s
    }
}


fn determine_kind(token: String) -> TokenKind {
    fn is_hex(word: String) -> bool {
        let mut id = 0;
        for c in word[2..].chars() {
            if !c.is_digit(16) {
                return false
            }
            id+=1;
        }

        if word.len() >= 2 {
            if word.chars().nth(0) == Some('0') && word.chars().nth(1) == Some('x') {
                return true;
            }
        }
        false
    }

    for k in KEYWORDS {
        if token.eq(k) {
            return TokenKind::Keyword
        }
    }

    for o in OPERATORS {
        if token.eq(o){
            return TokenKind::Operator
        }
    }

    if token.parse::<f64>().is_ok() || is_hex(token) {
        return TokenKind::Numeric
    }

    TokenKind::Word
}

fn lex(text: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut curr = String::new();
    let mut col = 0;
    let mut start = 0;

    for ch in text.chars() {
        col += 1;
        if ch != ' ' {
            curr.push(ch);
            continue;
        }

        tokens.push(
            Token::make(curr.clone().trim_newlines(),
                        Pos::from_tuple((String::from("main.rs"), 0, start)),
                        determine_kind(curr.clone().trim_newlines())
            )
        );

        start = col;
        curr = String::new();

    }

    tokens.push(
                Token::make(curr.clone().trim_newlines(),
                            Pos::from_tuple((String::from("main.rs"), 0, start)),
                            determine_kind(curr.clone().trim_newlines())
                )
            );

    tokens
}

struct SyntaxNode {
    value:      String,
    text:       String,
    position:   Pos,
    kind:       TokenKind
}

fn parse(tokens: Vec<Token>) -> HashMap<String, String> {
    let mut t_id = 0;
    let mut hash = HashMap::new();
    let mut last_key = String::new();

    while t_id < tokens.len()-1 {
        let curr_token = tokens[t_id].clone();
        let next_token = tokens[t_id+1].clone();

        match curr_token.kind {
            TokenKind::Keyword => {
                if curr_token.value.eq("let") {
                    if next_token.kind != TokenKind::Word {
                        panic!("Expected a word after \"let\" declaration at {}, got \"{}\" ({:?})",
                               curr_token.position, next_token.value, next_token.kind)
                    }
                    t_id+=2;
                    last_key = next_token.value.clone();
                }
                if curr_token.value.eq("be") {
                    t_id+=2;
                    hash.insert(last_key.clone(), next_token.value.clone());
                }
            },
            TokenKind::Operator => {
                if curr_token.value.eq("=") {
                    t_id+=2;
                    hash.insert(last_key.clone(), next_token.value.clone());
                }
            }
            _ => {  //word, numeric
                t_id+=1;
            }
        }
    }

    if false {
        for token in tokens {
            println!("{:?}", token);
        }
    }

    hash
}

fn main() {
    let tokens: Vec<Token> = lex(String::from(
        "let it be 0.654876418768547946\n \
        let\n\r hex be 0xfb00be\
        let a be hex"));

    let vars = parse(tokens);

    println!("{}", vars.len());

    for var in vars {
        println!("{} : {}", var.0, var.1);
    }
}
