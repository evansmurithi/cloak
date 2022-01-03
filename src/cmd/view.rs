use account::AccountStore;
use clap::{App, Arg, ArgMatches};
use otp::OneTimePassword;

// Create arguments for `view` subcommand
pub fn subcommand<'a>() -> App<'a> {
    App::new("view")
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
                .help("Length of the OTP")
                .validator(is_number),
        )
}

// Validate length provided in arguments is a number
fn is_number(value: &str) -> Result<(), String> {
    match value.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("length must be a number")),
    }
}

// Implementation for the `view` subcommand
pub fn run(args: &ArgMatches, account_store: &mut AccountStore) {
    let length = match args.value_of("length") {
        Some(length) => length.parse::<usize>().unwrap(),
        None => 6,
    };
    let account_name = args.value_of("account").unwrap();
    match account_store.get(account_name) {
        Some(account) => {
            let otp = OneTimePassword::new(
                &account.key,
                account.totp,
                &account.hash_function,
                account.counter,
                Some(length),
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_number() {
        let result = super::is_number("meow");
        assert!(result.is_err());
        assert_eq!(result.err(), Some(String::from("length must be a number")));

        let result = super::is_number("8");
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(()));
    }
}
