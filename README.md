# Challenge #3 - Professional accounting

For [Tokyo Rust Meetup's mini-hackathon on 2016-06-23](http://www.meetup.com/Tokyo-Rust-Meetup/events/231555496/).

## Setup

For this task, you MUST have a specific Rust nightly because the dependencies haven't caught up yet. Otherwise it just won't compile, as one of the dependencies is internally very complex.

```sh
rustup toolchain install nightly-2016-06-16
rustup override add nightly-2016-06-16
```

## The challenge

You have a highly professional JSON database of an account's transactions in [account.json](account.json). While ugly, the code does work. It isn't quite up to par in many ways, as it doesn't offer a way of gracefully recovering from an error, such as an I/O error or a JSON parsing error. It is therefore unusable in a library. Our target is to make `main` look no more complex than:

```rust
fn main() {
    let account = match Account::load("account.json") {
        Ok(account) => account,
        Err(err) => panic!("Aaand it's all gone: {}", err),
    };
    println!("Balance of account {} is {}", account.id, account.balance());
}
```

_Note: using `.unwrap()` or other panic triggering functions is OK in `main`, as that's the last place within your control anyway. Within a library, however, it's unacceptable (with the exception of correct use of  `unreachable!()`)._

Also, try introducing a syntax error to the JSON file to trigger a parsing error.

Making `cargo test` pass may help you on the way, but this time you have to write more code on your own!

## (Optional bonus challenge 1)

Skip this one if you're short on time and move on to optional bonus challenge 2.

Make `Account::load()` accept all of `&str`, `String` and `Path`. Hint: some of the work may have been done for you by the `std` library.

## (Optional bonus challenge 2)

Add or change a transaction in the account database so that the final balance is below zero. This will be considered an error, as our supreme enterprise system should not have let that happen in the first place.

Create a new error, `NegativeBalance`, and make `balance()` return a `Result` with either a positive balance or that error type. Also, `NegativeBalance` should store and allow access to the clearly impossible negative value.

Your `main` should then look something like this:

```rust
fn main() {
    let account = Account::load("account.json").unwrap();
    let balance = account.balance().expect("Impossible balance");
    println!("Balance of account {} is {}", account.id, balance);
}
```

## (Optional bonus challenge 3)

Make it possible to specify the account filename using an environment variable.
