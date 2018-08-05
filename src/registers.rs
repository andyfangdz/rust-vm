extern crate bytes;
extern crate protobuf;

use protos::main::RegisterSet;

pub type Word = i64;

pub struct RegisterFile {
    pub pc: Word,
    pub r0: Word,
    pub r1: Word,
    pub r2: Word,
    pub r3: Word,
    pub r4: Word,
    pub r5: Word,

    pub fp: Word,
    pub sp: Word,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            pc: 0,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            fp: 0,
            sp: 0,
        }
    }

    pub fn get_enum(name: &str) -> RegisterSet {
        match name {
            "PC" => RegisterSet::PC,
            "R0" => RegisterSet::R0,
            "R1" => RegisterSet::R1,
            "R2" => RegisterSet::R2,
            "R3" => RegisterSet::R3,
            "R4" => RegisterSet::R4,
            "R5" => RegisterSet::R5,
            "FP" => RegisterSet::FP,
            "SP" => RegisterSet::SP,
            _ => panic!("unknown register"),
        }
    }

    pub fn get(&self, reg: RegisterSet) -> Word {
        match reg {
            RegisterSet::PC => self.pc,
            RegisterSet::R0 => self.r0,
            RegisterSet::R1 => self.r1,
            RegisterSet::R2 => self.r2,
            RegisterSet::R3 => self.r3,
            RegisterSet::R4 => self.r4,
            RegisterSet::R5 => self.r5,
            RegisterSet::FP => self.fp,
            RegisterSet::SP => self.sp,
        }
    }

    pub fn set(&mut self, reg: RegisterSet, val: Word) {
        match reg {
            RegisterSet::PC => self.pc = val,
            RegisterSet::R0 => self.r0 = val,
            RegisterSet::R1 => self.r1 = val,
            RegisterSet::R2 => self.r2 = val,
            RegisterSet::R3 => self.r3 = val,
            RegisterSet::R4 => self.r4 = val,
            RegisterSet::R5 => self.r5 = val,
            RegisterSet::FP => self.fp = val,
            RegisterSet::SP => self.sp = val,
        }
    }
}
