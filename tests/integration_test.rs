use std::path::PathBuf;
use std::fs::read_to_string;
use rsa_utils::io::{
    get_full_path,
    encrypt_file,
    decrypt_file,
};


#[test]
fn test_encrypt_decrypt_file() {
    let file_path = get_full_path(&PathBuf::from("testfile.txt"));
    let key_path = get_full_path(&PathBuf::from("testkey.txt"));
    let msg = read_to_string(file_path.clone()).unwrap();
    dbg!(&msg);
    encrypt_file(file_path.clone(), key_path.clone());
    let enc_msg = read_to_string(file_path.clone()).unwrap();
    dbg!(&enc_msg);
    decrypt_file(file_path.clone(), key_path);
    let dec_msg = read_to_string(file_path.clone()).unwrap();
    dbg!(&dec_msg);
    assert_eq!(msg, dec_msg);
}