use data_encoding::BASE64_NOPAD;
use errors::{Error, Result};
use ring::rand::SecureRandom;
use ring::{aead, digest, pbkdf2, rand};
use std::num::NonZeroU32;
use std::str;

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const ITERATIONS: u32 = 100_000;
const HEADER: &str = "VAULT;AES256";

fn nonce(value: Option<&[u8]>) -> aead::Nonce {
    let mut nonce = [0u8; aead::NONCE_LEN];
    if value.is_none() {
        // generate a unique nonce
        let rng = rand::SystemRandom::new();
        rng.fill(&mut nonce).unwrap();
        aead::Nonce::assume_unique_for_key(nonce)
    } else {
        aead::Nonce::try_assume_unique_for_key(value.unwrap()).unwrap()
    }
}

fn salt() -> Vec<u8> {
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt).unwrap();
    salt.to_vec()
}

fn base64_decode(input: &str) -> Result<Vec<u8>> {
    let decoded_input = BASE64_NOPAD
        .decode(input.as_bytes())
        .map_err(|_| Error::FileNotEncrypted)?; // change error
    Ok(decoded_input)
}

fn pbkdf2_derive(password: &str) -> (Vec<u8>, Vec<u8>) {
    let mut derived_key = [0u8; CREDENTIAL_LEN];
    let salt = salt();
    pbkdf2::derive(
        &digest::SHA256,
        NonZeroU32::new(ITERATIONS).unwrap(),
        &salt,
        &password.as_bytes(),
        &mut derived_key,
    );
    (salt.to_vec(), derived_key.to_vec())
}

fn pbkdf2_verify(salt: &[u8], previously_derived: &[u8], password: &str) -> Result<()> {
    pbkdf2::verify(
        &digest::SHA256,
        NonZeroU32::new(ITERATIONS).unwrap(),
        salt,
        password.as_bytes(),
        previously_derived,
    )
    .map_err(|_| Error::WrongPassword)?;

    Ok(())
}

pub fn encrypt(plaintext: &str, password: &str) -> Result<String> {
    // encrypt plaintext using AES2561
    let (salt, derived_key) = pbkdf2_derive(password);
    let sealing_key = aead::SealingKey::new(&aead::AES_256_GCM, &derived_key).unwrap();
    let mut in_out = plaintext.as_bytes().to_vec();
    let tag_len = sealing_key.algorithm().tag_len();
    for _ in 0..tag_len {
        in_out.push(0);
    }

    // https://briansmith.org/rustdoc/ring/aead/fn.seal_in_place.html
    let nonce = nonce(None);
    let nonce_value = *nonce.as_ref();
    let out_len = aead::seal_in_place(
        &sealing_key,
        nonce,
        aead::Aad::empty(),
        &mut in_out[..],
        tag_len,
    )
    .map_err(|_| Error::EncryptionError)?;

    // return hexified salt + pbkdf2 key + nonce + cipher
    let encoded_salt = BASE64_NOPAD.encode(&salt);
    let encoded_pbkdf2_key = BASE64_NOPAD.encode(&derived_key);
    let encoded_nonce = BASE64_NOPAD.encode(&nonce_value);
    let encoded_ciphertext = BASE64_NOPAD.encode(&in_out[..out_len]);
    let text = format!(
        "{}\n{}\n{}\n{}",
        encoded_salt, encoded_pbkdf2_key, encoded_nonce, encoded_ciphertext
    );
    let encoded_text = BASE64_NOPAD.encode(text.as_bytes());
    Ok(format!("{}\n{}", HEADER, encoded_text))
}

pub fn decrypt(file_content: &str, password: &str) -> Result<String> {
    if file_content.is_empty() {
        return Ok(file_content.to_string());
    }
    // decrypt file using AES25
    if !file_content.starts_with(HEADER) {
        return Err(Error::FileNotEncrypted);
    }
    let split_content: Vec<&str> = file_content.split('\n').collect();
    let vaulttext = base64_decode(split_content[1])?; // check if index exists first
    let vaulttext = str::from_utf8(&vaulttext).unwrap();

    // vaulttext contains salt, pbkdf2 key, nonce and ciphertext
    let vaulttext_split: Vec<&str> = vaulttext.split('\n').collect();
    let salt = base64_decode(vaulttext_split[0])?;
    let pbkdf2_derived_key = base64_decode(vaulttext_split[1])?;
    let nonce_value = base64_decode(vaulttext_split[2])?;
    let mut ciphertext = base64_decode(vaulttext_split[3])?;

    // verify password
    pbkdf2_verify(&salt, &pbkdf2_derived_key, password)?;

    // decrypt
    let opening_key = aead::OpeningKey::new(&aead::AES_256_GCM, &pbkdf2_derived_key).unwrap();
    let plaintext = aead::open_in_place(
        &opening_key,
        nonce(Some(&nonce_value)),
        aead::Aad::empty(),
        0,
        &mut ciphertext,
    )
    .map_err(|_| Error::DecryptionError)?;

    Ok(String::from_utf8(plaintext.to_vec()).unwrap())
}
