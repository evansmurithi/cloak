use clap::{App, SubCommand};
use fs;
use otp::OTP;

// `list` subcommand
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list").about("List OTP for all accounts")
}

// Implementation for the `list` subcommand
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
                match otp {
                    Ok(otp) => {
                        if account.totp {
                            println!("Account: {}\nTOTP: {}", name, otp.generate());
                        } else {
                            println!("Account: {}\nHOTP: {}", name, otp.generate());
                        }
                        println!("\n");
                    }
                    Err(err) => eprintln!("{}", err),
                }
            }
        }
        Err(err) => eprintln!("{}", err),
    };
}
