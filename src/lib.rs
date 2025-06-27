pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // Ensure even number of characters
    if hex_str.len() % 2 != 0 {
        return Err("Hex string must have an even number of characters".to_string());
    }

    // Try parsing each byte
    (0..hex_str.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex_str[i..i + 2], 16)
                .map_err(|e| format!("Invalid hex character at position {}: {}", i, e))
        })
        .collect()
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    let mut reversed = bytes.to_vec();
    reversed.reverse();
    reversed
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex)
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    num.to_le_bytes()
}
pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    input
        .trim()
        .parse::<u64>()
        .map_err(|e| format!("Invalid satoshis value '{}': {}", input, e))
}

#[derive(Debug, PartialEq)]
pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    // P2PKH: OP_DUP OP_HASH160 <20-byte pubkey hash> OP_EQUALVERIFY OP_CHECKSIG
    if script.len() == 25
        && script[0] == 0x76
        && script[1] == 0xa9
        && script[2] == 0x14
        && script[23] == 0x88
        && script[24] == 0xac
    {
        ScriptType::P2PKH
    }
    // P2WPKH: 0x00 <20-byte pubkey hash>
    else if script.len() == 22 && script[0] == 0x00 && script[1] == 0x14 {
        ScriptType::P2WPKH
    } else {
        ScriptType::Unknown
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Outpoint(pub String, pub u32);
// Convenience getters
impl Outpoint {
    pub fn txid(&self) -> &str {
        &self.0
    }
    pub fn vout(&self) -> u32 {
        self.1
    }
}

pub fn read_pushdata(script: &[u8]) -> &[u8] {
    // Assumes OP_PUSHDATA starts at index 2
    &script[2..]
}

pub trait Wallet {
    fn balance(&self) -> u64;
}

pub struct TestWallet {
    pub confirmed: u64,
}

impl Wallet for TestWallet {
    fn balance(&self) -> u64 {
        self.confirmed
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    *balance = balance.saturating_sub(fee);
}

pub fn move_txid(txid: String) -> String {
    format!("Moved txid: {}", txid)
}

// Add necessary derive traits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    OpChecksig,
    OpDup,
    OpInvalid,
}

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0xac => Ok(Opcode::OpChecksig),
            0x76 => Ok(Opcode::OpDup),
            _ => Err(format!("Unknown opcode: 0x{:x}", byte)),
        }
    }
}

// Add necessary derive traits
#[derive(Debug, Clone)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    utxo
}
