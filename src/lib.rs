use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    Stop,
    Add,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    opcode: Opcode,
    stack_input: Vec<[u8; 32]>,
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
            0x00 => decode_stop(),
            0x01 => decode_add(bytes),
            _ => panic!("Invalid opcode: {}", value),
        },
    }
}

fn decode_stop() -> Operation {
    Operation {
        opcode: Opcode::Stop,
        stack_input: Vec::new(),
    }
}
fn decode_add(bytes: &mut VecDeque<u8>) -> Operation {
    Operation {
        opcode: Opcode::Add,
        stack_input: vec![pop_word(bytes), pop_word(bytes)],
    }
}

fn pop_word(bytes: &mut VecDeque<u8>) -> [u8; 32] {
    let mut word = [0u8; 32];
    for i in 0..32 {
        word[i] = bytes.pop_front().expect("Unexpected end of input in word");
    }
    return word;
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn decode_stop() {
        let result = disassemble("0x00");
        assert_eq!(
            result,
            vec![Operation {
                opcode: Opcode::Stop,
                stack_input: vec![],
            }]
        );
    }

    #[test]
    fn decode_add() {
        let a = pad_word("100");
        let b = pad_word("1234567");
        let encoded_op = encode_op("0x01", vec![a, b]);
        println!("encoded_op: {}", encoded_op);
        let result = disassemble(&encoded_op);
        assert_eq!(
            result,
            vec![Operation {
                opcode: Opcode::Add,
                stack_input: vec![a, b],
            }]
        );
    }

    #[test]
    fn decode_stop_and_add() {
        let a = pad_word("100");
        let b = pad_word("1234567");
        let add_op = encode_op("01", vec![a, b]);
        let stop_op = encode_op("00", vec![]);
        let result = disassemble(&(add_op + &stop_op));
        assert_eq!(
            result,
            vec![Operation {
                opcode: Opcode::Add,
                stack_input: vec![a, b],
            },
            Operation {
                opcode: Opcode::Stop,
                stack_input: vec![],
            }]
        );
    }
}
