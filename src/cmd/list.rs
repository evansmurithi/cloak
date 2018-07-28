use clap::{App, SubCommand};
use data_encoding::BASE32_NOPAD;
use fs;
use otp::OTP;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list").about("List OTP for all accounts")
}

pub fn run() {
    match fs::read() {
        Ok(accounts) => {
            for (name, account) in accounts {
                let decoded_key = BASE32_NOPAD.decode(account.key.as_bytes()).unwrap();
                let otp = OTP::new(
                    decoded_key,
                    account.totp,
                    &account.hash_function,
                    account.counter,
                    None,
                );
                if account.totp {
                    println!("Account: {}\nTOTP: {}", name, otp.generate());
                } else {
                    println!("Account: {}\nHOTP: {}", name, otp.generate());
                }
                println!("\n");
            }
        }
        Err(err) => println!("Error {}", err),
    };
}
