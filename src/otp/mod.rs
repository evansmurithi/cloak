#[derive(Debug)]
pub enum HashFunction {
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    SHA512_256,
}

mod hotp;
pub use self::hotp::HOTP;

mod totp;
pub use self::totp::TOTP;
