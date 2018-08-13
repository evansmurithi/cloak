use data_encoding::BASE32_NOPAD;
use errors::{Error, Result};
use ring::{digest, hmac};
use std::time::{SystemTime, UNIX_EPOCH};

// Define the types of hash functions supported
#[derive(Debug)]
pub enum HashFunction {
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    SHA512_256,
}

// Structure representing a One Time Password
#[derive(Debug)]
pub struct OTP {
    key: Vec<u8>,
    counter: u64,
    totp: bool,
    output_len: usize,
    output_base: Vec<u8>,
    hash_function: HashFunction,
}

impl OTP {
    pub fn new(
        key: &str,
        totp: bool,
        hash_function: &str,
        counter: Option<u64>,
        output_len: Option<usize>,
    ) -> Result<OTP> {
        let decoded_key = BASE32_NOPAD
            .decode(key.as_bytes())
            .map_err(|err| Error::KeyDecode {
                key: key.to_owned(),
                cause: Box::new(err),
            })?;
        let counter = match counter {
            Some(c) => c,
            None => 0 as u64,
        };
        let output_len = match output_len {
            Some(len) => len,
            None => 6,
        };
        let hash_function = match hash_function {
            "SHA1" => HashFunction::SHA1,
            "SHA256" => HashFunction::SHA256,
            "SHA384" => HashFunction::SHA384,
            "SHA512" => HashFunction::SHA512,
            "SHA512_256" => HashFunction::SHA512_256,
            _ => HashFunction::SHA1,
        };
        let otp = OTP {
            key: decoded_key,
            counter,
            totp,
            output_len,
            output_base: "0123456789".to_owned().into_bytes(),
            hash_function,
        };
        Ok(otp)
    }

    // Generate a code as defined in [RFC4226](https://tools.ietf.org/html/rfc4226)
    pub fn generate(&self) -> String {
        let counter = self.get_counter();
        let message: [u8; 8] = [
            ((counter >> 56) & 0xff) as u8,
            ((counter >> 48) & 0xff) as u8,
            ((counter >> 40) & 0xff) as u8,
            ((counter >> 32) & 0xff) as u8,
            ((counter >> 24) & 0xff) as u8,
            ((counter >> 16) & 0xff) as u8,
            ((counter >> 8) & 0xff) as u8,
            (counter & 0xff) as u8,
        ];
        let signing_key = match self.hash_function {
            HashFunction::SHA1 => hmac::SigningKey::new(&digest::SHA1, &self.key),
            HashFunction::SHA256 => hmac::SigningKey::new(&digest::SHA256, &self.key),
            HashFunction::SHA384 => hmac::SigningKey::new(&digest::SHA384, &self.key),
            HashFunction::SHA512 => hmac::SigningKey::new(&digest::SHA512, &self.key),
            HashFunction::SHA512_256 => hmac::SigningKey::new(&digest::SHA512_256, &self.key),
        };
        let digest = hmac::sign(&signing_key, &message);
        self.encode_digest(digest.as_ref())
    }

    fn encode_digest(&self, digest: &[u8]) -> String {
        let offset = (*digest.last().unwrap() & 0xf) as usize;
        let snum: u32 = ((u32::from(digest[offset]) & 0x7f) << 24)
            | ((u32::from(digest[offset + 1]) & 0xff) << 16)
            | ((u32::from(digest[offset + 2]) & 0xff) << 8)
            | (u32::from(digest[offset + 3]) & 0xff);
        let base = self.output_base.len() as u32;
        let hotp_code = snum % base.pow(self.output_len as u32);
        let code = format!("{:0width$}", hotp_code, width = self.output_len);
        code
    }

    // Calculate counter based on whether the OTP is time based or counter based
    fn get_counter(&self) -> u64 {
        if self.totp {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u64;
            timestamp / 30
        } else {
            self.counter
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OTP;

    macro_rules! test_hotp_hash_fn {
        ($func:ident, $hf:expr, $c:tt) => {
            #[test]
            fn $func() {
                let key = "4AZJFQFIGYM2KMTOO72I6FAOZ6ZFWJR6";
                let hotp = OTP::new(key, false, $hf, None, None).unwrap();
                assert_eq!(hotp.generate(), $c);
            }
        };
    }

    test_hotp_hash_fn!(test_sha1, "SHA1", "852241");
    test_hotp_hash_fn!(test_sha256, "SHA256", "851154");
    test_hotp_hash_fn!(test_sha384, "SHA384", "607946");
    test_hotp_hash_fn!(test_sha512, "SHA512", "377017");
    test_hotp_hash_fn!(test_sha512_256, "SHA512_256", "171117");

    #[test]
    fn test_hotp_default() {
        let key = "4AZJFQFIGYM2KMTOO72I6FAOZ6ZFWJR6";
        let hotp = OTP::new(key, false, "SHA1", None, None).unwrap();
        assert_eq!(hotp.counter, 0);
        let code = hotp.generate();
        assert_eq!(code.len(), 6);
        assert_eq!(code, "852241");
    }

    #[test]
    fn test_hotp_given_counter_and_length() {
        let key = "4AZJFQFIGYM2KMTOO72I6FAOZ6ZFWJR6";
        let hotp = OTP::new(key, false, "SHA1", Some(1), Some(8)).unwrap();
        let code = hotp.generate();
        assert_eq!(code.len(), 8);
        assert_eq!(code, "34863669");
    }
}
