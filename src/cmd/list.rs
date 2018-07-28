use clap::{App, SubCommand};
use fs;
use otp::OTPBuilder;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list").about("List OTP for all accounts")
}

pub fn run() {
    match fs::read() {
        Ok(accounts) => {
            for (name, account) in accounts {
                let counter = match account.counter {
                    Some(count) => count,
                    None => 0,
                };
                let otp = OTPBuilder::new()
                    .key(&account.key)
                    .hash_function(&account.hash_function)
                    .totp(account.totp)
                    .counter(counter)
                    .finalize();
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
