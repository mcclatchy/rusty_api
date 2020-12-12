extern crate cgi;
extern crate diesel;
use async_graphql::*;
use regex::Regex;
use rusty_api::graphql::{Query, Mutation};
use urldecode;

fn main() {

    // create an async runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    // regex query for parsing graphql query
    let re = Regex::new(r#"query=(.+[^&]})"#)
        .expect("Could not find a regex match");

    // crank up the cgi handler
    cgi::handle(|_request: cgi::Request| -> cgi::Response {

        // We have to use GET parameters since POST is unreliable / probably stripped by proxy
        let query_string = std::env::var("QUERY_STRING")
            .expect("Unable to locate QUERY_STRING");

        // url decode the query string
        let query_decoded = urldecode::decode(query_string);

        // use regex to parse the paramater `query=`
        let re_groups = re.captures(query_decoded.as_str())
            .expect("Unable to capture any groups");

        let graphql_query = re_groups.get(1)
            .map_or("", |m| m.as_str());

        // build the graphql schema handler
        let graphql_schema: Schema<Query, Mutation, EmptySubscription> = Schema::new(
            Query,
            Mutation,
            EmptySubscription
        );

        // execute the graphql query within an async/futures runtime
        let response: Response = rt.block_on(async {
            graphql_schema.execute(graphql_query).await
        });

        // serialize response
        let graphql_response = serde_json::to_vec(&response)
            .expect("failed to vectorize query response");

        // return the response, body should be Vec<u8, Global>
        cgi::binary_response(
            200,
            "application/json",
            graphql_response
        )

    })

}