use clap::{App, Arg, ArgMatches, SubCommand};
use fs;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add")
        .about("Add a new account")
        .arg(
            Arg::with_name("account")
                .required(true)
                .help("Name of the account"),
        )
        .arg(
            Arg::with_name("key")
                .required(true)
                .help("Secret key of the OTP"),
        )
        .arg(
            Arg::with_name("totp")
                .long("totp")
                .conflicts_with("hotp")
                .help("Time based account (default)"),
        )
        .arg(
            Arg::with_name("hotp")
                .long("hotp")
                .help("Counter based account"),
        )
        .arg(
            Arg::with_name("algorithm")
                .short("a")
                .long("algorithm")
                .takes_value(true)
                .possible_values(&["SHA1", "SHA256", "SHA384", "SHA512", "SHA512_256"])
                .value_name("ALGORITHM")
                .help("Algorithm to use to generate the OTP code"),
        )
}

pub fn run(args: &ArgMatches) {
    let totp = !args.is_present("hotp");
    let hash_function = match args.value_of("algorithm") {
        Some(algorithm) => algorithm,
        None => "SHA1",
    };
    let account_name = args.value_of("account").unwrap();
    let key = args.value_of("key").unwrap();
    match fs::read() {
        Ok(mut accounts) => {
            let mut counter = if !totp { Some(0) } else { None };
            let account = fs::Account {
                key: key.to_string(),
                totp,
                hash_function: hash_function.to_string(),
                counter,
            };

            if accounts.get(account_name).is_some() {
                println!("Account already exists");
            } else {
                accounts.insert(account_name.to_string(), account);
                match fs::write(&accounts) {
                    Ok(_) => println!("Account successfully created"),
                    Err(err) => println!("Error {}", err),
                };
            }
        }
        Err(err) => println!("Error {}", err),
    };
}
