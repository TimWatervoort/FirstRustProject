// #![recursion_limit = "5"]

extern crate league_project;
extern crate diesel;

use self::league_project::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("Champion: \n");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    println!("\nWhat role do you use {} in?\n", name);
    let mut role = String::new();
    stdin().read_to_string(&mut role).unwrap();
    let role = &role[..(role.len() - 1)]; // Drop the newline character
    println!("\nWhat is your comfort level with {} {}? {} to finish.\n", name, role, EOF);
    let mut comfort = String::new();
    stdin().read_to_string(&mut comfort).unwrap();

    let champion = create_champion(&connection, name, &role, &comfort);
    println!("\nAdded {} {} to your roster at id {}.", name, role, champion.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";
