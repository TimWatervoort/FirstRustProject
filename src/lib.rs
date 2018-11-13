#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod champions;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::champions::{Champion, NewChampion};

pub fn create_champion<'a>(conn: &PgConnection, name: &'a str, role: &'a str, comfort: &'a str) -> Champion {
    use self::schema::champions;

    let new_champion = NewChampion {
        name: name,
        role: role,
        comfort: comfort,
    };

    diesel::insert_into(champions::table)
        .values(&new_champion)
        .get_result(conn)
        .expect("Error saving new champion")
}
