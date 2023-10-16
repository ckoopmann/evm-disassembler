use crate::types::{Opcode, Operation};
use eyre::Result;

pub fn decode_operation(
    bytes: &mut dyn ExactSizeIterator<Item = u8>,
    cur_offset: u32,
) -> Result<(Operation, u32)> {
    let encoded_opcode = bytes.next().expect("Unexpected end of input");
    let num_bytes = match encoded_opcode {
        _ if encoded_opcode <= Opcode::PUSH32 as u8 && encoded_opcode >= Opcode::PUSH1 as u8 => {
            encoded_opcode + 1 - Opcode::PUSH1 as u8
        }
        _ => 0,
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
