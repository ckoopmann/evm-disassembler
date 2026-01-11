use crate::types::{Opcode, Operation};
use eyre::Result;

pub fn decode_operation(
    bytes: &mut dyn ExactSizeIterator<Item = u8>,
    cur_offset: u32,
    is_eof: bool,
) -> Result<(Operation, u32)> {
    let encoded_opcode = bytes.next().expect("Unexpected end of input");

    // Determine number of immediate bytes based on opcode
    // EOF opcodes with immediates are only decoded in EOF containers
    let num_bytes: u8 = match encoded_opcode {
        // PUSH1-PUSH32
        0x60..=0x7f => encoded_opcode - 0x5f,
        // EOF 2-byte immediates: DATALOADN, RJUMP, RJUMPI, CALLF, JUMPF
        0xd1 | 0xe0 | 0xe1 | 0xe3 | 0xe5 if is_eof => 2,
        // EOF 1-byte immediates: DUPN, SWAPN, EXCHANGE, EOFCREATE, TXCREATE, RETURNCONTRACT
        0xe6 | 0xe7 | 0xe8 | 0xec | 0xed | 0xee if is_eof => 1,
        // RJUMPV: variable length - 1 byte (max_index) + (max_index + 1) * 2 bytes
        #[allow(clippy::len_zero)]
        0xe2 if is_eof => {
            if bytes.len() > 0 {
                // Peek at max_index to calculate total immediate size
                // We need to consume the max_index byte and include it in the immediate
                let max_index = bytes.next().unwrap_or(0);
                let jump_table_size = (max_index as u16 + 1) * 2;
                // Return early with special handling for RJUMPV
                let new_offset = cur_offset + 2 + jump_table_size as u32;
                let opcode = Opcode::from_byte_eof(encoded_opcode);
                let mut operation = Operation::new(opcode, cur_offset);
                // Collect max_index + jump table bytes
                let mut input = vec![max_index];
                for _ in 0..jump_table_size {
                    if let Some(b) = bytes.next() {
                        input.push(b);
                    }
                }
                operation.input = input;
                return Ok((operation, new_offset));
            }
            0
        }
        _ => 0,
    };

    let mut new_offset = cur_offset + 1;
    let opcode = if is_eof {
        Opcode::from_byte_eof(encoded_opcode)
    } else {
        Opcode::from_byte(encoded_opcode)
    };
    let mut operation = Operation::new(opcode, cur_offset);
    if num_bytes > 0 {
        new_offset += num_bytes as u32;
        operation = operation.with_bytes(num_bytes, bytes)?
    };
    Ok((operation, new_offset))
}
