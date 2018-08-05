use protos::main::{InterruptCode};

pub fn get_interrupt_enum(int_code: i64) -> InterruptCode {
    match int_code {
        0 => InterruptCode::PRINT_REG0,
        _ => panic!("unknown interrupt"),
    }
}
