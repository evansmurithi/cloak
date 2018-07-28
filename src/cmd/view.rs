use clap::{App, Arg, ArgMatches, SubCommand};
use fs;
use otp::OTPBuilder;

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
                .help("Length of the OTP"),
        )
}

pub fn run(args: &ArgMatches) {
    let length = match args.value_of("length") {
        Some(length) => length.parse::<usize>().unwrap(),
        None => 6,
    };
    let account_name = args.value_of("account").unwrap();
    match fs::read() {
        Ok(accounts) => match accounts.get(account_name) {
            Some(account) => {
                let counter = match account.counter {
                    Some(count) => count,
                    None => 0,
                };
                let otp = OTPBuilder::new()
                    .key(&account.key)
                    .hash_function(&account.hash_function)
                    .totp(account.totp)
                    .counter(counter)
                    .output_len(length)
                    .finalize();
                println!("{}", otp.generate());
            }
            None => println!(
                "Account with the name {} does not exist. Consider adding it.",
                account_name
            ),
        },
        Err(err) => println!("Error {}", err),
    };
}
