use sha2::{Digest, Sha256};
use std::fs::File;
use anyhow;
use std::io::{BufReader, Read};

pub fn calculate_hash(path:&str)->anyhow::Result<String>{
    let file=File::open(path)?;
    let mut reader=BufReader::new(file);
    let mut hasher=Sha256::new();

    let mut buffer=[0u8;8192];
    loop {
        let n=reader.read(&mut buffer)?;
        if n==0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let digest = hasher.finalize();
    Ok(format!("{:x}", digest))
}
