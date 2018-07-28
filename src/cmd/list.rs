use clap::{App, SubCommand};
use fs;
use otp::OTP;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list").about("List OTP for all accounts")
}

pub fn run() {
    match fs::read() {
        Ok(accounts) => {
            for (name, account) in accounts {
                let otp = OTP::new(
                    &account.key,
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
