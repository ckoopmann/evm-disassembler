use std::collections::VecDeque;

use crate::decode::decode_operation;
use crate::types::Operation;

mod decode;
pub mod types;

#[cfg(test)]
mod test_utils;

pub fn disassemble(input: &str) -> Vec<Operation> {
    let mut input = input.trim_start_matches("0x").to_owned();
    // TODO: Potentially remove
    if input.len() % 2 != 0 {
        println!(
            "Odd number of bytes in input {}, adding 0 to front",
            input.len()
        );
        input = "0".to_owned() + &input;
    }
    let mut bytes = VecDeque::from(hex::decode(input).expect("Invalid hex string"));
    let mut operations = Vec::new();
    let mut new_operation: Operation;
    let mut offset = 0;
    while !bytes.is_empty() {
        (new_operation, offset) = decode_operation(&mut bytes, offset);
        operations.push(new_operation);
    }
    operations
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
        let operations = disassemble(&code);
        assert_eq!(operations.len(), expected_length);
        for (opcode, expected_position) in expected_opcodes.iter() {
            assert_eq!(operations[*expected_position].opcode, *opcode);
        }

        println!("Decoded from rpc:\n{}", format_operations(operations));
    }

    #[rstest]
    fn decode_code_from_file() {
        let encoded_files = fs::read_dir("testdata");
        for encoded_file in encoded_files
            .unwrap()
            .filter(|f| f.is_ok())
            .map(|f| f.unwrap().path())
            .filter(|f| {
                f.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains("_encoded.txt")
            })
        {
            let decoded_file = encoded_file.with_file_name(
                encoded_file
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace("_encoded.txt", "_decoded.txt"),
            );
            if !decoded_file.exists() {
                println!(
                    "Skipping {} because no corresponding decoded file  at {} exists",
                    encoded_file.to_str().unwrap(),
                    decoded_file.to_str().unwrap()
                );
                continue;
            }

            println!("Decoding {}", encoded_file.to_str().unwrap());
            let mut code = fs::read_to_string(&encoded_file).unwrap();
            let decoded_reference = std::fs::read_to_string(decoded_file).unwrap();
            // Remove trailing \n
            code.pop();

            let operations = disassemble(&code);
            assert!(!operations.is_empty());
            let formatted_operations = format_operations(operations);
            for (i, line) in formatted_operations.lines().enumerate() {
                assert_eq!(line, decoded_reference.lines().nth(i).unwrap());
            }
            println!("Decoded output from file {} matches reference", encoded_file.to_str().unwrap());
        }
    }

    #[rstest]
    fn decode_preamble() {
        let code = "608060405260043610603f57600035";
        let operations = disassemble(code);
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
        let result = disassemble(encoded_opcode);
        assert_eq!(result, vec![Operation::new(opcode, 0)]);
    }

    #[rstest]
    fn decode_stop_and_add() {
        let add_op = "01";
        let stop_op = "00";
        let result = disassemble(&(add_op.to_owned() + stop_op));
        assert_eq!(
            result,
            vec![
                Operation::new(Opcode::ADD, 0),
                Operation::new(Opcode::STOP, 1),
            ]
        );
    }
}
