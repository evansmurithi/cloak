use account::{Account, AccountStore};
use clap::{App, Arg, ArgMatches};
use data_encoding::BASE32_NOPAD;

// Create arguments for `add` subcommand
pub fn subcommand<'a>() -> App<'a> {
    App::new("add")
        .about("Add a new account")
        .arg(
            Arg::new("account")
                .required(true)
                .help("Name of the account"),
        )
        .arg(
            Arg::new("key")
                .required(true)
                .help("Secret key of the OTP")
                .validator(is_base32_key),
        )
        .arg(
            Arg::new("totp")
                .long("totp")
                .conflicts_with("hotp")
                .help("Time based account (default)"),
        )
        .arg(
            Arg::new("hotp")
                .long("hotp")
                .help("Counter based account"),
        )
        .arg(
            Arg::new("algorithm")
                .short('a')
                .long("algorithm")
                .takes_value(true)
                .possible_values(&["SHA1", "SHA256", "SHA384", "SHA512", "SHA512_256"])
                .value_name("ALGORITHM")
                .help("Algorithm to use to generate the OTP code"),
        )
}

// Validate key provided in arguments is a valid base32 encoding
fn is_base32_key(value: &str) -> Result<(), String> {
    let value = value.to_uppercase();
    match BASE32_NOPAD.decode(value.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("the key is not a valid base32 encoding")),
    }
}

// Implementation for the `add` subcommand
pub fn run(args: &ArgMatches, account_store: &mut AccountStore) {
    let totp = !args.is_present("hotp");
    let hash_function = args.value_of("algorithm").unwrap_or("SHA1");
    let account_name = args.value_of("account").unwrap();
    let key = args.value_of("key").unwrap().to_uppercase();

    let counter = if !totp { Some(0) } else { None };
    let account = Account {
        key,
        totp,
        hash_function: hash_function.to_string(),
        counter,
    };

    if account_store.get(account_name).is_some() {
        println!("Account already exists");
    } else {
        account_store.add(account_name.to_string(), account);
        match account_store.save() {
            Ok(_) => println!("Account successfully created"),
            Err(err) => eprintln!("{}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_base32_key() {
        let result = super::is_base32_key("12123EQ");
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(String::from("the key is not a valid base32 encoding"))
        );

        let result = super::is_base32_key("4AZJFQFIGYM2KMTOO72I6FAOZ6ZFWJR6");
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(()));
    }
}
