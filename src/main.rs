use clap::Parser;
use std::path::PathBuf;
use rsa_utils::io::{
    get_full_path,
    encrypt_file,
    decrypt_file,
};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short = 'm', long = "mode")]
    mode: String,
    #[clap(parse(from_os_str), short = 'p', long = "file_path")]
    file_path: PathBuf,
    #[clap(parse(from_os_str), short = 'k', long = "key_path")]
    key_path: PathBuf,
}

impl Cli {
    fn file_path(&self) -> PathBuf {
        get_full_path(&self.file_path)
    }
    fn key_path(&self) -> PathBuf {
        get_full_path(&self.key_path)
    } 
}

fn main() {
    let args = Cli::parse();
    let file_path = args.file_path();
    let key_path = args.key_path();
    match args.mode.as_str() {
        "encrypt" => encrypt_file(file_path, key_path),
        "decrypt" => decrypt_file(file_path, key_path),
        _ => panic!("invalid mode; options = [encrypt, decrypt]")
    } 
}
