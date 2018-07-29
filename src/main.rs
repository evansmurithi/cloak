#[macro_use]
extern crate clap;
extern crate data_encoding;
extern crate dirs;
extern crate open;
extern crate ring;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use clap::App;

mod cmd;
mod fs;
mod otp;

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(cmd::add::subcommand())
        .subcommand(cmd::view::subcommand())
        .subcommand(cmd::list::subcommand())
        .subcommand(cmd::delete::subcommand())
        .subcommand(cmd::recovery_codes::subcommand())
        .get_matches();

    match matches.subcommand() {
        ("add", Some(sub_m)) => cmd::add::run(&sub_m),
        ("view", Some(sub_m)) => cmd::view::run(&sub_m),
        ("list", Some(_)) => cmd::list::run(),
        ("delete", Some(sub_m)) => cmd::delete::run(&sub_m),
        ("recovery_codes", Some(sub_m)) => cmd::recovery_codes::run(&sub_m),
        _ => {
            eprintln!("No subcommand chosen");
            std::process::exit(1);
        },
    }
}
