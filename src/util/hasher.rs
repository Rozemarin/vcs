use sha1::{Sha1, Digest};

pub fn hasher(content: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let x: String = format!("{:x}", hasher.finalize());
    x
}