use async_graphql::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

// creates a database connection / used by Diesel during queries
pub fn establish_connection() -> SqliteConnection {
    // assign the location to the sqlite database
    let database_url = "rusty_api.sqlite";
    // establish and return the connection
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// create a struct describing the data within a post
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub published: bool,
}

// create rules for how the data returned by the query should be treated
#[Object]
impl Post {
    pub async fn id(&self) -> &String { &self.id }

    pub async fn title(&self) -> &String {
        &self.title
    }

    pub async fn body(&self) -> &String {
        &self.body
    }

    pub async fn published(&self) -> &bool {
        &self.published
    }

}

pub struct Query;

#[Object]
impl Query {

    // Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    // get a post by id
    async fn get_post(&self, post_id: String) -> Vec<crate::graphql::Post> {
        // use crate::models::Post;
        use crate::schema::posts::dsl::*;

        // create the connection
        let connection:SqliteConnection = establish_connection();

        // make the query and return the results
        let results = posts
            .find(post_id)
            .load::<crate::models::Post>(&connection)
            .expect("Error loading posts");

        // convert the model struct to the struct used locally
        // TODO: find a better way to do this without having to map from one struct to another
        results.into_iter().map(|x| crate::graphql::Post{
            id: x.id,
            title: x.title,
            body: x.body,
            published: x.published
        }).rev().collect()
    }

    // get all of the posts
    async fn all_posts(&self) -> Vec<crate::graphql::Post> {
        // use crate::models::Post;
        use crate::schema::posts::dsl::*;

        // create the connection
        let connection:SqliteConnection = establish_connection();

        // make the query and return the results
        let results = posts
            .load::<crate::models::Post>(&connection)
            .expect("Error loading posts");

        // convert the model struct to the struct used locally
        // TODO: find a better way to do this without having to map from one struct to another
        results.into_iter().map(|x| crate::graphql::Post{
            id: x.id,
            title: x.title,
            body: x.body,
            published: x.published
        }).rev().collect()
    }


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

}