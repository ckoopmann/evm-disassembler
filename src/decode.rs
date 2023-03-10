use crate::types::{Opcode, Operation};
use std::collections::VecDeque;

pub fn decode_operation(bytes: &mut VecDeque<u8>, cur_offset: u32) -> (Operation, u32) {
    let encoded_opcode = bytes.pop_front().expect("Unexpected end of input");
     let num_bytes = match encoded_opcode {
        0x60..=0x7f => {
            encoded_opcode - 0x5f
        },
        _ => 0
     };

    let mut new_offset = cur_offset + 1;
    let mut operation = Operation::new(Opcode::from_byte(encoded_opcode), cur_offset);
    if num_bytes > 0 {
            new_offset += num_bytes as u32;
            operation = operation.with_bytes(num_bytes, bytes)
    };
    return (operation, new_offset);
}
