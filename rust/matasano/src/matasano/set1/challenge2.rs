pub fn fixed_xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    assert_eq!(a.len(), b.len());

    let mut result = Vec::with_capacity(a.len());
    for (x, y) in a.iter().zip(b.iter()) {
        result.push(x ^ y);
    }
    result
}
