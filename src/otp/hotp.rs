use super::HashFunction;
use ring::{hmac, digest};

#[derive(Debug)]
pub struct HOTP {
    secret_key: Vec<u8>,
    counter: u64,
    output_len: usize,
    output_base: Vec<u8>,
    hash_function: HashFunction,
}

impl HOTP {
    pub fn new(secret_key: Vec<u8>, counter: u64, output_len: usize) -> HOTP {
        HOTP {
            secret_key: secret_key,
            counter: counter,
            output_len: output_len,
            output_base: "0123456789".to_owned().into_bytes(),
            hash_function: HashFunction::SHA1,
        }
    }

    pub fn generate(&self) -> String {
        let message: [u8; 8] = [
            ((self.counter >> 56) & 0xff) as u8,
            ((self.counter >> 48) & 0xff) as u8,
            ((self.counter >> 40) & 0xff) as u8,
            ((self.counter >> 32) & 0xff) as u8,
            ((self.counter >> 24) & 0xff) as u8,
            ((self.counter >> 16) & 0xff) as u8,
            ((self.counter >> 8) & 0xff) as u8,
            (self.counter & 0xff) as u8,
        ];
        let signing_key = match self.hash_function {
            HashFunction::SHA1 => hmac::SigningKey::new(&digest::SHA1, &self.secret_key),
            HashFunction::SHA256 => hmac::SigningKey::new(&digest::SHA256, &self.secret_key),
            HashFunction::SHA384 => hmac::SigningKey::new(&digest::SHA384, &self.secret_key),
            HashFunction::SHA512 => hmac::SigningKey::new(&digest::SHA512, &self.secret_key),
            HashFunction::SHA512_256 => hmac::SigningKey::new(&digest::SHA512_256, &self.secret_key),
        };
        let digest = hmac::sign(&signing_key, &message);
        self.encode_digest(digest.as_ref())
    }

    fn encode_digest(&self, digest: &[u8]) -> String {
        let offset = (*digest.last().unwrap() & 0xf) as usize;
        let snum: u32 = ((digest[offset] as u32 & 0x7f) << 24) |
                        ((digest[offset + 1] as u32 & 0xff) << 16) |
                        ((digest[offset + 2] as u32 & 0xff) << 8) |
                        (digest[offset + 3] as u32 & 0xff);
        let base = self.output_base.len() as u32;
        let hotp_code = snum % base.pow(self.output_len as u32);
        let mut code = hotp_code.to_string();
        while code.len() < self.output_len {
            code = "0".to_owned() + &code;
        }
        code
    }
}
