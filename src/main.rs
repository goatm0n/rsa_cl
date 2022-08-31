use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short = 'm', long = "mode")]
    mode: String,
    #[clap(parse(from_os_str), short = 'p', long = "file_path")]
    file_path: PathBuf,
    #[clap(parse(from_os_str), short = 'k', long = "key_path")]
    key_path: PathBuf,
}

fn get_full_path(path: &PathBuf) -> PathBuf {
    let mut full_path = PathBuf::new();
    let path_str = path.to_str().unwrap();
    if path_str.chars().nth(0).unwrap() != 'C' {
        let cwd = std::env::current_dir().unwrap();
        full_path.push(cwd);
    }
    full_path.push(path);
    return full_path;
}

impl Cli {
    fn file_path(&self) -> PathBuf {
        get_full_path(&self.file_path)
    }
    fn key_path(&self) -> PathBuf {
        get_full_path(&self.key_path)
    } 
}

fn handle_encryption() {}

fn handle_decryption() {}

fn main() {
    let args = Cli::parse();

    // parse file paths and get contents
    let file_content = std::fs::read_to_string(args.file_path()).expect("could not read file");
    let key_content = std::fs::read_to_string(args.key_path()).expect("could not read file");

    // perform checks
    if file_content.len() == 0 {
        panic!("file is empty");
    }
    if key_content.len() == 0 && args.mode == "decrypt" {
        panic!("cannot decrypt without private key");
    }
    
    match args.mode.as_str() {
        "encrypt" => handle_encryption(),
        "decrypt" => handle_decryption(),
        _ => panic!("invalid mode; options = [encrypt, decrypt]")
    } 

}







