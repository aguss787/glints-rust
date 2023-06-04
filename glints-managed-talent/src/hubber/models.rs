use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct Hubber {
    pub id: uuid::Uuid,
    pub code: String,
    pub name: String,
}
