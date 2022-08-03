use crate::account::AccountStore;
use crate::otp::OneTimePassword;
use clap::{value_parser, Arg, ArgMatches, Command};

// Create arguments for `view` subcommand
pub fn subcommand<'a>() -> Command<'a> {
    Command::new("view")
        .about("View the OTP for an account")
        .arg(
            Arg::new("account")
                .required(true)
                .help("Name of the account"),
        )
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .takes_value(true)
                .value_name("NUMBER")
                .default_value("6")
                .help("Length of the OTP")
                .value_parser(value_parser!(usize)),
        )
}

// Implementation for the `view` subcommand
pub fn run(args: &ArgMatches, account_store: &mut AccountStore) {
    let length = args.get_one::<usize>("length").unwrap();
    let account_name = args.get_one::<String>("account").unwrap();
    match account_store.get(account_name) {
        Some(account) => {
            let otp = OneTimePassword::new(
                &account.key,
                account.totp,
                &account.hash_function,
                account.counter,
                Some(*length),
            );
            match otp {
                Ok(otp) => {
                    println!("{}", otp.generate());
                }
                Err(err) => eprintln!("{}", err),
            }
        }
        None => println!(
            "Account with the name '{}' does not exist. Consider adding it.",
            account_name
        ),
    };
}
