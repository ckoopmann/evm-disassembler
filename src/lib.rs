use std::collections::VecDeque;

use crate::types::Operation;
use crate::decode::decode_operation;

pub mod types;
mod decode;

pub fn disassemble(mut input: &str) -> Vec<Operation> {
    input = input.trim_start_matches("0x");
    let mut bytes = VecDeque::from(hex::decode(input).expect("Invalid hex string"));
    let mut operations = Vec::new();
    while bytes.len() > 0 {
        operations.push(decode_operation(&mut bytes));
    }
    return operations;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use crate::types::Opcode;

    fn pad_word(input: &str) -> [u8; 32] {
        let mut word = [0u8; 32];
        let padded_string = format!("{:0>64}", input);
        hex::decode_to_slice(padded_string, &mut word).expect("Invalid hex string");
        return word;
    }

    fn encode_op(opcode: &str, stack_input: Vec<[u8; 32]>) -> String {
        let mut bytes: String = opcode.to_owned();
        for word in stack_input {
            bytes += &hex::encode(word);
        }
        return bytes;
    }

    #[rstest]
    #[case(Opcode::STOP, "0x00", vec![])]
    #[case(Opcode::ADD, "0x01", vec!["100", "1234567"])]
    #[case(Opcode::MUL, "0x02", vec!["100", "1234567"])]
    #[case(Opcode::SUB, "0x03", vec!["100", "1234567"])]
    #[case(Opcode::DIV, "0x04", vec!["100", "1234567"])]
    #[case(Opcode::SDIV, "0x05", vec!["100", "1234567"])]
    #[case(Opcode::MOD, "0x06", vec!["100", "1234567"])]
    #[case(Opcode::SMOD, "0x07", vec!["100", "1234567"])]
    #[case(Opcode::ADDMOD, "0x08", vec!["100", "1234567", "11"])]
    #[case(Opcode::MULMOD, "0x09", vec!["100", "1234567", "11"])]
    fn decode_single_op(#[case] opcode: Opcode, #[case] encoded_opcode: &str, #[case] arguments: Vec<&str>) {
        let stack_input: Vec<[u8;32]> = arguments.iter().map(|arg| pad_word(arg)).collect();
        println!("stack_input: {:?}", stack_input);
        let encoded_op = encode_op(encoded_opcode, stack_input.clone());
        let result = disassemble(&encoded_op);
        assert_eq!(
            result,
            vec![Operation {
                opcode,
                stack_input,
            }]
        );
    }

    #[rstest]
    fn decode_stop_and_add() {
        let a = pad_word("100");
        let b = pad_word("1234567");
        let add_op = encode_op("01", vec![a, b]);
        let stop_op = encode_op("00", vec![]);
        let result = disassemble(&(add_op + &stop_op));
        assert_eq!(
            result,
            vec![Operation {
                opcode: Opcode::ADD,
                stack_input: vec![a, b],
            },
            Operation {
                opcode: Opcode::STOP,
                stack_input: vec![],
            }]
        );
    }
}
