extern crate cgi;
extern crate diesel;
use rusty_api::graphql::Query;

// async fn test() {
//     use async_graphql::*;
//     let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
//     let res = schema.execute("{ add(a: 10, b: 20) }").await;
//     let json = serde_json::to_string(&res).unwrap();
//     println!("{}", json)
// }

async fn get_post(post_id: String) -> String {
    use async_graphql::*;
    let query: String = format!(r#"{{ getPost(postId: "{}"){{id title}} }}"#, post_id);
    let schema: Schema<Query, EmptyMutation, EmptySubscription> = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res: Response = schema.execute(query).await;
    serde_json::to_string(&res).unwrap()
}

#[tokio::main]
async fn main() {

    let query_response = get_post("659cc6d7-b74d-4fff-bfae-c06aedb905f1".to_string()).await;
    let chunked_response = query_response.as_bytes().to_owned();

    cgi::handle(|request: cgi::Request| -> cgi::Response {
        cgi::binary_response(200, "application/json", chunked_response)
    })
}
