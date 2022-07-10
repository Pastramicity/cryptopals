
use crate::common;
pub fn crack_single_key_xor(bytes: &Vec<u8>) -> (u8, Vec<u8>) {
    let decoded_bytes = Vec::new();
    let mut scores = Vec::new();
    let mut max_score = 0;
    let mut best_decoder_byte = 0u8;
    for i in 0..=255u8 {
        //for all characters
        let block_xored = common::xor_one_2_string(bytes, &i);
        let score = common::english_checker(&block_xored);
        if score > max_score {
            max_score = score;
            best_decoder_byte = i;
        }
        scores.push(score);
    }
    (best_decoder_byte, decoded_bytes)
}
pub fn crack_repeating_key_xor_from_raw(filename: &str) {
    let rawfile = std::fs::read_to_string(filename).expect("Couldn't read file");
    let b = rawfile.as_bytes().to_vec();
    let b_str = String::from_utf8(b.clone()).expect("couldn't convert bytes to string");

    let num_keysize_samples = 4;

    let mut normalized_distances = Vec::new();
    for keysize in 2..=40 {
        let mut samples = Vec::new();
        for i in 0..num_keysize_samples {
            samples.push(&b[(keysize * i)..(keysize * (i + 1))]);
        }

        let mut hamming_sum = 0;
        for i in 0..num_keysize_samples - 1 {
            hamming_sum += common::hamming_distance(samples[i], samples[i + 1]);
        }
        let normalized_distance = (hamming_sum) / (keysize * (num_keysize_samples - 1)) as u64;
        normalized_distances.push((keysize, normalized_distance));
    }

    normalized_distances.sort_by_key(|d| d.1);
    let mut best_result = String::new();
    let mut best_key = String::new();
    let mut best_score = 0;
    for (keysize, _) in &normalized_distances[..5] {
        let mut block_bytes = vec![vec![0u8; 0]; *keysize];
        for (i, byte) in b.iter().enumerate() {
            block_bytes[i % keysize].push(*byte);
        }
        let mut keys = String::new();
        for bbytes in block_bytes {
            keys.push(crate::cracking::crack_single_key_xor(&bbytes).0 as char);
        }
        // let key = keys.repeat(b.len()).as_bytes().to_vec();
        let split_keys = keys.split("\n");
        let key = split_keys.last().expect("no keys in the keys");
        let plaintext = common::repeating_key_xor(&b_str, &key);
        let plaintext = String::from_utf8(plaintext).expect("bad utf8 result string");

        let score = common::english_checker(&plaintext);
        if score > best_score {
            best_result = plaintext;
            best_key = String::from(key);
            best_score = score;
        }
    }

    println!("key: {}", best_key);
    println!("result: {}", best_result);
}
