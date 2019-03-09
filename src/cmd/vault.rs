use clap::{App, SubCommand};
use rpassword;
use vault;

// `vault` subcommand
pub fn subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("vault").about("Test vault")
}

// Implementation for the `vault` subcommand
pub fn run() {
    let content = "This is a test document. A very secret one";
    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    let vault = vault::encrypt(content, &pass);
    let ciphertext = vault.unwrap();
    println!("ciphertext {:?}", ciphertext);

    let pass = rpassword::read_password_from_tty(Some("Password: ")).unwrap();
    let plaintext = vault::decrypt(&ciphertext, &pass).unwrap();
    println!("plaintext {:?}", plaintext);
}
