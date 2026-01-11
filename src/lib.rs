//! Disassemble evm bytecode into individual instructions.
//!
//! This crate provides a simple interface for disassembling evm bytecode into individual
//! instructions / opcodes.
//! It supports both hex encoded strings as well as a vector of bytes as input
//! Additionally it provides a method to format the disassembled instructions into a human readable
//! format identical to that of the [pyevmasm](https://github.com/crytic/pyevmasm) library
//!
//! ```rust
//! use evm_disassembler::{disassemble_str, disassemble_bytes, format_operations};
//!    
//! let bytecode = "60606040526040";
//! let instructions = disassemble_str(bytecode).unwrap();
//! // Will print:
//! // 00000000: PUSH1 0x60
//! // 00000002: PUSH1 0x40
//! // 00000004: MSTORE
//! // 00000005: PUSH1 0x40
//! println!("{}", format_operations(instructions).unwrap());
//!
//! let bytes = hex::decode(bytecode).unwrap();
//! let instructions_from_bytes = disassemble_bytes(bytes).unwrap();
//! println!("{}", format_operations(instructions_from_bytes).unwrap());
//!
//! ```
#![warn(missing_docs)]
use crate::decode::decode_operation;
use std::fmt::Write;

use eyre::Result;

mod decode;

pub mod types;
pub use types::{Opcode, Operation};

#[cfg(test)]
mod test_utils;

/// Disassemble a hex encoded string into a vector of instructions / operations
///
/// # Arguments
/// - `input` - A hex encoded string representing the bytecode to disassemble
///
/// # Examples
///
/// ```rust
/// use evm_disassembler::disassemble_str;
///
/// let bytecode = "0x608060405260043610603f57600035";
/// let instructions = disassemble_str(bytecode).unwrap();
/// ```
pub fn disassemble_str(input: &str) -> Result<Vec<Operation>> {
    let input = input.trim_start_matches("0x");
    let bytes = hex::decode(input)?;
    disassemble_bytes(bytes)
}

/// Disassemble a vector of bytes into a vector of decoded Operations
///
/// Will stop disassembling when it encounters a push instruction with a size greater than
/// remaining bytes in the input.
///
/// Automatically detects EOF containers (starting with 0xef00) and decodes EOF-specific
/// opcodes only when appropriate.
///
/// # Arguments
/// - `bytes` - A vector of bytes representing the encoded bytecode
///
/// # Examples
///
/// ```rust
/// use evm_disassembler::disassemble_bytes;
///
/// let bytecode = "608060405260043610603f57600035";
/// let bytes = hex::decode(bytecode).unwrap();
/// let instructions_from_bytes = disassemble_bytes(bytes).unwrap();
/// ```
pub fn disassemble_bytes(bytes: Vec<u8>) -> Result<Vec<Operation>> {
    // Detect EOF container: starts with 0xef00
    let is_eof = bytes.len() >= 2 && bytes[0] == 0xef && bytes[1] == 0x00;

    let mut operations = Vec::new();
    let mut new_operation: Operation;
    let mut offset = 0;
    let mut bytes_iter = bytes.into_iter();
    while bytes_iter.len() > 0 {
        (new_operation, offset) = match decode_operation(&mut bytes_iter, offset, is_eof) {
            Ok((operation, new_offset)) => (operation, new_offset),
            Err(e) => {
                println!("Stop decoding at offset {offset} due to error : {e}");
                break;
            }
        };
        operations.push(new_operation);
    }
    Ok(operations)
}

/// Converts a vector of decoded operations into a human readable formatted string
///
/// Operations are formatted on individual lines with the following format:
/// `{offset}: {opcode} {bytes}`
///
/// - `offset` - The offset of the operation in the bytecode (as hex)
/// - `opcode` - The respective opcode (i.e. "PUSH1", "ADD")
/// - `bytes` - Additional bytes that are part of the operation (only for "PUSH" instructions)
///
/// # Arguments
/// - `operations` - A vector of decoded operations as returned by `disassemble_str` or
///   `disassemble_bytes`
///
/// # Examples
/// ```rust
/// use evm_disassembler::{disassemble_str, format_operations};
///
/// let bytecode = "0x608060405260043610603f57600035";
/// let instructions = disassemble_str(bytecode).unwrap();
/// println!("{}", format_operations(instructions).unwrap());
/// ```
pub fn format_operations(operations: Vec<Operation>) -> Result<String> {
    let mut formatted = String::new();
    for operation in operations.iter() {
        writeln!(formatted, "{operation:?}")?;
    }
    Ok(formatted)
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
        let operations = disassemble_bytes(code).expect("Unable to disassemble code");
        assert_eq!(operations.len(), expected_length);
        for (opcode, expected_position) in expected_opcodes.iter() {
            assert_eq!(operations[*expected_position].opcode, *opcode);
        }
    }

    #[rstest]
    #[case("0xDef1C0ded9bec7F1a1670819833240f027b25EfF")] // UniswapV3 Router
    #[case("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")] // Weth
    #[case("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")] // ZeroEx Proxy
    #[case("0x00000000006c3852cbEf3e08E8dF289169EdE581")] // Seaport
    fn decode_code_from_file(#[case] address: &str) {
        let mut code = fs::read_to_string(format!("testdata/{address}_encoded.txt"))
            .expect("Unable to read encoded file");
        let decoded_reference = fs::read_to_string(format!("testdata/{address}_decoded.txt"))
            .expect("No reference file");
        code.pop();

        let operations = disassemble_str(&code).expect("Unable to decode");
        assert!(!operations.is_empty());
        let formatted_operations = format_operations(operations);
        for (i, line) in formatted_operations
            .expect("failed to format")
            .lines()
            .enumerate()
        {
            assert_eq!(line, decoded_reference.lines().nth(i).unwrap());
        }
        println!("Decoded output from contract {address} matches reference");
    }

    #[rstest]
    fn decode_preamble() {
        let code = "608060405260043610603f57600035";
        let operations = disassemble_str(code).expect("Unable to decode");
        assert_eq!(operations.len(), 10);
    }

    #[rstest]
    fn decode_preamble_from_bytes() {
        let bytes = hex::decode("608060405260043610603f57600035").unwrap();
        let operations = disassemble_bytes(bytes).expect("Unable to decode");
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
        let result = disassemble_str(encoded_opcode).expect("Unable to decode");
        assert_eq!(result, vec![Operation::new(opcode, 0)]);
    }

    #[rstest]
    fn decode_stop_and_add() {
        let add_op = "01";
        let stop_op = "00";
        let result = disassemble_str(&(add_op.to_owned() + stop_op)).expect("Unable to decode");
        assert_eq!(
            result,
            vec![
                Operation::new(Opcode::ADD, 0),
                Operation::new(Opcode::STOP, 1),
            ]
        );
    }

    // EOF container tests
    // EOF format: ef0001 [header] [types] [code] [data]
    // Header: 01 XXXX (type section) 02 YYYY ZZZZ (code section) 04 WWWW (data section) 00 (terminator)

    #[rstest]
    fn test_eof_detection() {
        // This should be detected as EOF (starts with ef00)
        // Minimal EOF: ef0001 01 0004 02 0001 0001 04 0000 00 [types: 00000000] [code: 00]
        let eof_bytecode = "ef00010100040200010001040000000000000000";
        let ops = disassemble_str(eof_bytecode).expect("Should decode EOF");
        // In EOF mode, the header bytes are decoded as opcodes (some will be INVALID)
        // The important thing is it doesn't crash
        assert!(!ops.is_empty());
    }

    #[rstest]
    fn test_eof_with_rjump() {
        // EOF with RJUMP instruction in code section
        // Header: ef0001 01 0004 02 0001 0003 04 0000 00
        // Types: 00 80 00 01 (0 inputs, non-returning, max stack 1)
        // Code: e0 00 00 (RJUMP with offset 0)
        let eof_bytecode = "ef000101000402000100030400000000800001e00000";
        let ops = disassemble_str(eof_bytecode).expect("Should decode EOF");
        let formatted = format_operations(ops).unwrap();
        println!("EOF with RJUMP:\n{}", formatted);
        // Should contain RJUMP since this is EOF format
        assert!(
            formatted.contains("RJUMP"),
            "Should decode RJUMP in EOF container"
        );
    }

    #[rstest]
    fn test_eof_with_callf() {
        // EOF with CALLF and RETF
        // Header: ef0001 01 0008 02 0002 0003 0001 04 0000 00
        // Types section (8 bytes for 2 functions):
        //   00 80 00 01 - func 0: 0 inputs, non-returning, max stack 1
        //   00 00 00 00 - func 1: 0 inputs, 0 outputs, max stack 0
        // Code section 0 (3 bytes): e3 0001 - CALLF 1
        // Code section 1 (1 byte): e4 - RETF
        let eof_bytecode = "ef00010100080200020003000104000000008000010000000000e30001e4";
        let ops = disassemble_str(eof_bytecode).expect("Should decode EOF");
        let formatted = format_operations(ops).unwrap();
        println!("EOF with CALLF:\n{}", formatted);
        assert!(
            formatted.contains("CALLF"),
            "Should decode CALLF in EOF container"
        );
    }

    #[rstest]
    fn test_legacy_bytecode_no_eof_opcodes() {
        // Legacy bytecode containing byte 0xe0 should NOT decode as RJUMP
        // This tests that EOF opcodes are only decoded in EOF containers
        let legacy_bytecode = "60e0"; // PUSH1 0xe0
        let ops = disassemble_str(legacy_bytecode).expect("Should decode legacy");
        let formatted = format_operations(ops).unwrap();
        println!("Legacy bytecode:\n{}", formatted);
        // Should be PUSH1, not RJUMP
        assert!(formatted.contains("PUSH1"), "Should decode as PUSH1");
        assert!(
            !formatted.contains("RJUMP"),
            "Should NOT decode as RJUMP in legacy"
        );
    }

    #[rstest]
    fn test_legacy_with_eof_like_bytes() {
        // Legacy bytecode with bytes that would be EOF opcodes if in EOF mode
        // 0xe7 would be SWAPN in EOF, but should be INVALID in legacy
        let legacy_bytecode = "e7";
        let ops = disassemble_str(legacy_bytecode).expect("Should decode legacy");
        assert_eq!(
            ops[0].opcode,
            Opcode::INVALID,
            "0xe7 should be INVALID in legacy"
        );
    }

    #[rstest]
    fn test_real_eof_contract_from_solidity() {
        // Real EOF bytecode compiled from Solidity with --evm-version osaka --eofVersion 1
        // Contract: SimpleEOF with setValue, getValue, add functions
        let eof = "ef000101009c020027004b0004000400030003000b00010006000d001c00020001000b00050007001c000c000800120003001b001c001d0003000200030007000500090003000f0001000a0001000f00050013001300080400430000800003010100020001000100800002008000020200000301010001020000020201000500800004020100020101000102010003020100020001000200800004010000020201000202010005010100020080000302020005008000040080000200010001010100020101000101010001000100010101000202010004010100010101000101010001020000030100000200800002020100030201000360806040526004361015e10003e500175f35e3000180632096525514e1002980633fa4f24514e1001c80635524107714e1000f63771602f714e10003e0ffcee50016e50014e5000fe5000960e01ce4604051e45f80fd5f80fd5f910312e10001e4e50004e4e300069052e4905f60208301920190e30007e434e10015366004e30005e3001ce30002809181e300080390f3e500031ce4e490600802e3000ae3000be454e3000ce45f5f90e3000de434e10015366004e30005e3000ee30002809181e300080390f3e5000380e3000603e10001e45f80fd90503580e30010e4602081830312e100065f01e30011e4e500045f01e434e10014366004e30012e30023e300028080e300130390f3e5000390604082820312e1000f805f8301e3001191602001e30011e4e5000434e10016366004e3001590e30026e30002809181e300080390f3e500035f80fd5fe45f1ce4e30019e3000be454e3001ae4e30018505fe3001be45f1be4905f1990e3001d91811916911617e4e4e30006e3001fe30006e4e490e30020e300218154e3001e9055e45fe30022e4634e487b7160e01b5f52601160045260245ffde3000690e300068101809111e10001e4e50024e3001850e30025e4";

        let ops = disassemble_str(eof).expect("Should decode real EOF contract");
        let formatted = format_operations(ops).unwrap();

        println!("\n=== Real EOF Contract Disassembly ===");
        println!("(Header bytes decoded as opcodes, code section follows)\n");

        // Show lines containing EOF opcodes
        println!("Lines containing EOF opcodes:");
        for line in formatted.lines() {
            if line.contains("RJUMP")
                || line.contains("CALLF")
                || line.contains("RETF")
                || line.contains("JUMPF")
                || line.contains("DATALOAD")
            {
                println!("{}", line);
            }
        }
        println!();

        // Verify EOF opcodes are present
        assert!(formatted.contains("RJUMPI"), "Should contain RJUMPI");
        assert!(formatted.contains("JUMPF"), "Should contain JUMPF");
        assert!(formatted.contains("CALLF"), "Should contain CALLF");
        assert!(formatted.contains("RETF"), "Should contain RETF");
    }
}
