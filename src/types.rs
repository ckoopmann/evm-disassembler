//! Output types for Operation and Opcode
use eyre::{eyre, Result};
use std::fmt;

/// A single EVM operation
///
/// For additional information on each operation see: https://www.evm.codes/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Opcode {
    STOP = 0,
    ADD = 1,
    MUL = 2,
    SUB = 3,
    DIV = 4,
    SDIV = 5,
    MOD = 6,
    SMOD = 7,
    ADDMOD = 8,
    MULMOD = 9,
    EXP = 10,
    SIGNEXTEND = 11,
    LT = 16,
    GT = 17,
    SLT = 18,
    SGT = 19,
    EQ = 20,
    ISZERO = 21,
    AND = 22,
    OR = 23,
    XOR = 24,
    NOT = 25,
    BYTE = 26,
    SHL = 27,
    SHR = 28,
    SAR = 29,
    SHA3 = 32,
    ADDRESS = 48,
    BALANCE = 49,
    ORIGIN = 50,
    CALLER = 51,
    CALLVALUE = 52,
    CALLDATALOAD = 53,
    CALLDATASIZE = 54,
    CALLDATACOPY = 55,
    CODESIZE = 56,
    CODECOPY = 57,
    GASPRICE = 58,
    EXTCODESIZE = 59,
    EXTCODECOPY = 60,
    RETURNDATASIZE = 61,
    RETURNDATACOPY = 62,
    EXTCODEHASH = 63,
    BLOCKHASH = 64,
    COINBASE = 65,
    TIMESTAMP = 66,
    NUMBER = 67,
    DIFFICULTY = 68,
    GASLIMIT = 69,
    CHAINID = 70,
    SELFBALANCE = 71,
    BASEFEE = 72,
    POP = 80,
    MLOAD = 81,
    MSTORE = 82,
    MSTORE8 = 83,
    SLOAD = 84,
    SSTORE = 85,
    JUMP = 86,
    JUMPI = 87,
    PC = 88,
    MSIZE = 89,
    GAS = 90,
    JUMPDEST = 91,
    PUSH0 = 95,
    PUSH1 = 96,
    PUSH2 = 97,
    PUSH3 = 98,
    PUSH4 = 99,
    PUSH5 = 100,
    PUSH6 = 101,
    PUSH7 = 102,
    PUSH8 = 103,
    PUSH9 = 104,
    PUSH10 = 105,
    PUSH11 = 106,
    PUSH12 = 107,
    PUSH13 = 108,
    PUSH14 = 109,
    PUSH15 = 110,
    PUSH16 = 111,
    PUSH17 = 112,
    PUSH18 = 113,
    PUSH19 = 114,
    PUSH20 = 115,
    PUSH21 = 116,
    PUSH22 = 117,
    PUSH23 = 118,
    PUSH24 = 119,
    PUSH25 = 120,
    PUSH26 = 121,
    PUSH27 = 122,
    PUSH28 = 123,
    PUSH29 = 124,
    PUSH30 = 125,
    PUSH31 = 126,
    PUSH32 = 127,
    DUP1 = 128,
    DUP2 = 129,
    DUP3 = 130,
    DUP4 = 131,
    DUP5 = 132,
    DUP6 = 133,
    DUP7 = 134,
    DUP8 = 135,
    DUP9 = 136,
    DUP10 = 137,
    DUP11 = 138,
    DUP12 = 139,
    DUP13 = 140,
    DUP14 = 141,
    DUP15 = 142,
    DUP16 = 143,
    SWAP1 = 144,
    SWAP2 = 145,
    SWAP3 = 146,
    SWAP4 = 147,
    SWAP5 = 148,
    SWAP6 = 149,
    SWAP7 = 150,
    SWAP8 = 151,
    SWAP9 = 152,
    SWAP10 = 153,
    SWAP11 = 154,
    SWAP12 = 155,
    SWAP13 = 156,
    SWAP14 = 157,
    SWAP15 = 158,
    SWAP16 = 159,
    LOG0 = 160,
    LOG1 = 161,
    LOG2 = 162,
    LOG3 = 163,
    LOG4 = 164,
    CREATE = 240,
    CALL = 241,
    CALLCODE = 242,
    RETURN = 243,
    DELEGATECALL = 244,
    CREATE2 = 245,
    STATICCALL = 250,
    REVERT = 253,
    INVALID = 254,
    SELFDESTRUCT = 255,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
            x if x == Opcode::STOP as u8 => Ok(Opcode::STOP),
            x if x == Opcode::ADD as u8 => Ok(Opcode::ADD),
            x if x == Opcode::MUL as u8 => Ok(Opcode::MUL),
            x if x == Opcode::SUB as u8 => Ok(Opcode::SUB),
            x if x == Opcode::DIV as u8 => Ok(Opcode::DIV),
            x if x == Opcode::SDIV as u8 => Ok(Opcode::SDIV),
            x if x == Opcode::MOD as u8 => Ok(Opcode::MOD),
            x if x == Opcode::SMOD as u8 => Ok(Opcode::SMOD),
            x if x == Opcode::ADDMOD as u8 => Ok(Opcode::ADDMOD),
            x if x == Opcode::MULMOD as u8 => Ok(Opcode::MULMOD),
            x if x == Opcode::EXP as u8 => Ok(Opcode::EXP),
            x if x == Opcode::SIGNEXTEND as u8 => Ok(Opcode::SIGNEXTEND),
            x if x == Opcode::LT as u8 => Ok(Opcode::LT),
            x if x == Opcode::GT as u8 => Ok(Opcode::GT),
            x if x == Opcode::SLT as u8 => Ok(Opcode::SLT),
            x if x == Opcode::SGT as u8 => Ok(Opcode::SGT),
            x if x == Opcode::EQ as u8 => Ok(Opcode::EQ),
            x if x == Opcode::ISZERO as u8 => Ok(Opcode::ISZERO),
            x if x == Opcode::AND as u8 => Ok(Opcode::AND),
            x if x == Opcode::OR as u8 => Ok(Opcode::OR),
            x if x == Opcode::XOR as u8 => Ok(Opcode::XOR),
            x if x == Opcode::NOT as u8 => Ok(Opcode::NOT),
            x if x == Opcode::BYTE as u8 => Ok(Opcode::BYTE),
            x if x == Opcode::SHL as u8 => Ok(Opcode::SHL),
            x if x == Opcode::SHR as u8 => Ok(Opcode::SHR),
            x if x == Opcode::SAR as u8 => Ok(Opcode::SAR),
            x if x == Opcode::SHA3 as u8 => Ok(Opcode::SHA3),
            x if x == Opcode::ADDRESS as u8 => Ok(Opcode::ADDRESS),
            x if x == Opcode::BALANCE as u8 => Ok(Opcode::BALANCE),
            x if x == Opcode::ORIGIN as u8 => Ok(Opcode::ORIGIN),
            x if x == Opcode::CALLER as u8 => Ok(Opcode::CALLER),
            x if x == Opcode::CALLVALUE as u8 => Ok(Opcode::CALLVALUE),
            x if x == Opcode::CALLDATALOAD as u8 => Ok(Opcode::CALLDATALOAD),
            x if x == Opcode::CALLDATASIZE as u8 => Ok(Opcode::CALLDATASIZE),
            x if x == Opcode::CALLDATACOPY as u8 => Ok(Opcode::CALLDATACOPY),
            x if x == Opcode::CODESIZE as u8 => Ok(Opcode::CODESIZE),
            x if x == Opcode::CODECOPY as u8 => Ok(Opcode::CODECOPY),
            x if x == Opcode::GASPRICE as u8 => Ok(Opcode::GASPRICE),
            x if x == Opcode::EXTCODESIZE as u8 => Ok(Opcode::EXTCODESIZE),
            x if x == Opcode::EXTCODECOPY as u8 => Ok(Opcode::EXTCODECOPY),
            x if x == Opcode::RETURNDATASIZE as u8 => Ok(Opcode::RETURNDATASIZE),
            x if x == Opcode::RETURNDATACOPY as u8 => Ok(Opcode::RETURNDATACOPY),
            x if x == Opcode::EXTCODEHASH as u8 => Ok(Opcode::EXTCODEHASH),
            x if x == Opcode::BLOCKHASH as u8 => Ok(Opcode::BLOCKHASH),
            x if x == Opcode::COINBASE as u8 => Ok(Opcode::COINBASE),
            x if x == Opcode::TIMESTAMP as u8 => Ok(Opcode::TIMESTAMP),
            x if x == Opcode::NUMBER as u8 => Ok(Opcode::NUMBER),
            x if x == Opcode::DIFFICULTY as u8 => Ok(Opcode::DIFFICULTY),
            x if x == Opcode::GASLIMIT as u8 => Ok(Opcode::GASLIMIT),
            x if x == Opcode::CHAINID as u8 => Ok(Opcode::CHAINID),
            x if x == Opcode::SELFBALANCE as u8 => Ok(Opcode::SELFBALANCE),
            x if x == Opcode::BASEFEE as u8 => Ok(Opcode::BASEFEE),
            x if x == Opcode::POP as u8 => Ok(Opcode::POP),
            x if x == Opcode::MLOAD as u8 => Ok(Opcode::MLOAD),
            x if x == Opcode::MSTORE as u8 => Ok(Opcode::MSTORE),
            x if x == Opcode::MSTORE8 as u8 => Ok(Opcode::MSTORE8),
            x if x == Opcode::SLOAD as u8 => Ok(Opcode::SLOAD),
            x if x == Opcode::SSTORE as u8 => Ok(Opcode::SSTORE),
            x if x == Opcode::JUMP as u8 => Ok(Opcode::JUMP),
            x if x == Opcode::JUMPI as u8 => Ok(Opcode::JUMPI),
            x if x == Opcode::PC as u8 => Ok(Opcode::PC),
            x if x == Opcode::MSIZE as u8 => Ok(Opcode::MSIZE),
            x if x == Opcode::GAS as u8 => Ok(Opcode::GAS),
            x if x == Opcode::JUMPDEST as u8 => Ok(Opcode::JUMPDEST),
            x if x == Opcode::PUSH0 as u8 => Ok(Opcode::PUSH0),
            x if x == Opcode::PUSH1 as u8 => Ok(Opcode::PUSH1),
            x if x == Opcode::PUSH2 as u8 => Ok(Opcode::PUSH2),
            x if x == Opcode::PUSH3 as u8 => Ok(Opcode::PUSH3),
            x if x == Opcode::PUSH4 as u8 => Ok(Opcode::PUSH4),
            x if x == Opcode::PUSH5 as u8 => Ok(Opcode::PUSH5),
            x if x == Opcode::PUSH6 as u8 => Ok(Opcode::PUSH6),
            x if x == Opcode::PUSH7 as u8 => Ok(Opcode::PUSH7),
            x if x == Opcode::PUSH8 as u8 => Ok(Opcode::PUSH8),
            x if x == Opcode::PUSH9 as u8 => Ok(Opcode::PUSH9),
            x if x == Opcode::PUSH10 as u8 => Ok(Opcode::PUSH10),
            x if x == Opcode::PUSH11 as u8 => Ok(Opcode::PUSH11),
            x if x == Opcode::PUSH12 as u8 => Ok(Opcode::PUSH12),
            x if x == Opcode::PUSH13 as u8 => Ok(Opcode::PUSH13),
            x if x == Opcode::PUSH14 as u8 => Ok(Opcode::PUSH14),
            x if x == Opcode::PUSH15 as u8 => Ok(Opcode::PUSH15),
            x if x == Opcode::PUSH16 as u8 => Ok(Opcode::PUSH16),
            x if x == Opcode::PUSH17 as u8 => Ok(Opcode::PUSH17),
            x if x == Opcode::PUSH18 as u8 => Ok(Opcode::PUSH18),
            x if x == Opcode::PUSH19 as u8 => Ok(Opcode::PUSH19),
            x if x == Opcode::PUSH20 as u8 => Ok(Opcode::PUSH20),
            x if x == Opcode::PUSH21 as u8 => Ok(Opcode::PUSH21),
            x if x == Opcode::PUSH22 as u8 => Ok(Opcode::PUSH22),
            x if x == Opcode::PUSH23 as u8 => Ok(Opcode::PUSH23),
            x if x == Opcode::PUSH24 as u8 => Ok(Opcode::PUSH24),
            x if x == Opcode::PUSH25 as u8 => Ok(Opcode::PUSH25),
            x if x == Opcode::PUSH26 as u8 => Ok(Opcode::PUSH26),
            x if x == Opcode::PUSH27 as u8 => Ok(Opcode::PUSH27),
            x if x == Opcode::PUSH28 as u8 => Ok(Opcode::PUSH28),
            x if x == Opcode::PUSH29 as u8 => Ok(Opcode::PUSH29),
            x if x == Opcode::PUSH30 as u8 => Ok(Opcode::PUSH30),
            x if x == Opcode::PUSH31 as u8 => Ok(Opcode::PUSH31),
            x if x == Opcode::PUSH32 as u8 => Ok(Opcode::PUSH32),
            x if x == Opcode::DUP1 as u8 => Ok(Opcode::DUP1),
            x if x == Opcode::DUP2 as u8 => Ok(Opcode::DUP2),
            x if x == Opcode::DUP3 as u8 => Ok(Opcode::DUP3),
            x if x == Opcode::DUP4 as u8 => Ok(Opcode::DUP4),
            x if x == Opcode::DUP5 as u8 => Ok(Opcode::DUP5),
            x if x == Opcode::DUP6 as u8 => Ok(Opcode::DUP6),
            x if x == Opcode::DUP7 as u8 => Ok(Opcode::DUP7),
            x if x == Opcode::DUP8 as u8 => Ok(Opcode::DUP8),
            x if x == Opcode::DUP9 as u8 => Ok(Opcode::DUP9),
            x if x == Opcode::DUP10 as u8 => Ok(Opcode::DUP10),
            x if x == Opcode::DUP11 as u8 => Ok(Opcode::DUP11),
            x if x == Opcode::DUP12 as u8 => Ok(Opcode::DUP12),
            x if x == Opcode::DUP13 as u8 => Ok(Opcode::DUP13),
            x if x == Opcode::DUP14 as u8 => Ok(Opcode::DUP14),
            x if x == Opcode::DUP15 as u8 => Ok(Opcode::DUP15),
            x if x == Opcode::DUP16 as u8 => Ok(Opcode::DUP16),
            x if x == Opcode::SWAP1 as u8 => Ok(Opcode::SWAP1),
            x if x == Opcode::SWAP2 as u8 => Ok(Opcode::SWAP2),
            x if x == Opcode::SWAP3 as u8 => Ok(Opcode::SWAP3),
            x if x == Opcode::SWAP4 as u8 => Ok(Opcode::SWAP4),
            x if x == Opcode::SWAP5 as u8 => Ok(Opcode::SWAP5),
            x if x == Opcode::SWAP6 as u8 => Ok(Opcode::SWAP6),
            x if x == Opcode::SWAP7 as u8 => Ok(Opcode::SWAP7),
            x if x == Opcode::SWAP8 as u8 => Ok(Opcode::SWAP8),
            x if x == Opcode::SWAP9 as u8 => Ok(Opcode::SWAP9),
            x if x == Opcode::SWAP10 as u8 => Ok(Opcode::SWAP10),
            x if x == Opcode::SWAP11 as u8 => Ok(Opcode::SWAP11),
            x if x == Opcode::SWAP12 as u8 => Ok(Opcode::SWAP12),
            x if x == Opcode::SWAP13 as u8 => Ok(Opcode::SWAP13),
            x if x == Opcode::SWAP14 as u8 => Ok(Opcode::SWAP14),
            x if x == Opcode::SWAP15 as u8 => Ok(Opcode::SWAP15),
            x if x == Opcode::SWAP16 as u8 => Ok(Opcode::SWAP16),
            x if x == Opcode::LOG0 as u8 => Ok(Opcode::LOG0),
            x if x == Opcode::LOG1 as u8 => Ok(Opcode::LOG1),
            x if x == Opcode::LOG2 as u8 => Ok(Opcode::LOG2),
            x if x == Opcode::LOG3 as u8 => Ok(Opcode::LOG3),
            x if x == Opcode::LOG4 as u8 => Ok(Opcode::LOG4),
            x if x == Opcode::CREATE as u8 => Ok(Opcode::CREATE),
            x if x == Opcode::CALL as u8 => Ok(Opcode::CALL),
            x if x == Opcode::CALLCODE as u8 => Ok(Opcode::CALLCODE),
            x if x == Opcode::RETURN as u8 => Ok(Opcode::RETURN),
            x if x == Opcode::DELEGATECALL as u8 => Ok(Opcode::DELEGATECALL),
            x if x == Opcode::CREATE2 as u8 => Ok(Opcode::CREATE2),
            x if x == Opcode::STATICCALL as u8 => Ok(Opcode::STATICCALL),
            x if x == Opcode::REVERT as u8 => Ok(Opcode::REVERT),
            x if x == Opcode::INVALID as u8 => Ok(Opcode::INVALID),
            x if x == Opcode::SELFDESTRUCT as u8 => Ok(Opcode::SELFDESTRUCT),
            _ => Err(()),
        }
    }
}

impl Opcode {
    /// Convert a byte into an Opcode
    pub fn from_byte(byte: u8) -> Opcode {
        match byte.try_into() {
            Ok(v) => v,
            Err(_) => Opcode::INVALID,
        }
    }
}

/// A decoded operation
///
/// An operation is represented by the combination of an opcode, the offset in the bytecode and any
/// additional bytes that are part of the operation (only for PUSH operations).
#[derive(PartialEq, Eq)]
pub struct Operation {
    /// The opcode
    pub opcode: Opcode,
    /// Additional bytes that are part of the Operation (only for PUSH)
    pub input: Vec<u8>,
    /// The offset in the bytecode
    pub offset: u32,
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted = format!(
            "{:0>8}: {:?}",
            format!("{:#x}", self.offset).trim_start_matches("0x"),
            self.opcode
        );
        if !self.input.is_empty() {
            let encoded_bytes = hex::encode(&self.input);
            let mut formatted_bytes = encoded_bytes.trim_start_matches('0');
            if formatted_bytes.is_empty() {
                formatted_bytes = "0";
            }
            formatted = format!("{} {}", formatted, "0x".to_owned() + formatted_bytes);
        }
        write!(f, "{formatted}")
    }
}

impl Operation {
    /// Creates a new operation with empty `input` bytes
    pub fn new(opcode: Opcode, offset: u32) -> Self {
        Operation {
            opcode,
            offset,
            input: Vec::new(),
        }
    }

    /// Adds additional bytes to the operation (for PUSH instructions)
    pub fn with_bytes(
        self,
        num_bytes: u8,
        bytes: &mut dyn ExactSizeIterator<Item = u8>,
    ) -> Result<Self> {
        if num_bytes == 0 {
            return Ok(self);
        }
        if num_bytes as usize > bytes.len() {
            return Err(eyre!(
                "Not enough bytes to read - expected {} but only {} left",
                num_bytes,
                bytes.len()
            ));
        }
        Ok(Operation {
            opcode: self.opcode,
            offset: self.offset,
            input: bytes.take(num_bytes as usize).collect(),
        })
    }
}
