extern crate diesel;
use rusty_api::graphql::Query;
use std::io::{Write, Read, stdin};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    [pub] static ref CONFIG: _ = crate::config::get_configuration().unwrap();
    [pub] static ref API_KEY: String = CONFIG.api_key;
    [pub] static ref DB_URL: String = CONFIG.database_url;
}
const API_KEY: String = "Foo".to_string();

// // get config parameters
// static CONFIG: _ = crate::config::get_configuration().unwrap();
// // API KEY
// pub const API_KEY: &'static str = CONFIG.api_key;
// // DB File Name
// pub const DB_URL: &'static str = CONFIG.database_url;

async fn get_post(query: String) -> Vec<u8> {
    use async_graphql::*;
    let string_query: String = format!(r#"{}"#, query);
    let schema: Schema<Query, EmptyMutation, EmptySubscription> = Schema::new(
        Query,
        EmptyMutation,
        EmptySubscription
    );
    // execute the query
    let response: Response = schema.execute(string_query).await;
    // serialize response
    serde_json::to_vec(&response).unwrap()
}

#[tokio::main]
async fn main() {




    // get the content length; prevents the insertion of any additional commands
    let content_length= envmnt::get_usize("CONTENT_LENGTH", 0);

    // read the body (POST sends payload in body)
    let mut stdin_contents = vec![0; content_length];

    // declare string that can be updated with the following if/else statement
    let mut graphql_query = String::new();

    // if stdin_contents length is 0, run a test
    if content_length != 0 {
        // read in the stdin
        stdin().read_exact(&mut stdin_contents).unwrap();
        // get the graphql query from the body
        graphql_query = String::from_utf8(stdin_contents).unwrap();
    } else {
        // pass in a test query
        graphql_query = r#"{ allPosts{id title body published} }"#.to_string();
    }

    // send the query to the graphql abstraction layer
    // let query_response = get_post("659cc6d7-b74d-4fff-bfae-c06aedb905f1".to_string()).await;
    let query_response = get_post(graphql_query).await;

    // send back a response; CGI relies on stdout, so just going to form our own http payload
    // to keep things as simple as possible. The extra line break is necessary to indicate body
    println!("Content-Type: application/json");
    println!("");
    std::io::stdout().write(query_response.as_slice());

}