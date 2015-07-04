use serialize::hex::FromHex;
use serialize::base64::{self, ToBase64};

fn hex2base64(s: &str) -> String {
    (&(s.from_hex().unwrap())[..]).to_base64(base64::STANDARD)
}
