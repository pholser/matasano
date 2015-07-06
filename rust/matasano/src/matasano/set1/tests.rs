pub use super::challenge1::*;
pub use super::challenge2::*;

#[test]
fn converting_string_of_hex_to_base_64() {
    assert_eq!(
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
        hex2base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"));
}

#[test]
fn xoring_two_equal_length_buffers() {
    let first = hex2bytes("1c0111001f010100061a024b53535009181c");
    let second = hex2bytes("686974207468652062756c6c277320657965");

    let result = fixed_xor(&first, &second);

    assert_eq!(
        "746865206b696420646f6e277420706c6179",
        bytes2hex(&result)
    );
}
