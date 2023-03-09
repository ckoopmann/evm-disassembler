use std::collections::VecDeque;
use crate::types::{Opcode, Operation};

pub fn decode_operation(bytes: &mut VecDeque<u8>) -> Operation {
    match bytes.pop_front() {
        None => panic!("Unexpected end of input"),
        Some(value) => match value {
            0x00 => Operation::new(Opcode::STOP),
            0x01 => Operation::new(Opcode::ADD).with_stack_input(2, bytes),
            0x02 => Operation::new(Opcode::MUL).with_stack_input(2, bytes),
            0x03 => Operation::new(Opcode::SUB).with_stack_input(2, bytes),
            0x04 => Operation::new(Opcode::DIV).with_stack_input(2, bytes),
            0x05 => Operation::new(Opcode::SDIV).with_stack_input(2, bytes),
            0x06 => Operation::new(Opcode::MOD).with_stack_input(2, bytes),
            0x07 => Operation::new(Opcode::SMOD).with_stack_input(2, bytes),
            0x08 => Operation::new(Opcode::ADDMOD).with_stack_input(3, bytes),
            0x09 => Operation::new(Opcode::MULMOD).with_stack_input(3, bytes),
            0x0A => Operation::new(Opcode::EXP).with_stack_input(2, bytes),
            _ => panic!("Invalid opcode: {}", value),
        },
    }
}

