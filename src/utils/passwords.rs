use sha2::{Digest, Sha512};

pub fn hash_password(password: &str) -> String {
    format!("{:x}", Sha512::digest(password.as_bytes()))
}
