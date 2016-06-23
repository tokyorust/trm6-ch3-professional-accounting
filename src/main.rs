#![feature(plugin)]
#![feature(custom_derive)]

#![plugin(clippy)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{}: {}", StdError::description(self), err),
            Error::Json(ref err) => write!(f, "{}: {}", StdError::description(self), err),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "I/O error",
            Error::Json(_) => "JSON error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Deserialize)]
struct Account {
    id: String,
    transactions: Vec<Transaction>,
}

impl Account {
    fn load<P: AsRef<Path>>(path: P) -> Result<Account> {
        let reader = try!(File::open(path));
        let account: Account = try!(serde_json::from_reader(reader));
        Ok(account)
    }

    fn balance(&self) -> i64 {
        self.transactions.iter().fold(0, |acc, ref tx| acc + tx.amount)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct Transaction {
    id: String,
    amount: i64,
}

fn main() {
    let account = match Account::load("account.json") {
        Ok(account) => account,
        Err(err) => panic!("Aaand it's all gone: {}", err),
    };
    println!("Balance of account {} is {}", account.id, account.balance());
}

#[test]
fn should_load_account() {
    Account::load("account.json").unwrap();
}

#[test]
fn should_calculate_balance() {
    let account = Account::load("account.json").unwrap();
    assert_eq!(account.balance(), 4536);
}
