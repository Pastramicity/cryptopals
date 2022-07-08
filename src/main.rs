use solutions::*;
fn main() {}

pub mod solutions {
    use crate::*;
    use std::io::stdin;
    fn input() -> String {
        let mut ret = String::new();
        stdin().read_line(&mut ret).expect("clean input string.");
        ret.trim().to_owned()
    }

    pub fn s1c1() {
        println!("enter your input string: ");
        let input = input();
        let bytes = s2b::hex_2_b(&input[..]);
        let b64 = b2s::b_2_b64(&bytes);
        println!("{}", b64);
    }
    pub fn s1c2() {
        println!("Enter string 1: ");
        let input1 = input();
        println!("Enter string 2: ");
        let input2 = input();

        let bytes1 = s2b::hex_2_b(&input1[..]);
        let bytes2 = s2b::hex_2_b(&input2[..]);
        let out_bytes = manip::xor(&bytes1, &bytes2);
        for b in &out_bytes {
            println!("{}", b);
        }
        let text = b2s::b_2_hex(&out_bytes);
        println!("{}", text);
    }

    pub fn s1c3() {}
}

// string to bytes
pub mod s2b {
    use crate::d2b;
    // 2 hex digits per byte
    pub fn hex_2_b(hex: &str) -> Vec<u8> {
        let mut ret = Vec::new();
        let chars = hex.as_bytes();
        // get vec with one byte per hex char
        for c in chars.chunks(2) {
            ret.push(d2b::hex2d(&c[0]) << 4 | d2b::hex2d(&c[1]));
        }
        ret
    }
}

pub mod b2s {
    use crate::manip;
    // 6 bits per hex digit
    // no overflow
    pub fn b_2_b64(bytes: &Vec<u8>) -> String {
        let mut ret = String::new();
        //take bytes in chunks of 3 to turn into 4 digits of Base64
        let b64_bytes = manip::rechunk(64, bytes);
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
}
pub mod manip {
    pub fn rechunk(base: u32, bytes: &Vec<u8>) -> Vec<u8> {
        let mut ret: Vec<u8> = Vec::new();
        let bpd = f32::log2(base as f32) as u8; //bits per digit
        let lcm = lcm(8, bpd as u32);
        let num_input_bytes = lcm / 8;
        let num_output_bytes = lcm / bpd as u32;

        // put bytes into big number together
        for chunk in bytes.chunks(num_input_bytes as usize) {
            let mut big_num: u64 = 0;
            for (i, val) in chunk.iter().enumerate() {
                big_num += *val as u64;
                if i != (num_input_bytes - 1) as usize {
                    big_num <<= 8;
                }
            }

            let mut output_chunk = Vec::new();
            for i in 0..num_output_bytes {
                // split bytes up and push to ret
                let mask = (base - 1) as u64;
                output_chunk.insert(0, (mask & big_num) as u8);
                big_num >>= bpd;
            }
            ret.append(&mut output_chunk);
        }

        ret
    }

    pub fn xor(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> Vec<u8> {
        let mut ret = Vec::new();
        assert_eq!(bytes1.len(), bytes2.len());
        let len = bytes1.len();
        for i in 0..len {
            ret.push(bytes1[i] ^ bytes2[i]);
        }
        ret
    }

    fn lcm(first: u32, second: u32) -> u32 {
        first * second / gcd(first, second)
    }
    fn gcd(first: u32, second: u32) -> u32 {
        let mut max = first;
        let mut min = second;
        if min > max {
            let val = max;
            max = min;
            min = val;
        }

        loop {
            let res = max % min;
            if res == 0 {
                return min;
            }

            max = min;
            min = res;
        }
    }
}

// digit to byte
pub mod d2b {
    pub fn hex2d(ch: &u8) -> u8 {
        match ch {
            b'0'..=b'9' => ch - b'0',
            b'a'..=b'f' => ch - b'a' + 10,
            _ => {
                panic!("bad hex string!");
            }
        }
    }
}

pub mod b2d {
    pub fn d2hex(b: &u8) -> u8 {
        match b {
            0..=9 => b'0' + b,
            10..=15 => b'a' - 10 + b,
            _ => {
                panic!("hex value not in digit range.")
            }
        }
    }
}
