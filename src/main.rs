#![feature(plugin)]
#![feature(custom_derive)]

#![plugin(clippy)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    NegativeBalance(i64),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{}: {}", StdError::description(self), err),
            Error::Json(ref err) => write!(f, "{}: {}", StdError::description(self), err),
            Error::NegativeBalance(balance) => write!(f, "Negative balance of {} credits", balance),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "I/O error",
            Error::Json(_) => "JSON error",
            Error::NegativeBalance(_) => "Negative balance",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            Error::NegativeBalance(_) => None,
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

    fn balance(&self) -> Result<i64> {
        let balance = self.transactions.iter().fold(0, |acc, ref tx| acc + tx.amount);
        if balance < 0 {
            return Err(Error::NegativeBalance(balance))
        }
        Ok(balance)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct Transaction {
    id: String,
    amount: i64,
}

fn main() {
    let filename = env::var("ACCOUNT_FILE").unwrap_or("account.json".to_owned());
    let account = Account::load(filename).unwrap();
    let balance = account.balance().expect("Impossible balance");
    println!("Balance of account {} is {}", account.id, balance);
}

#[test]
fn should_load_account() {
    Account::load("account.json").unwrap();
}

#[test]
fn should_calculate_balance() {
    let account = Account::load("account.json").unwrap();
    match account.balance() {
        Ok(_) => unreachable!(),
        Err(Error::NegativeBalance(balance)) => assert_eq!(balance, -5464),
        Err(_) => unreachable!(),
    }
}
