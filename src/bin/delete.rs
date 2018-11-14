extern crate league_project;
extern crate diesel;

use self::diesel::prelude::*;
use self::league_project::*;
use self::champions::Champion;
use std::env::args;

fn main () {
    let id = args().nth(1).expect("Give an id please!")
        .parse::<i32>().expect("Not a valid ID.");

    use league_project::schema::champions::dsl::{champions};

    let connection = establish_connection();
    let champion = diesel::delete(champions.find(id))
        .get_result::<Champion>(&connection)
        .expect(&format!("Unable to find champion at {}", id));

    println!("Deleted champion {} at id {}", champion.name, id);
}
