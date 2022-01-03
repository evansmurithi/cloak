use account::AccountStore;
use clap::{App, Arg, ArgMatches};
use std::io::{self, Write};

// Create arguments for `delete` subcommand
pub fn subcommand<'a, 'b>() -> App<'a> {
    App::new("delete")
        .about("Delete an account")
        .arg(
            Arg::new("account")
                .required(true)
                .help("Name of the account"),
        )
}

// Implementation for the `delete` subcommand
pub fn run(args: &ArgMatches, account_store: &mut AccountStore) {
    let account_name = args.value_of("account").unwrap();
    print!("Are you sure you want to delete {} [N/y]? ", account_name);
    io::stdout().flush().unwrap();
    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_) => {
            if answer.trim().to_lowercase() == "y" {
                if account_store.get(account_name).is_some() {
                    account_store.delete(account_name);
                    match account_store.save() {
                        Ok(_) => println!("Account successfully deleted"),
                        Err(err) => eprintln!("{}", err),
                    };
                } else {
                    println!("Account does not exist");
                }
            } else {
                println!("Abort.");
            }
        }
        Err(_) => eprintln!("Failed to read input"),
    };
}
