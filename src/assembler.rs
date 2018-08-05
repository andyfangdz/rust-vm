extern crate bytes;
extern crate protobuf;

use protos::main::{Instruction, InstructionCode};
use registers::RegisterFile;
use registers::Word;

fn addr_mode_reg(slices: Vec<&str>, code: InstructionCode) -> Instruction {
    let mut ret = Instruction::new();
    ret.set_code(code);
    ret.set_reg1(RegisterFile::get_enum(slices.get(1).unwrap()));
    ret.set_reg2(RegisterFile::get_enum(slices.get(2).unwrap()));
    ret
}

fn addr_mode_ja(slices: Vec<&str>, code: InstructionCode) -> Instruction {
    let mut ret = Instruction::new();
    ret.set_code(code);
    ret.set_offset(slices.get(1).unwrap().parse().unwrap());
    ret
}

fn addr_mode_jeq(slices: Vec<&str>, code: InstructionCode) -> Instruction {
    let mut ret = Instruction::new();
    ret.set_code(code);
    ret.set_reg1(RegisterFile::get_enum(slices.get(1).unwrap()));
    ret.set_reg2(RegisterFile::get_enum(slices.get(2).unwrap()));
    ret.set_offset(slices.get(3).unwrap().parse().unwrap());
    ret
}
fn addr_mode_jeqi(slices: Vec<&str>, code: InstructionCode) -> Instruction {
    let mut ret = Instruction::new();
    ret.set_code(code);
    ret.set_reg1(RegisterFile::get_enum(slices.get(1).unwrap()));
    ret.set_imm(slices.get(2).unwrap().parse().unwrap());
    ret.set_offset(slices.get(3).unwrap().parse().unwrap());
    ret
}

fn addr_mode_imm(slices: Vec<&str>, code: InstructionCode) -> Instruction {
    let mut ret = Instruction::new();
    ret.set_code(code);
    ret.set_reg1(RegisterFile::get_enum(slices.get(1).unwrap()));
    ret.set_imm(slices.get(2).unwrap().parse().unwrap());
    ret
}

fn addr_mode_int(slices: Vec<&str>, code: InstructionCode) -> Instruction {
    let mut ret = Instruction::new();
    ret.set_code(code);
    ret.set_imm(slices.get(1).unwrap().parse().unwrap());
    ret
}

fn addr_mode_hlt(code: InstructionCode) -> Instruction {
    let mut ret = Instruction::new();
    ret.set_code(code);
    ret
}

pub fn line_to_instruction(line: String) -> Instruction {
    let slices: Vec<&str> = line.split_whitespace().collect();
    match *slices.get(0).unwrap() {
        "MOV" => addr_mode_reg(slices,InstructionCode::MOV),
        "MOVI" => addr_mode_imm(slices, InstructionCode::MOVI),
        "ADD" => addr_mode_reg(slices,InstructionCode::ADD),
        "ADDI" => addr_mode_imm(slices, InstructionCode::ADDI),
        "JA" => addr_mode_ja(slices, InstructionCode::JA),
        "JEQ" => addr_mode_jeq(slices, InstructionCode::JEQ),
        "JEQI" => addr_mode_jeqi(slices, InstructionCode::JEQI),
        "INT" => addr_mode_int(slices, InstructionCode::INT),
        "HALT" => addr_mode_hlt(InstructionCode::HALT),
        _ => panic!("unknown instruction"),
    }
}
