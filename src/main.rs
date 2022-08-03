extern crate clap;
extern crate data_encoding;
extern crate dirs_next;
extern crate ring;
extern crate serde;
extern crate thiserror;
#[macro_use]
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate lazy_static;

use crate::account::AccountStore;
use clap::command;

mod account;
mod cmd;
mod dirs;
mod errors;
mod otp;

fn main() {
    // Define list of subcommand for the `cloak` app
    let matches = command!()
        .subcommand(cmd::add::subcommand())
        .subcommand(cmd::view::subcommand())
        .subcommand(cmd::list::subcommand())
        .subcommand(cmd::delete::subcommand())
        .get_matches();

    let mut account_store = AccountStore::new().expect("Unable to initialize store");

    match matches.subcommand() {
        Some(("add", sub_m)) => cmd::add::run(sub_m, &mut account_store),
        Some(("view", sub_m)) => cmd::view::run(sub_m, &mut account_store),
        Some(("list", _)) => cmd::list::run(&mut account_store),
        Some(("delete", sub_m)) => cmd::delete::run(sub_m, &mut account_store),
        _ => eprintln!("No subcommand chosen. Add --help | -h to view the subcommands."),
    }
}
