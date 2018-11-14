extern crate league_project;
extern crate diesel;

use self::diesel::prelude::*;
use self::league_project::*;
use self::champions::Champion;
use std::env::args;
use std::io::stdin;

fn main () {
    let id = args().nth(1).expect("Give an id please!")
        .parse::<i32>().expect("Not a valid ID.");

    println!("What would you like to update?");
    let mut att = String::new();
    stdin().read_line(&mut att).unwrap();
    let att = &att[..(att.len() - 1)];
    match att {
        "role" => update_role(id),
        "name" => update_name(id),
        "comfort" => update_comfort(id),
        _ => println!("Not a valid attribute"),
    }

}

fn update_role(id:i32) {
    use league_project::schema::champions::dsl::{champions, role};

    let connection = establish_connection();
    println!("New role: ");
    let mut new_role = String::new();
    stdin().read_line(&mut new_role).unwrap();
    let new_role = &new_role[..(new_role.len() - 1)];

    let champion = diesel::update(champions.find(id))
        .set(role.eq(new_role))
        .get_result::<Champion>(&connection)
        .expect(&format!("Unable to find champion {}", id));
    println!("Updated {} to have role {}", champion.name, champion.role);
}

fn update_name(id:i32) {
    use league_project::schema::champions::dsl::{champions, name};

    let connection = establish_connection();
    println!("New name: ");
    let mut new_name = String::new();
    stdin().read_line(&mut new_name).unwrap();
    let new_name = &new_name[..(new_name.len() - 1)];

    let champion = diesel::update(champions.find(id))
        .set(name.eq(new_name))
        .get_result::<Champion>(&connection)
        .expect(&format!("Unable to find champion {}", id));
    println!("Updated {} to have name {}", champion.id, champion.name);
}

fn update_comfort(id:i32) {
    use league_project::schema::champions::dsl::{champions, comfort};
    let connection = establish_connection();
    println!("New comfort: ");
    let mut new_comfort = String::new();
    stdin().read_line(&mut new_comfort).unwrap();
    let new_comfort = &new_comfort[..(new_comfort.len() - 1)];

    let champion = diesel::update(champions.find(id))
        .set(comfort.eq(new_comfort))
        .get_result::<Champion>(&connection)
        .expect(&format!("Unable to find champion {}", id));
    println!("Updated {} to have comfort {}", champion.name, champion.comfort);
}
