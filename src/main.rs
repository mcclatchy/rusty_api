extern crate diesel;
use async_std::task;
use rusty_api::graphql::Query;


// async fn test() {
//     use async_graphql::*;
//     let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
//     let res = schema.execute("{ add(a: 10, b: 20) }").await;
//     let json = serde_json::to_string(&res).unwrap();
//     println!("{}", json)
// }

async fn get_post(post_id: String){
    use async_graphql::*;
    let query: String = format!(r#"{{ getPost(postId: "{}"){{id title}} }}"#, post_id);
    let schema: Schema<Query, EmptyMutation, EmptySubscription> = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res: Response = schema.execute(query).await;
    let json: String  = serde_json::to_string(&res).unwrap();
    println!("{}", json)
}


fn main() {

    // run the test
    // task::block_on(test());

    // search for a post by id
    task::block_on(get_post("659cc6d7-b74d-4fff-bfae-c06aedb905f1".to_string()));


    // // create new post
    // create_post("hello, world!".to_string(), "lorem ipsum".to_string());
    // // get the 5 most recent posts
    // let posts = get_post("659cc6d7-b74d-4fff-bfae-c06aedb905f1".to_string());
    //
    // // show the post results
    // println!("Displaying {} posts", posts.len());
    // for post in posts {
    //     println!("{}", post.title);
    //     println!("----------\n");
    //     println!("{}", post.body);
    // }

}
