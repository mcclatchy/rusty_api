use super::schema::posts;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub id: String,
    pub title: String,
    pub body: String,
}
