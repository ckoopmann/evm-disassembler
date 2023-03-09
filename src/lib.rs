use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    ADDMOD,
    MULMOD,
    EXP,
    SIGNEXTEND,
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SHL,
    SHR,
    SAR,
    SHA3,
    ADDRESS,
    BALANCE,
    ORIGIN,
    CALLER,
    CALLVALUE,
    CALLDATALOAD,
    CALLDATASIZE,
    CALLDATACOPY,
    CODESIZE,
    CODECOPY,
    GASPRICE,
    EXTCODESIZE,
    EXTCODECOPY,
    RETURNDATASIZE,
    RETURNDATACOPY,
    EXTCODEHASH,
    BLOCKHASH,
    COINBASE,
    TIMESTAMP,
    NUMBER,
    DIFFICULTY,
    GASLIMIT,
    CHAINID,
    SELFBALANCE,
    BASEFEE,
    POP,
    MLOAD,
    MSTORE,
    MSTORE8,
    SLOAD,
    SSTORE,
    JUMP,
    JUMPI,
    PC,
    MSIZE,
    GAS,
    JUMPDEST,
    PUSH1,
    PUSH2,
    PUSH3,
    PUSH4,
    PUSH5,
    PUSH6,
    PUSH7,
    PUSH8,
    PUSH9,
    PUSH10,
    PUSH11,
    PUSH12,
    PUSH13,
    PUSH14,
    PUSH15,
    PUSH16,
    PUSH17,
    PUSH18,
    PUSH19,
    PUSH20,
    PUSH21,
    PUSH22,
    PUSH23,
    PUSH24,
    PUSH25,
    PUSH26,
    PUSH27,
    PUSH28,
    PUSH29,
    PUSH30,
    PUSH31,
    PUSH32,
    DUP1,
    DUP2,
    DUP3,
    DUP4,
    DUP5,
    DUP6,
    DUP7,
    DUP8,
    DUP9,
    DUP10,
    DUP11,
    DUP12,
    DUP13,
    DUP14,
    DUP15,
    DUP16,
    SWAP1,
    SWAP2,
    SWAP3,
    SWAP4,
    SWAP5,
    SWAP6,
    SWAP7,
    SWAP8,
    SWAP9,
    SWAP10,
    SWAP11,
    SWAP12,
    SWAP13,
    SWAP14,
    SWAP15,
    SWAP16,
    LOG0,
    LOG1,
    LOG2,
    LOG3,
    LOG4,
    CREATE,
    CALL,
    CALLCODE,
    RETURN,
    DELEGATECALL,
    CREATE2,
    STATICCALL,
    REVERT,
    INVALID,
    SELFDESTRUCT,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    opcode: Opcode,
    stack_input: Vec<[u8; 32]>,
}

impl Operation {
    pub fn new(opcode: Opcode) -> Self {
        Operation {
            opcode,
            stack_input: Vec::new(),
        }
    }

    pub fn with_stack_input(&self, num_words: u8, bytes: &mut VecDeque<u8>) -> Self {
        let stack_input = (0..num_words)
            .map(|_| {
                let mut word = [0u8; 32];
                word.copy_from_slice(&bytes.drain(0..32).collect::<Vec<u8>>());
                word
            })
            .collect();
        Operation {
            opcode: self.opcode,
            stack_input,
        }
    }
}

pub fn disassemble(mut input: &str) -> Vec<Operation> {
    input = input.trim_start_matches("0x");
    let mut bytes = VecDeque::from(hex::decode(input).expect("Invalid hex string"));
    let mut operations = Vec::new();
    while bytes.len() > 0 {
        operations.push(decode_operation(&mut bytes));
    }
    return operations;
}

fn decode_operation(bytes: &mut VecDeque<u8>) -> Operation {
    match bytes.pop_front() {
        None => panic!("Unexpected end of input"),
        Some(value) => match value {
            0x00 => Operation::new(Opcode::STOP),
            0x01 => Operation::new(Opcode::ADD).with_stack_input(2, bytes),
            0x02 => Operation::new(Opcode::MUL).with_stack_input(2, bytes),
            0x03 => Operation::new(Opcode::SUB).with_stack_input(2, bytes),
            0x04 => Operation::new(Opcode::DIV).with_stack_input(2, bytes),
            0x05 => Operation::new(Opcode::SDIV).with_stack_input(2, bytes),
            0x06 => Operation::new(Opcode::MOD).with_stack_input(2, bytes),
            0x07 => Operation::new(Opcode::SMOD).with_stack_input(2, bytes),
            0x08 => Operation::new(Opcode::ADDMOD).with_stack_input(3, bytes),
            0x09 => Operation::new(Opcode::MULMOD).with_stack_input(3, bytes),
            0x0A => Operation::new(Opcode::EXP).with_stack_input(2, bytes),
            _ => panic!("Invalid opcode: {}", value),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

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
