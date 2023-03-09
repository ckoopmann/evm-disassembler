use crate::types::{Opcode, Operation};
use std::collections::VecDeque;

pub fn decode_operation(bytes: &mut VecDeque<u8>) -> Operation {
    let encoded_opcode = bytes.pop_front().expect("Unexpected end of input");
    let opcode = Opcode::from_byte(encoded_opcode);
    let operation = match encoded_opcode {
        0x60..=0x7f => {
            let num_bytes = encoded_opcode - 0x5f;
            Operation::new(opcode).with_bytes(num_bytes, bytes)
        },
        _ => Operation::new(opcode),
    };
    println!("Decoded operation: {:#?}", operation);
    operation
}

