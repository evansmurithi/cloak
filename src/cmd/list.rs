use crate::account::AccountStore;
use crate::otp::OneTimePassword;
use clap::App;

// `list` subcommand
pub fn subcommand<'a>() -> App<'a> {
    App::new("list").about("List OTP for all accounts")
}

// Implementation for the `list` subcommand
pub fn run(account_store: &mut AccountStore) {
    let accounts = account_store.list();

    for (name, account) in accounts {
        let otp = OneTimePassword::new(
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
