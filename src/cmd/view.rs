use clap::{App, Arg, ArgMatches, SubCommand};
use fs;
use otp::OTP;

// Create arguments for `view` subcommand
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("view")
        .about("View the OTP for an account")
        .arg(
            Arg::with_name("account")
                .required(true)
                .help("Name of the account"),
        )
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .takes_value(true)
                .value_name("NUMBER")
                .help("Length of the OTP")
                .validator(is_number),
        )
}

// Validate length provided in arguments is a number
fn is_number(value: String) -> Result<(), String> {
    match value.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("length must be a number")),
    }
}

// Implementation for the `view` subcommand
pub fn run(args: &ArgMatches) {
    let length = match args.value_of("length") {
        Some(length) => length.parse::<usize>().unwrap(),
        None => 6,
    };
    let account_name = args.value_of("account").unwrap();
    match fs::read() {
        Ok(accounts) => match accounts.get(account_name) {
            Some(account) => {
                let otp = OTP::new(
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
        },
        Err(err) => eprintln!("{}", err),
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_number() {
        let result = super::is_number(String::from("meow"));
        assert!(result.is_err());
        assert_eq!(result.err(), Some(String::from("length must be a number")));

        let result = super::is_number(String::from("8"));
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(()));
    }
}
