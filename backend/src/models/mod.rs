use diesel::prelude::*;
use serde::Serialize;
use pointbreak::schema;

#[derive(Serialize, Selectable, Queryable)]
#[diesel(table_name = schema::users)]
pub struct User {
    id: i32,
    name: String,
    hair_color: Option<String>,
}
