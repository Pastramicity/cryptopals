use solutions::*;
use std::collections::HashSet;
mod b2s;
mod common;
mod cracking;
mod decrypt;
mod s2b;


fn main() {
    // s2c10();
    s1c6();
}

pub mod solutions {
    use crate::*;
    use common::input;
    use std::fs;

    pub fn s2c10(){
        let _b64f = fs::read_to_string("s2c10.b64").unwrap();
        let rawf = fs::read("s2c10.raw").unwrap();
    }

    pub fn s2c9(){

        let string = input().trim().to_owned();
        let length = input().trim().parse::<usize>().expect("please input a valid usize");
        let output = common::pkcs7_pad(string.as_str(), length);
        println!("output string: {:?}", output);
    }

    pub fn s1c8(){
        let file = fs::read_to_string("s1c8.hex").expect("couldn't read file");
        let hex = file.lines();
        let mut most_likely_line = String::new();
        let mut most_repeats = 0;
        for (i, line) in hex.enumerate(){
            let bytes = s2b::hex2b(line);
            let sets: Vec<_> = bytes.chunks_exact(16).collect();
            let unique_sets: HashSet<_> = sets.iter().cloned().collect();
            let repeats = sets.len() - unique_sets.len();
            if repeats > most_repeats{
                most_repeats = repeats;
                most_likely_line = line.to_string();
            }
        }
        println!("{}", most_likely_line);

    }

    pub fn s1c7() {
        let file = fs::read("s1c7.raw").expect("couldn't read file");
        // let file = fs::read_to_string("s1c7.b64").expect("couldn't read file");
        // let file = file.replace("\n", "");
        // let file = file.as_bytes().to_owned();
        let key_str = "YELLOW SUBMARINE";

        let output = decrypt::aes_ecb(&file, key_str);
        println!("{}", output);
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
