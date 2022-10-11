#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::fs;

fn main() {
    println!("Hello, world!");

    let idk = fs::read("asdfasd").unwrap();

    println!("{idk:?}");
}
