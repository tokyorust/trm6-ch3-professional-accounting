#![feature(plugin)]
#![feature(custom_derive)]

#![plugin(clippy)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use std::fs::File;

#[derive(Debug, PartialEq, Deserialize)]
struct Account {
    id: String,
    transactions: Vec<Transaction>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Transaction {
    id: String,
    amount: i64,
}

fn main() {
    let file = File::open("account.json").unwrap();

    let account: Account = serde_json::from_reader(file).unwrap();

    let mut total = 0;

    for transaction in account.transactions {
        println!("Spent {} credits on transaction {}", transaction.amount, transaction.id);
        total += transaction.amount;
    }

    println!("Balance of account {} is {}", account.id, total);
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
