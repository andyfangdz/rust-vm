extern crate rust_vm;

use rust_vm::assembler::line_to_instruction;
use rust_vm::protos::main::{Instruction};
use std::io::{self, BufRead};
use rust_vm::context::ExecutionContext;

fn main() {
    let stdin = io::stdin();
    let instructions: Vec<Instruction> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(line_to_instruction)
        .collect();

    println!("{:#?}", instructions);
    println!("Begin execution:");
    ExecutionContext::new(instructions).eval_all();
}
