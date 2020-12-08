pub mod schema;
pub mod models;
pub mod graphql;

#[macro_use]
extern crate diesel;

//
// pub fn create_post<'a>(new_title: String, new_body: String) -> Post {
//     let connection = establish_connection();
//     // create a randomly generated string as the primary key
//     let uuid = Uuid::new_v4().to_hyphenated().to_string();
//     // create the new post
//     // pull in the posts table
//     use schema::posts;
//     let new_post = NewPost {
//         id: uuid.clone(),
//         title: new_title.clone(),
//         body: new_body.clone(),
//     };
//     // add new post to the db
//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .execute(&connection)
//         .expect("Error saving new post");
//     // return the newly created record from the db
//     Post {
//         id: uuid.clone(),
//         title: new_title.clone(),
//         body: new_body.clone(),
//         published: false
//     }
// }

