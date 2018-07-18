use super::{HashFunction, HOTP};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct TOTP {
    secret_key: Vec<u8>,
    period: u64,
    output_len: usize,
    output_base: Vec<u8>,
    hash_function: HashFunction,
}

impl TOTP {
    pub fn new(secret_key: Vec<u8>, period: u64, output_len: usize) -> TOTP {
        TOTP {
            secret_key: secret_key,
            period: period,
            output_len: output_len,
            output_base: "0123456789".to_owned().into_bytes(),
            hash_function: HashFunction::SHA1,
        }
    }

    pub fn generate(&self) -> String {
        let counter = self.get_counter();
        let hotp = HOTP::new(
            self.secret_key.to_owned(), counter, self.output_len);
        hotp.generate()
    }

    fn get_counter(&self) -> u64 {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;
        timestamp / self.period
    }


}
