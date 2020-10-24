use std::{collections::LinkedList, convert::{TryFrom, TryInto}, io::Read};

use serde::{Serialize, Deserialize};

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

pub fn tokenize(s: &mut impl Read) -> Option<LinkedList<Token>> {
    let mut buf = LinkedList::new();
    let mut bytes = [0u8; 100];
    let mut read = 1;

    while read > 0 {
        read = s.read(&mut bytes).ok()?;

        for i in 0..read {
            if let Ok(t) = (bytes[i] as char).try_into() {
                buf.push_back(t);
            }
        }
    }

    // for c in s. {
    //     if let Ok(t) = c.try_into() {
    //         buf.push(t)
    //     }
    // }
    Some(buf)
}
