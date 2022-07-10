
use crate::common;
// 6 bits per hex digit
// no overflow
pub fn b_2_b64(bytes: &Vec<u8>) -> String {
    let mut ret = String::new();
    //take bytes in chunks of 3 to turn into 4 digits of Base64
    let b64_bytes = common::rechunk(64, bytes);
    for byte in b64_bytes {
        ret.push(match byte {
            0..=25 => byte + b'A',
            26..=51 => byte - 26 + b'a',
            52..=61 => byte - 52 + b'0',
            62 => b'+',
            63 => b'/',
            _ => {
                panic!("this byte was not encoded correctly");
            }
        } as char);
    }
    ret
}

pub fn b_2_hex(bytes: &Vec<u8>) -> String {
    let mut ret = String::new();
    for byte in bytes {
        let dr = crate::b2d::d2hex(&(byte & 0xf));
        let dl = crate::b2d::d2hex(&(byte >> 4));
        ret.push(dl as char);
        ret.push(dr as char);
    }
    ret
}
