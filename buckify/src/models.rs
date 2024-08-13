use crate::schema::resources;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::resources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct Resource {
    pub id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub slug: String,
    pub path: String,
    pub size: i32,
    pub public: bool,
}

#[derive(Insertable)]
#[diesel(table_name = resources)]
pub struct NewResource<'a> {
    pub name: &'a str,
    pub slug: &'a str,
    pub path: &'a str,
    pub size: i32,
}
