// This file is generated by rust-protobuf 2.0.4. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Instruction {
    // message fields
    pub code: InstructionCode,
    pub imm: i64,
    pub reg1: RegisterSet,
    pub reg2: RegisterSet,
    pub offset: i64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl Instruction {
    pub fn new() -> Instruction {
        ::std::default::Default::default()
    }

    // .InstructionCode code = 1;

    pub fn clear_code(&mut self) {
        self.code = InstructionCode::MOV;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: InstructionCode) {
        self.code = v;
    }

    pub fn get_code(&self) -> InstructionCode {
        self.code
    }

    // int64 imm = 2;

    pub fn clear_imm(&mut self) {
        self.imm = 0;
    }

    // Param is passed by value, moved
    pub fn set_imm(&mut self, v: i64) {
        self.imm = v;
    }

    pub fn get_imm(&self) -> i64 {
        self.imm
    }

    // .RegisterSet reg1 = 3;

    pub fn clear_reg1(&mut self) {
        self.reg1 = RegisterSet::PC;
    }

    // Param is passed by value, moved
    pub fn set_reg1(&mut self, v: RegisterSet) {
        self.reg1 = v;
    }

    pub fn get_reg1(&self) -> RegisterSet {
        self.reg1
    }

    // .RegisterSet reg2 = 4;

    pub fn clear_reg2(&mut self) {
        self.reg2 = RegisterSet::PC;
    }

    // Param is passed by value, moved
    pub fn set_reg2(&mut self, v: RegisterSet) {
        self.reg2 = v;
    }

    pub fn get_reg2(&self) -> RegisterSet {
        self.reg2
    }

    // int64 offset = 5;

    pub fn clear_offset(&mut self) {
        self.offset = 0;
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: i64) {
        self.offset = v;
    }

    pub fn get_offset(&self) -> i64 {
        self.offset
    }
}

impl ::protobuf::Message for Instruction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.code, 1, &mut self.unknown_fields)?
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.imm = tmp;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.reg1, 3, &mut self.unknown_fields)?
                },
                4 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.reg2, 4, &mut self.unknown_fields)?
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.offset = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.code != InstructionCode::MOV {
            my_size += ::protobuf::rt::enum_size(1, self.code);
        }
        if self.imm != 0 {
            my_size += ::protobuf::rt::value_size(2, self.imm, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.reg1 != RegisterSet::PC {
            my_size += ::protobuf::rt::enum_size(3, self.reg1);
        }
        if self.reg2 != RegisterSet::PC {
            my_size += ::protobuf::rt::enum_size(4, self.reg2);
        }
        if self.offset != 0 {
            my_size += ::protobuf::rt::value_size(5, self.offset, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != InstructionCode::MOV {
            os.write_enum(1, self.code.value())?;
        }
        if self.imm != 0 {
            os.write_int64(2, self.imm)?;
        }
        if self.reg1 != RegisterSet::PC {
            os.write_enum(3, self.reg1.value())?;
        }
        if self.reg2 != RegisterSet::PC {
            os.write_enum(4, self.reg2.value())?;
        }
        if self.offset != 0 {
            os.write_int64(5, self.offset)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Instruction {
        Instruction::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<InstructionCode>>(
                    "code",
                    |m: &Instruction| { &m.code },
                    |m: &mut Instruction| { &mut m.code },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "imm",
                    |m: &Instruction| { &m.imm },
                    |m: &mut Instruction| { &mut m.imm },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RegisterSet>>(
                    "reg1",
                    |m: &Instruction| { &m.reg1 },
                    |m: &mut Instruction| { &mut m.reg1 },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RegisterSet>>(
                    "reg2",
                    |m: &Instruction| { &m.reg2 },
                    |m: &mut Instruction| { &mut m.reg2 },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "offset",
                    |m: &Instruction| { &m.offset },
                    |m: &mut Instruction| { &mut m.offset },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Instruction>(
                    "Instruction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Instruction {
        static mut instance: ::protobuf::lazy::Lazy<Instruction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Instruction,
        };
        unsafe {
            instance.get(Instruction::new)
        }
    }
}

impl ::protobuf::Clear for Instruction {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_imm();
        self.clear_reg1();
        self.clear_reg2();
        self.clear_offset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Instruction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Program {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

impl Program {
    pub fn new() -> Program {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for Program {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Program {
        Program::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Program>(
                    "Program",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Program {
        static mut instance: ::protobuf::lazy::Lazy<Program> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Program,
        };
        unsafe {
            instance.get(Program::new)
        }
    }
}

impl ::protobuf::Clear for Program {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Program {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Program {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum InstructionCode {
    MOV = 0,
    MOVI = 1,
    ADD = 2,
    ADDI = 3,
    JA = 25,
    JEQ = 26,
    JEQI = 27,
    HALT = 99,
    INT = 100,
}

impl ::protobuf::ProtobufEnum for InstructionCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<InstructionCode> {
        match value {
            0 => ::std::option::Option::Some(InstructionCode::MOV),
            1 => ::std::option::Option::Some(InstructionCode::MOVI),
            2 => ::std::option::Option::Some(InstructionCode::ADD),
            3 => ::std::option::Option::Some(InstructionCode::ADDI),
            25 => ::std::option::Option::Some(InstructionCode::JA),
            26 => ::std::option::Option::Some(InstructionCode::JEQ),
            27 => ::std::option::Option::Some(InstructionCode::JEQI),
            99 => ::std::option::Option::Some(InstructionCode::HALT),
            100 => ::std::option::Option::Some(InstructionCode::INT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [InstructionCode] = &[
            InstructionCode::MOV,
            InstructionCode::MOVI,
            InstructionCode::ADD,
            InstructionCode::ADDI,
            InstructionCode::JA,
            InstructionCode::JEQ,
            InstructionCode::JEQI,
            InstructionCode::HALT,
            InstructionCode::INT,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("InstructionCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for InstructionCode {
}

impl ::std::default::Default for InstructionCode {
    fn default() -> Self {
        InstructionCode::MOV
    }
}

impl ::protobuf::reflect::ProtobufValue for InstructionCode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum InterruptCode {
    PRINT_REG0 = 0,
}

impl ::protobuf::ProtobufEnum for InterruptCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<InterruptCode> {
        match value {
            0 => ::std::option::Option::Some(InterruptCode::PRINT_REG0),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [InterruptCode] = &[
            InterruptCode::PRINT_REG0,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("InterruptCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for InterruptCode {
}

impl ::std::default::Default for InterruptCode {
    fn default() -> Self {
        InterruptCode::PRINT_REG0
    }
}

impl ::protobuf::reflect::ProtobufValue for InterruptCode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RegisterSet {
    PC = 0,
    R0 = 1,
    R1 = 2,
    R2 = 3,
    R3 = 4,
    R4 = 5,
    R5 = 6,
    FP = 7,
    SP = 8,
}

impl ::protobuf::ProtobufEnum for RegisterSet {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RegisterSet> {
        match value {
            0 => ::std::option::Option::Some(RegisterSet::PC),
            1 => ::std::option::Option::Some(RegisterSet::R0),
            2 => ::std::option::Option::Some(RegisterSet::R1),
            3 => ::std::option::Option::Some(RegisterSet::R2),
            4 => ::std::option::Option::Some(RegisterSet::R3),
            5 => ::std::option::Option::Some(RegisterSet::R4),
            6 => ::std::option::Option::Some(RegisterSet::R5),
            7 => ::std::option::Option::Some(RegisterSet::FP),
            8 => ::std::option::Option::Some(RegisterSet::SP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RegisterSet] = &[
            RegisterSet::PC,
            RegisterSet::R0,
            RegisterSet::R1,
            RegisterSet::R2,
            RegisterSet::R3,
            RegisterSet::R4,
            RegisterSet::R5,
            RegisterSet::FP,
            RegisterSet::SP,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RegisterSet", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RegisterSet {
}

impl ::std::default::Default for RegisterSet {
    fn default() -> Self {
        RegisterSet::PC
    }
}

impl ::protobuf::reflect::ProtobufValue for RegisterSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nmain.proto\"\xa1\x01\n\x0bInstruction\x12$\n\x04code\x18\x01\x20\x01\
    (\x0e2\x10.InstructionCodeR\x04code\x12\x10\n\x03imm\x18\x02\x20\x01(\
    \x03R\x03imm\x12\x20\n\x04reg1\x18\x03\x20\x01(\x0e2\x0c.RegisterSetR\
    \x04reg1\x12\x20\n\x04reg2\x18\x04\x20\x01(\x0e2\x0c.RegisterSetR\x04reg\
    2\x12\x16\n\x06offset\x18\x05\x20\x01(\x03R\x06offset\"\t\n\x07Program*e\
    \n\x0fInstructionCode\x12\x07\n\x03MOV\x10\0\x12\x08\n\x04MOVI\x10\x01\
    \x12\x07\n\x03ADD\x10\x02\x12\x08\n\x04ADDI\x10\x03\x12\x06\n\x02JA\x10\
    \x19\x12\x07\n\x03JEQ\x10\x1a\x12\x08\n\x04JEQI\x10\x1b\x12\x08\n\x04HAL\
    T\x10c\x12\x07\n\x03INT\x10d*\x1f\n\rInterruptCode\x12\x0e\n\nPRINT_REG0\
    \x10\0*U\n\x0bRegisterSet\x12\x06\n\x02PC\x10\0\x12\x06\n\x02R0\x10\x01\
    \x12\x06\n\x02R1\x10\x02\x12\x06\n\x02R2\x10\x03\x12\x06\n\x02R3\x10\x04\
    \x12\x06\n\x02R4\x10\x05\x12\x06\n\x02R5\x10\x06\x12\x06\n\x02FP\x10\x07\
    \x12\x06\n\x02SP\x10\x08b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
