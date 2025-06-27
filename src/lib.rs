pub fn decode_hex(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Invalid hex".to_string());
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| "Invalid hex".to_string()))
        .collect()
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().rev().cloned().collect()
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    decode_hex(hex)
}

pub fn swap_endian_u32(value: u32) -> [u8; 4] {
    value.to_le_bytes()
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    input
        .trim()
        .parse::<u64>()
        .map_err(|_| "Invalid satoshi amount".to_string())
}

pub fn classify_script(script: &[u8]) -> ScriptType {
    if script.starts_with(&[0x76, 0xa9, 0x14]) {
        ScriptType::P2PKH
    } else if script.starts_with(&[0x00, 0x14]) {
        ScriptType::P2WPKH
    } else {
        ScriptType::Unknown
    }
}

pub fn read_pushdata(script: &[u8]) -> Vec<u8> {
    if script.len() > 2 {
        script[2..].to_vec()
    } else {
        vec![]
    }
}

pub fn apply_fee(balance: &mut u64, fee: u64) {
    *balance = balance.saturating_sub(fee);
}

pub fn move_txid(txid: String) -> String {
    format!("txid: {}", txid)
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    OpDup,
    OpChecksig,
}

#[derive(Debug, PartialEq)]
pub enum ScriptType {
    P2PKH,
    P2WPKH,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UTXO {
    pub txid: Vec<u8>,
    pub vout: u32,
    pub value: u64,
}

#[derive(Debug)]
pub struct Outpoint(pub String, pub u32);

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

impl Opcode {
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0x76 => Ok(Opcode::OpDup),
            0xac => Ok(Opcode::OpChecksig),
            _ => Err(format!("Invalid opcode: 0x{:02x}", byte)),
        }
    }
}

pub fn consume_utxo(utxo: UTXO) -> UTXO {
    utxo
}
