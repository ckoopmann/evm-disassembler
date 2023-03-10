use std::collections::VecDeque;

use crate::decode::decode_operation;
use crate::types::Operation;

use eyre::{eyre, Result};

mod decode;
pub mod types;

#[cfg(test)]
mod test_utils;

pub fn disassemble(input: &str) -> Result<Vec<Operation>> {
    let input = input.trim_start_matches("0x").to_owned();
    // TODO: Potentially remove
    if input.len() % 2 != 0 {
        return Err(eyre!("Odd number of hex characters"));
    }
    let mut bytes = VecDeque::from(hex::decode(input).expect("Invalid hex string"));
    let mut operations = Vec::new();
    let mut new_operation: Operation;
    let mut offset = 0;
    while !bytes.is_empty() {
        (new_operation, offset) = match decode_operation(&mut bytes, offset) {
            Ok((operation, new_offset)) => (operation, new_offset),
            Err(e) => {
                println!("Stop decoding at offset {} due to error : {}", offset, e);
                break;
            }
        };
        operations.push(new_operation);
    }
    Ok(operations)
}

pub fn format_operations(operations: Vec<Operation>) -> String {
    let mut formatted = String::new();
    for operation in operations {
        formatted = format!("{}{:?}\n", formatted, operation);
    }
    return formatted;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_contract_code;
    use crate::types::Opcode;
    use rstest::*;
    use std::fs;

    #[rstest]
    #[case("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", 1577, vec![(Opcode::DUP7, 1000), (Opcode::EXTCODECOPY, 1563)])]
    #[tokio::test]
    async fn decode_code_from_rpc_provider(
        #[case] address: &str,
        #[case] expected_length: usize,
        #[case] expected_opcodes: Vec<(Opcode, usize)>,
    ) {
        let code = get_contract_code(address).await;
        let operations = disassemble(&code).expect("Unable to disassemble code");
        assert_eq!(operations.len(), expected_length);
        for (opcode, expected_position) in expected_opcodes.iter() {
            assert_eq!(operations[*expected_position].opcode, *opcode);
        }
    }

    #[rstest]
    #[case("0xDef1C0ded9bec7F1a1670819833240f027b25EfF")]  // UniswapV3 Router
    #[case("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")]  // Weth
    #[case("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")]  // ZeroEx Proxy
    #[case("0x00000000006c3852cbEf3e08E8dF289169EdE581")]  // Seaport
    fn decode_code_from_file(#[case] address: &str) {
        let mut code = fs::read_to_string(format!("testdata/{}_encoded.txt", address)).expect("Unable to read encoded file");
        let decoded_reference =
            fs::read_to_string(format!("testdata/{}_decoded.txt", address)).expect("No reference file");
        code.pop();

        let operations = disassemble(&code).expect("Unable to decode");
        assert!(!operations.is_empty());
        let formatted_operations = format_operations(operations);
        for (i, line) in formatted_operations.lines().enumerate() {
            assert_eq!(line, decoded_reference.lines().nth(i).unwrap());
        }
        println!(
            "Decoded output from contract {} matches reference",
            address
        );
    }

    #[rstest]
    fn decode_preamble() {
        let code = "608060405260043610603f57600035";
        let operations = disassemble(code).expect("Unable to decode");
        assert_eq!(operations.len(), 10);
    }

    #[rstest]
    #[case(Opcode::STOP, "0x00")]
    #[case(Opcode::ADD, "0x01")]
    #[case(Opcode::MUL, "0x02")]
    #[case(Opcode::SUB, "0x03")]
    #[case(Opcode::DIV, "0x04")]
    #[case(Opcode::SDIV, "0x05")]
    #[case(Opcode::MOD, "0x06")]
    #[case(Opcode::SMOD, "0x07")]
    #[case(Opcode::ADDMOD, "0x08")]
    #[case(Opcode::MULMOD, "0x09")]
    fn decode_single_op(#[case] opcode: Opcode, #[case] encoded_opcode: &str) {
        let result = disassemble(encoded_opcode).expect("Unable to decode");
        assert_eq!(result, vec![Operation::new(opcode, 0)]);
    }

    #[rstest]
    fn decode_stop_and_add() {
        let add_op = "01";
        let stop_op = "00";
        let result = disassemble(&(add_op.to_owned() + stop_op)).expect("Unable to decode");
        assert_eq!(
            result,
            vec![
                Operation::new(Opcode::ADD, 0),
                Operation::new(Opcode::STOP, 1),
            ]
        );
    }
}
