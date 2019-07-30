#[macro_use]
extern crate diesel;

mod database;

use database::models::User;

fn main() {
    use database::schema::users::dsl::*;
    use diesel::RunQueryDsl;

    let _connection = database::establish_connection();

    let _results = users.load::<User>(&_connection).expect("Error loading users");

    println!("Displaying {} users", _results.len());
    for user in _results {
        println!("{}", user.name);
        println!("-----------\n");
    }
}

