use std::collections::VecDeque;

use crate::types::Operation;
use crate::decode::decode_operation;

pub mod types;
mod decode;

#[cfg(test)]
mod test_utils;

pub fn disassemble(input: &str) -> Vec<Operation> {
    let mut input = input.trim_start_matches("0x").to_owned();
    if input.len() % 2 != 0 {
        input = "0".to_owned() + &input;
    }
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
    use crate::test_utils::get_contract_code;

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
    #[case("testdata/usdc_encoded.txt")]
    fn decode_transaction_from_file(#[case] address: &str) {
        let mut code = std::fs::read_to_string(address).unwrap();
        // Remove trailing \n
        code.pop();
        let operations = disassemble(&code);
        assert!(operations.len() > 0);
    }

    #[rstest]
    fn decode_preamble() {
        let code = "608060405260043610603f57600035";
        let operations = disassemble(&code);
        assert_eq!(operations.len(), 10);
        println!("Decoded preamble: {:#?}", operations);
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
        let result = disassemble(&encoded_opcode);
        assert_eq!(
            result,
            vec![Operation::new(opcode)]
        );
    }

    #[rstest]
    fn decode_stop_and_add() {
        let add_op = "01";
        let stop_op = "00";
        let result = disassemble(&(add_op.to_owned() + &stop_op));
        assert_eq!(
            result,
            vec![
            Operation::new(Opcode::ADD),
            Operation::new(Opcode::STOP),
            ]
        );
    }
}
