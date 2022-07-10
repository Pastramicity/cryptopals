// string to bytes
use crate::d2b;
use base64;
// 2 hex digits per byte
pub fn hex2b(hex: &str) -> Vec<u8> {
    let mut ret = Vec::new();
    let chars = hex.as_bytes();
    // get vec with one byte per hex char
    for c in chars.chunks(2) {
        ret.push(d2b::hex2d(&c[0]) << 4 | d2b::hex2d(&c[1]));
    }
    ret
}

////easy way
pub fn b642b(b64: &str) -> Vec<u8> {
    base64::decode(b64).expect("bad base64")
}
