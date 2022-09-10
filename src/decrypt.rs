pub fn aes_ecb(bytes: &Vec<u8>, key_str: &str) -> String {
    use aes::cipher::{
        generic_array::GenericArray, BlockDecrypt,  KeyInit,
    };
    use aes::Aes128;

    let ret;

    let key = GenericArray::clone_from_slice(key_str.as_bytes());

    let mut blocks = Vec::new();

    (0..bytes.len()).step_by(16).for_each(|x| {
        blocks.push(GenericArray::clone_from_slice(&bytes[x..x + 16]));
    });

    let cipher = Aes128::new(&key);
    cipher.decrypt_blocks(&mut blocks);

    ret = blocks.iter().flatten().map(|&x| x as char).collect(); // declarative

    ret
}
