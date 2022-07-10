
pub fn input() -> String {
    let mut ret = String::new();
    std::io::stdin()
        .read_line(&mut ret)
        .expect("clean input string.");
    ret.trim().to_owned()
}
pub fn hex2d(ch: &u8) -> u8 {
    match ch {
        b'0'..=b'9' => ch - b'0',
        b'a'..=b'f' => ch - b'a' + 10,
        _ => {
            panic!("bad hex string!");
        }
    }
}
pub fn d2hex(b: &u8) -> u8 {
    match b {
        0..=9 => b'0' + b,
        10..=15 => b'a' - 10 + b,
        _ => {
            panic!("hex value not in digit range.")
        }
    }
}
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
        for _i in 0..num_output_bytes {
            // split bytes up and push to ret
            let mask = (base - 1) as u64;
            output_chunk.insert(0, (mask & big_num) as u8);
            big_num >>= bpd;
        }
        ret.append(&mut output_chunk);
    }

    ret
}

pub fn xor_all(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> Vec<u8> {
    let mut ret = Vec::new();
    assert_eq!(bytes1.len(), bytes2.len());
    let len = bytes1.len();
    for i in 0..len {
        ret.push(bytes1[i] ^ bytes2[i]);
    }
    ret
}

pub fn xor_one(bytes: &Vec<u8>, byte: &u8) -> Vec<u8> {
    let mut ret = Vec::new();
    for b in bytes {
        ret.push(b ^ byte);
    }
    ret
}
pub fn xor_one_2_string(bytes: &Vec<u8>, byte: &u8) -> String {
    let mut ret = String::new();
    for b in bytes {
        ret.push((b ^ byte) as char);
    }
    ret
}

pub fn lcm(first: u32, second: u32) -> u32 {
    first * second / gcd(first, second)
}
pub fn gcd(first: u32, second: u32) -> u32 {
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
// scores a string based on how likely it is to be language, may be changed later
pub fn english_checker(string: &String) -> i64 {
    let mut ret = 0;
    for ch in string.chars() {
        ret += match ch {
            ' ' => 12,
            'e' => 8,
            't' => 7,
            'a' => 7,
            'i' => 6,
            'o' => 6,
            'n' | 's' | 'h' | 'r' => 5,
            'a'..='z' => 4,
            '0'..='9' | ',' | '.' | '!' | '?' => 3,
            ' '..='~' => 2,
            _ => 0,
        }
    }
    ret
}

pub fn repeating_key_xor(string: &str, key: &str) -> Vec<u8> {
    let mut ret = Vec::new();
    let key_bytes = key.as_bytes();
    for (i, val) in string.bytes().enumerate() {
        let key_letter: u8 = key_bytes[i % key.len()];
        let xored_letter = val ^ key_letter;
        ret.push(xored_letter);
    }
    ret
}

pub fn hamming_distance(s1: &[u8], s2: &[u8]) -> u64 {
    let mut ret: u64 = 0;
    let mask = 1;
    assert_eq!(s1.len(), s2.len());
    for i in 0..s1.len() {
        let mut a = s1[i].clone();
        let mut b = s2[i].clone();
        for _j in 0..8 {
            ret += ((a & mask) ^ (b & mask)) as u64;
            a >>= 1;
            b >>= 1;
        }
    }
    ret
}
