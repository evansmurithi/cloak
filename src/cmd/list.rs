use account::AccountStore;
use clap::{App, SubCommand};
use otp::OTP;

// `list` subcommand
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("list").about("List OTP for all accounts")
}

// Implementation for the `list` subcommand
pub fn run(account_store: &mut AccountStore) {
    let accounts = account_store.list();

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
