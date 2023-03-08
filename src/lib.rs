use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    Stop,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    opcode: Opcode,
    stack_input: Vec<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_stop() {
        let result = disassemble("0x00");
        assert_eq!(result, vec![Operation {
            opcode: Opcode::Stop,
            stack_input: vec![],
        }]);
    }
}
