extern crate league_project;
extern crate diesel;

use self::league_project::*;
use self::champions::*;
use self::diesel::prelude::*;

fn main() {
    use league_project::schema::champions::dsl::*;

    let connection = establish_connection();
    let results = champions
        .limit(5)
        .load::<Champion>(&connection)
        .expect("Error loading champions");

    println!("Displaying {} champions", results.len());
    for champion in results {
        println!("{} {}: {}", champion.name, champion.role, champion.comfort);
    }
}
