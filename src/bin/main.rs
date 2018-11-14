#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate league_project;
extern crate diesel;

use self::league_project::*;
use self::champions::*;
use self::diesel::prelude::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/champions")]
fn get_champions() -> String {
    use league_project::schema::champions::dsl::{champions};

    let connection = establish_connection();

    let results = champions
        .load::<Champion>(&connection)
        .expect("Error loading champions");

    let mut result_string = String::new();

    for x in results {
        result_string = result_string + &format!("{} {}: {}", x.name, x.role, x.comfort);
    }

    result_string

}

#[get("/champions/<id>")]
fn get_champion(id: i32) -> String {

    use league_project::schema::champions::dsl::{champions};

    let connection = establish_connection();

    let champion = champions.find(id)
        .get_result::<Champion>(&connection)
        .expect(&format!("Unable to find champion {:?}", id));

    format!("{} {}: {}", champion.name, champion.role, champion.comfort )
}

#[post("/champions/<name>/<role>/<comfort>")]
fn post_champion(name:String, role:String, comfort:String) -> String {
    let connection = establish_connection();
    let champion = create_champion(&connection, &name, &role, &comfort);

    format!("Added {} {} to your roster.", champion.name, champion.role)
}

#[put("/champions/<id>/<new_name>/<new_role>/<new_comfort>")]
fn put_champion(id:i32, new_name:String, new_role:String, new_comfort:String) -> String {
    use league_project::schema::champions::dsl::{champions, comfort, role, name};
    let connection = establish_connection();

    diesel::update(champions.find(id))
        .set(name.eq(new_name))
        .get_result::<Champion>(&connection)
        .expect(&format!("Unable to find champion {}", id));

    diesel::update(champions.find(id))
        .set(role.eq(new_role))
        .get_result::<Champion>(&connection)
        .expect(&format!("Unable to find champion {}", id));

    let champion = diesel::update(champions.find(id))
        .set(comfort.eq(new_comfort))
        // .set(name.eq(new_name))
        // .set(role.eq(new_role))
        .get_result::<Champion>(&connection)
        .expect(&format!("Unable to find champion {}", id));

    format!("Updated {} {}.", champion.name, champion.role)
}

#[delete("/champions/<id>")]
fn delete_champion(id:i32) -> String {
    use league_project::schema::champions::dsl::{champions};
    let connection = establish_connection();

    let champion = diesel::delete(champions.find(id))
        .get_result::<Champion>(&connection)
        .expect(&format!("Unable to find champion {}", id));

    format!("Deleted {} {}", champion.name, champion.role)
}

fn main() {
    rocket::ignite().mount("/", routes![index, get_champions, get_champion, post_champion, put_champion, delete_champion]).launch();
}
