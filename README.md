# cloak

A Command Line OTP Authenticator application written in Rust that generates
time-based and counter-based OTP codes.

[![Linux build status](https://travis-ci.com/evansmurithi/cloak.svg?branch=master)](https://travis-ci.com/evansmurithi/cloak)
[![Windows build status](https://ci.appveyor.com/api/projects/status/9mlfpfru3ng4c689?svg=true)](https://ci.appveyor.com/project/evansmurithi/cloak)

## Motivation

- [Why you shouldn’t scan two-factor authentication QR codes!](https://medium.com/crypto-punks/why-you-shouldnt-scan-two-factor-authentication-qr-codes-e2a44876a524)
- As a means of learning the Rust programming language.
- Easier to copy the OTP code from my terminal to the login form, rather than from
my phone to my laptop.

## Example

<p align="center">
    <img src="/tmp/cloak_example.svg">
</p>

## Installation

To install `cloak`, you can do either of the following:

1. **Binaries**

    You can download the binaries [here](https://github.com/evansmurithi/cloak/releases).
    Make sure to put the path to the binary into your `PATH`.

2. **Crates.io**

    Once you've installed Rust, install `cloak` by typing the following in the terminal:

    ```bash
    cargo install cloak
    ```

    This will download and compile `cloak`. After this is finished, add the Cargo
    bin directory to your `PATH`.

## Usage

The sub-commands included in `cloak` are as follows:

```bash
$ cloak -h
cloak 0.1.0
Evans Murithi <murithievans80@gmail.com>
A Command Line OTP Authenticator application.

USAGE:
    cloak [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add               Add a new account
    delete            Delete an account
    help              Prints this message or the help of the given subcommand(s)
    list              List OTP for all accounts
    recovery_codes    View recovery codes for an account
    view              View the OTP for an account
```

To view the help of any of the subcommands below, add `-h` or `--help`, e.g. `cloak add -h`.

- `cloak add <account> <key>`

    This will add a new account. You will need to provide the name of the account
    as well as valid base32 encoded key. Example:

    ```bash
    $ cloak add github 4AZJFQFIGYM2KMTOO72I6FAOZ6ZFWJR6
    ```

- `cloak view <account>`

    This will print the TOTP/HOTP of the account you want to view. Example:

    ```bash
    $ cloak view github
    123456
    ```

- `cloak list`

    This prints all the accounts with their respective TOTP/HOTP codes. Example:

    ```bash
    $ cloak list
    Account: github
    TOTP: 607091

    Account: gitlab
    TOTP: 325414
    ```

- `cloak delete <account>`

    This will delete an account. Once deleted, you cannot view the OTP codes for
    the account. Example:

    ```bash
    $ cloak delete github
    Account successfully deleted
    $ cloak view github
    Account with the name 'github' does not exist. Consider adding it.
    ```

- `cloak recovery_codes <account>`

    This will open a file, using your preferred editor, to either save new recovery
    codes or view existing recovery codes.

## Contributions

If you want to contribute to `cloak` you will have to clone the repository on your
local machine:

```bash
$ git clone https://github.com/evansmurithi/cloak.git
```

To build, `cd` into `cloak/` and run:

```bash
$ cargo build
```

To run tests:

```bash
$ cargo test
```
