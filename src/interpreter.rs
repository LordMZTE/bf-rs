use std::{
    collections::HashMap,
    io::{Read, Write},
};

use crate::tokenizer::{Instruction, Tree};

pub fn run<'a>(
    mut tokens: impl Iterator<Item = &'a Tree>,
    stdout: &mut impl Write,
    stdin: &mut impl Read,
    mem: &mut HashMap<isize, u8>,
    ptr: &mut isize,
) {
    while let Some(t) = tokens.next() {
        match t {
            Tree::Instruction(i) => {
                run_instruction(*i, stdout, stdin, mem, ptr);
            }
            Tree::Block(tr) => {
                while mem.get(ptr).unwrap_or(&0) != &0 {
                    run(tr.iter(), stdout, stdin, mem, ptr);

                    if mem.get(ptr).unwrap_or(&0) == &0 {
                        break;
                    }
                }
            }
        }
    }
}

pub fn run_instruction(
    i: Instruction,
    stdout: &mut impl Write,
    stdin: &mut impl Read,
    mem: &mut HashMap<isize, u8>,
    ptr: &mut isize,
) {
    match i {
        Instruction::IncrementPtr => *ptr += 1,
        Instruction::DecrementPtr => *ptr -= 1,
        Instruction::Increment => {
            mem.entry(*ptr).and_modify(|n| *n = n.overflowing_add(1).0).or_insert(1);
        }
        Instruction::Decrement => {
            mem.entry(*ptr).and_modify(|n| *n = n.overflowing_sub(1).0).or_insert(255);
        }
        Instruction::Output => {
            stdout.write(&[*mem.entry(*ptr).or_default()]).unwrap();
            stdout.flush().unwrap();
        }
        Instruction::Input => {
            let mut buf = [0u8; 1];
            stdin.read(&mut buf).unwrap();
            mem.insert(*ptr, buf[0]);
        }
    }
}

pub fn run_new<'a>(
    tokens: impl Iterator<Item = &'a Tree>,
    stdout: &mut impl Write,
    stdin: &mut impl Read,
) {
    let mut mem = HashMap::new();
    run(tokens, stdout, stdin, &mut mem, &mut 0);
}
