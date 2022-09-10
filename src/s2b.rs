use crate::common;
// string to bytes
use base64;
// 2 hex digits per byte
pub fn hex2b(hex: &str) -> Vec<u8> {
    let mut ret = Vec::new();
    let chars = hex.as_bytes();
    // get vec with one byte per hex char
    for c in chars.chunks(2) {
        ret.push(common::hex2d(&c[0]) << 4 | common::hex2d(&c[1]));
    }
    ret
}

////easy way
pub fn b642b(b64: &str) -> Vec<u8> {
    base64::decode(b64).expect("bad base64")
}

//shouldn't be here, maybe fix later/figure out how sub modules work lol
