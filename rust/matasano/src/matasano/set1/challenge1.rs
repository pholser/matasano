use serialize::hex::{FromHex, ToHex};
use serialize::base64::{self, ToBase64};

pub fn hex2bytes(s: &str) -> Vec<u8> {
    s.from_hex().unwrap()
}

pub fn bytes2hex(bytes: &Vec<u8>) -> String {
    (&bytes[..]).to_hex()
}

pub fn bytes2base64(bytes: &Vec<u8>) -> String {
    (&bytes[..]).to_base64(base64::STANDARD)
}

pub fn hex2base64(s: &str) -> String {
    bytes2base64(&hex2bytes(s))
}
