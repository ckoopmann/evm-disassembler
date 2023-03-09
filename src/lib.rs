use std::collections::VecDeque;

use crate::types::Operation;
use crate::decode::decode_operation;

pub mod types;
mod decode;

#[cfg(test)]
mod test_utils;

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
    use crate::test_utils::{pad_word, encode_op, get_contract_code};

    #[rstest]
    #[case("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")]
    #[tokio::test]
    async fn decode_transaction(#[case] address: &str) {
        let code = get_contract_code(address).await;
        let operations = disassemble(&code);
        assert!(operations.len() > 0);
    }

    #[rstest]
    #[case("testdata/weth_encoded.txt")]
    fn decode_transaction_from_file(#[case] address: &str) {
        let code = std::fs::read_to_string(address).unwrap();
        let operations = disassemble(&code);
        assert!(operations.len() > 0);
    }

    #[rstest]
    fn decode_preamble() {
        let code = "608060405260043610603f57600035";
        let operations = disassemble(&code);
        assert_eq!(operations.len(), 10);
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
