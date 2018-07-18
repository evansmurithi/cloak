#[macro_use]
extern crate clap;
extern crate ring;
extern crate data_encoding;

use clap::{App, Arg, ArgMatches, SubCommand};
use data_encoding::BASE32_NOPAD;

mod otp;

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(SubCommand::with_name("add")
            .about("Add a new account")
            .arg(Arg::with_name("account")
                .required(true)
                .help("Name of the account"))
            .arg(Arg::with_name("secret_key")
                .required(true)
                .help("Secret key of the OTP"))
            .arg(Arg::with_name("counter")
                .required(true)
                .help("Counter")))
        .subcommand(SubCommand::with_name("view")
            .about("View the OTP for an account")
            .arg(Arg::with_name("account")
                .required(true)
                .help("Name of the account")))
        .subcommand(SubCommand::with_name("list")
            .about("List your accounts"))
        .subcommand(SubCommand::with_name("edit")
            .about("Edit an account")
            .arg(Arg::with_name("account")
                .required(true)
                .help("Name of the account")))
        .get_matches();

    run(matches)
}

fn run(matches: ArgMatches) {
    match matches.subcommand() {
        ("add", Some(sub_m)) => {
            add_account(
                sub_m.value_of("account").unwrap(),
                sub_m.value_of("secret_key").unwrap(),
                sub_m.value_of("counter").unwrap().parse::<u64>().unwrap())
        },
        ("view", Some(_sub_m)) => {},
        ("list", Some(_sub_m)) => {},
        ("edit", Some(_sub_m)) => {},
        _ => {println!("No subcommand chosen")},
    }
}

fn add_account(account: &str, secret_key: &str, counter: u64) {
    println!("{:?}", account);
    let totp = otp::TOTP::new(BASE32_NOPAD.decode(secret_key.as_bytes()).unwrap(), 30, 6);
    let hotp = otp::HOTP::new(BASE32_NOPAD.decode(secret_key.as_bytes()).unwrap(), counter, 6);
    println!("TOTP{:?}", totp.generate());
    println!("HOTP{:?}", hotp.generate());
}
