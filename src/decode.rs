use crate::types::{Opcode, Operation};
use std::collections::VecDeque;
use eyre::Result;

pub fn decode_operation(bytes: &mut VecDeque<u8>, cur_offset: u32) -> Result<(Operation, u32)> {
    let encoded_opcode = bytes.pop_front().expect("Unexpected end of input");
     let num_bytes = match encoded_opcode {
        0x60..=0x7f => {
            encoded_opcode - 0x5f
        },
        _ => 0
     };

    let mut new_offset = cur_offset + 1;
    let opcode = Opcode::from_byte(encoded_opcode);
    let mut operation = Operation::new(opcode, cur_offset);
    if num_bytes > 0 {
            new_offset += num_bytes as u32;
            operation = operation.with_bytes(num_bytes, bytes)?
    };
    Ok((operation, new_offset))
}
