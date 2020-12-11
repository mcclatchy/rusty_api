use async_graphql::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use uuid::Uuid;

// creates a database connection / used by Diesel during queries
pub fn establish_connection() -> SqliteConnection {
    // get the DB_URL const from lib.rs
    let database_url = crate::Config::get_configuration().database_url;
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

    pub async fn id(&self) -> &String {
        &self.id
    }

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
        // TODO: split this off into it's own function
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
        // TODO: split this off into it's own function
        results.into_iter().map(|x| crate::graphql::Post{
            id: x.id,
            title: x.title,
            body: x.body,
            published: x.published
        }).rev().collect()
    }

}

pub struct NewPost {
    pub id: String,
    pub title: String,
    pub body: String,
}

// create rules for how the data returned by the query should be treated
#[Object]
impl NewPost {
    pub async fn id(&self) -> &String { &self.id }
    pub async fn title(&self) -> &String {
        &self.title
    }
    pub async fn body(&self) -> &String {
        &self.body
    }
}

pub struct Mutation;

#[Object]
impl Mutation {

    async fn create_post<'a>(
        &self,
        post_title: String,
        post_body: String,
        post_published: bool) -> Vec<crate::graphql::Post> {
        // use crate::models::Post;
        use crate::schema::posts::dsl::*;
        // create db connection
        let connection:SqliteConnection = establish_connection();
        // create a randomly generated string as the primary key
        let uuid = Uuid::new_v4().to_hyphenated().to_string();
        // create new post structure
        let new_post = crate::models::Post {
            id: uuid,
            title: post_title,
            body: post_body,
            published: post_published
        };
        // execute database insertion
        diesel::insert_into(posts)
            .values(&new_post)
            .execute(&connection)
            .unwrap();
        // make the query and return the results
        let results = posts
            .load::<crate::models::Post>(&connection)
            .expect("Error loading new post");
        // convert the model struct to the struct used locally
        // TODO: find a better way to do this without having to map from one struct to another
        // TODO: split this off into it's own function
        results.into_iter().map(|x| crate::graphql::Post{
            id: x.id,
            title: x.title,
            body: x.body,
            published: x.published
        }).rev().collect()
    }
}