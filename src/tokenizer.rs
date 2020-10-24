use std::{collections::LinkedList, convert::TryFrom, io::BufRead};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    IncrementPtr,
    DecrementPtr,
    Increment,
    Decrement,
    Output,
    Input,
    JumpFwd,
    JumpBack,
}

impl TryFrom<char> for Token {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '>' => Ok(Token::IncrementPtr),
            '<' => Ok(Token::DecrementPtr),
            '+' => Ok(Token::Increment),
            '-' => Ok(Token::Decrement),
            '.' => Ok(Token::Output),
            ',' => Ok(Token::Input),
            '[' => Ok(Token::JumpFwd),
            ']' => Ok(Token::JumpBack),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", untagged)]
pub enum Tree {
    Instruction(Instruction),
    Block(LinkedList<Tree>),
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Instruction {
    IncrementPtr,
    DecrementPtr,
    Increment,
    Decrement,
    Output,
    Input,
}

impl Tree {
    pub fn parse<'a>(mut it: impl Iterator<Item = &'a Token>) -> LinkedList<Tree> {
        let mut buf = LinkedList::new();

        while let Some(t) = it.next() {
            match t {
                Token::IncrementPtr => buf.push_back(Tree::Instruction(Instruction::IncrementPtr)),
                Token::DecrementPtr => buf.push_back(Tree::Instruction(Instruction::DecrementPtr)),
                Token::Increment => buf.push_back(Tree::Instruction(Instruction::Increment)),
                Token::Decrement => buf.push_back(Tree::Instruction(Instruction::Decrement)),
                Token::Output => buf.push_back(Tree::Instruction(Instruction::Output)),
                Token::Input => buf.push_back(Tree::Instruction(Instruction::Input)),

                Token::JumpFwd => {
                    let mut tokens = Vec::new();
                    let mut depth = 1;
                    while let Some(token) = it.next() {
                        match token {
                            Token::JumpFwd => depth += 1,
                            Token::JumpBack => depth -= 1,
                            _ => {}
                        }

                        if depth < 1 {
                            break;
                        }

                        tokens.push(token);
                    }

                    buf.push_back(Self::Block(Self::parse(tokens.into_iter())));
                }

                _ => {}
            }
        }

        buf
    }
}

pub fn tokenize(s: impl BufRead) -> LinkedList<Token> {
    s.bytes()
        .filter_map(|c| c.ok().and_then(|x| Token::try_from(x as char).ok()))
        .collect()
}
