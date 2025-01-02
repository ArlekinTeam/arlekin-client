use argon2::Argon2;
use hex::encode;

pub fn hash_password(password: &str, email: &str) -> String {
    let mut password_hash = [0u8; 32];

    Argon2::default()
        .hash_password_into(
            password.as_bytes(),
            format!("arlekin{}login", email).as_bytes(),
            &mut password_hash,
        )
        .unwrap();

    encode(password_hash)
}
