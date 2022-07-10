use solutions::*;
mod b2s;
mod common;
mod cracking;
mod s2b;

fn main() {
    s1c7();
}

pub mod solutions {
    use crate::*;
    use aes::cipher::{
        generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit,
    };
    use aes::Aes128;
    use common::input;
    use std::fs;

    pub fn s1c7() {
        // let file = fs::read("s1c7.raw").expect("couldn't read file");
        let file = fs::read_to_string("s1c7.b64").expect("couldn't read file");
        let file = file.replace("\n", "");
        let file = file.as_bytes();
        let key_str = "YELLOW SUBMARINE";
        let key = GenericArray::clone_from_slice(key_str.as_bytes());


        // pub fn decrypt_message(path: &str, key_str: &str) -> String {
        //     let base64_bytes = read_bytes(path);
        //     let key = GenericArray::clone_from_slice(key_str.as_bytes());

        //     // Construct blocks of 16 byte size for AES-128
        //     let mut blocks = Vec::new();
        //     (0..base64_bytes.len()).step_by(16).for_each(|x| {
        //         blocks.push(GenericArray::clone_from_slice(&base64_bytes[x..x + 16]));
        //     });

        //     // Initialize cipher
        //     let cipher = Aes128::new(&key);
        //     cipher.decrypt_blocks(&mut blocks);

        //     blocks.iter().flatten().map(|&x| x as char).collect()
        // }
    }

    pub fn s1c6() {
        cracking::crack_repeating_key_xor_from_raw("s1c6.raw");
    }
    pub fn s1c5() {
        println!("Enter string to encode: ");
        let string = input();
        println!("Enter encryption key: ");
        let key = input();
        let bytes = common::repeating_key_xor(&string[..], &key[..]);
        let out = b2s::b_2_hex(&bytes);
        println!("{}", out);
    }
    pub fn s1c4_single_threaded() {
        let file = fs::read_to_string("s1c4.txt").expect("Couldn't read s1c4.txt");
        let mut strings = Vec::new();
        let mut scores = Vec::new();
        let mut max_score = 0;
        let mut max_score_index = 0;
        for line in file.split("\n") {
            let line_decoded = s2b::hex2b(line);
            for i in 0..=255u8 {
                let line_xored = common::xor_one_2_string(&line_decoded, &i);
                let score = common::english_checker(&line_xored);
                if score > max_score {
                    max_score = score;
                    max_score_index = scores.len();
                }
                scores.push(score);
                strings.push(line_xored);
            }
        }
        println!("{}", strings[max_score_index]);
    }

    pub fn s1c2() {
        println!("Enter string 1: ");
        let input1 = input();
        println!("Enter string 2: ");
        let input2 = input();

        let bytes1 = s2b::hex2b(&input1[..]);
        let bytes2 = s2b::hex2b(&input2[..]);
        let out_bytes = common::xor_all(&bytes1, &bytes2);
        for b in &out_bytes {
            println!("{}", b);
        }
        let text = b2s::b_2_hex(&out_bytes);
        println!("{}", text);
    }

    pub fn s1c1() {
        println!("enter your input string: ");
        let input = input();
        let bytes = s2b::hex2b(&input[..]);
        let b64 = b2s::b_2_b64(&bytes);
        println!("{}", b64);
    }
}
