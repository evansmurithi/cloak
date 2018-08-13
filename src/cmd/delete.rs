use clap::{App, Arg, ArgMatches, SubCommand};
use fs;
use std::io::{self, Write};

// Create arguments for `delete` subcommand
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("delete")
        .about("Delete an account")
        .arg(
            Arg::with_name("account")
                .required(true)
                .help("Name of the account"),
        )
}

// Implementation for the `delete` subcommand
pub fn run(args: &ArgMatches) {
    let account_name = args.value_of("account").unwrap();
    print!("Are you sure you want to delete {} [N/y]? ", account_name);
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_) => {
            if answer.trim().to_lowercase() == "y" {
                match fs::read() {
                    Ok(mut accounts) => {
                        if accounts.get(account_name).is_some() {
                            accounts.remove(account_name);
                            match fs::write(&accounts) {
                                Ok(_) => println!("Account successfully deleted"),
                                Err(err) => eprintln!("{}", err),
                            };
                        } else {
                            println!("Account does not exist");
                        }
                    }
                    Err(err) => eprintln!("{}", err),
                };
            } else {
                println!("Abort.");
            }
        }
        Err(_) => eprintln!("Failed to read input"),
    };
}
