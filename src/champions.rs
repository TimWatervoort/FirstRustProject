#[derive(Queryable)]
pub struct Champion {
    pub id: i32,
    pub name: String,
    pub role: String,
    pub comfort: String,
}

use super::schema::champions;

#[derive(Insertable)]
#[table_name="champions"]
pub struct NewChampion<'a> {
    pub name: &'a str,
    pub role: &'a str,
    pub comfort: &'a str,
}
