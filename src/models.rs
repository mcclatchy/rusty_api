use super::schema::posts;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[derive(Queryable, Insertable)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub published: bool,
}
