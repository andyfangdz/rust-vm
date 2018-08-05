extern crate bytes;
extern crate protobuf;

use memory::make_ram;
use memory::RAM;
use protos::main::Instruction;
use protos::main::InterruptCode;
use protos::main::InstructionCode;
use protos::main::RegisterSet;
use registers::RegisterFile;
use interrupts::get_interrupt_enum;

pub struct ExecutionContext {
    pub reg_file: RegisterFile,
    pub halted: bool,
    pub instr_mem: Vec<Instruction>,
    pub data_mem: RAM,
}

impl ExecutionContext {
    fn fetch(&mut self) -> Option<&Instruction> {
        let ret;
        if self.halted {
            ret = None
        } else {
            ret = self.instr_mem.get(self.reg_file.pc as usize)
        }
        self.reg_file.pc = self.reg_file.pc + 1;
        return ret;
    }

    fn eval(&mut self, instr: Instruction) {
        match instr.get_code() {
            InstructionCode::MOV => {
                let val1 = self.reg_file.get(instr.get_reg2());
                self.reg_file.set(instr.get_reg1(), val1);
            }
            InstructionCode::MOVI => self.reg_file.set(instr.get_reg1(), instr.get_imm()),
            InstructionCode::ADD => {
                let val1 = self.reg_file.get(instr.get_reg1());
                let val2 = self.reg_file.get(instr.get_reg2());
                let result = val1 + val2;
                self.reg_file.set(instr.get_reg1(), result);
            }
            InstructionCode::ADDI => {
                let val1 = self.reg_file.get(instr.get_reg1());
                let val2 = instr.get_imm();
                let result = val1 + val2;
                self.reg_file.set(instr.get_reg1(), result);
            }
            InstructionCode::INT => {
                let int_code = instr.get_imm();
                self.dispatch_interrupt(get_interrupt_enum(int_code));
            }
            InstructionCode::JA => {
                let offset = instr.get_offset();
                self.reg_file.pc += offset;
            }
            InstructionCode::JEQ => {
                let val1 = self.reg_file.get(instr.get_reg1());
                let val2 = self.reg_file.get(instr.get_reg2());
                let offset = instr.get_offset();
                if val1 == val2 {
                    self.reg_file.pc += offset;
                }
            }
            InstructionCode::JEQI => {
                let val1 = self.reg_file.get(instr.get_reg1());
                let val2 = instr.get_imm();
                let offset = instr.get_offset();
                if val1 == val2 {
                    self.reg_file.pc += offset;
                }
            }
            InstructionCode::HALT => self.halted = true,
        }
    }

    pub fn eval_all(&mut self) {
        loop {
            match self.fetch().cloned() {
                Some(instr) => self.eval(instr),
                None => break,
            }
        }
    }

    fn dispatch_interrupt(&mut self, interrupt: InterruptCode) {
        match interrupt {
            InterruptCode::PRINT_REG0 => println!("{:#?}", self.reg_file.r0)
        }
    }

    pub fn new(instructions: Vec<Instruction>) -> Self {
        ExecutionContext{
            reg_file: RegisterFile::new(),
            halted: false,
            instr_mem: instructions,
            data_mem: make_ram(1024),
        }
    }
}
