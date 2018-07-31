use clap::{App, Arg, ArgMatches, SubCommand};
use fs;
use open;

pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("recovery_codes")
        .about("View recovery codes for an account")
        .arg(
            Arg::with_name("account")
                .required(true)
                .help("Name of the account"),
        )
}

pub fn run(args: &ArgMatches) {
    let account_name = args.value_of("account").unwrap();
    match fs::recovery_codes(account_name) {
        Ok(file_path) => {
            match open::that(file_path) {
                Ok(_) => {}
                Err(err) => eprintln!("{}", err),
            };
        }
        Err(err) => eprintln!("{}", err),
    };
}
