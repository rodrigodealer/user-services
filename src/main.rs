#[macro_use]
extern crate diesel;

// use diesel::prelude::*;
// use database::models::User;
// use std::env::args;

mod database;

fn main() {
    let connection = database::establish_connection();
    println!("Hello World!");
}

