use rocket::http::{ContentType, Status};
use rocket::local::Client;

use crate::create_app;
use crate::models::Id;
use crate::models::post::{NewPost, Post};

fn create_client() -> Client {
    let rocket = create_app();
    Client::new(rocket).unwrap()
}

fn create_post() -> (Id, NewPost) {
    let client = create_client();

    let post = NewPost { title: "Test".to_string(), body: "Body".to_string() };
    let post_serialized = serde_json::to_string(&post).unwrap();
    let req = client
        .post("/posts")
        .header(ContentType::JSON)
        .body(post_serialized);
    let mut response = req.dispatch();
    let id: Id = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    (id, post)
}

#[test]
fn test_create_post() {
    let client = create_client();

    let post = NewPost { title: "Test title".to_string(), body: "Test body".to_string() };
    let serialized = serde_json::to_string(&post).unwrap();
    let req = client
        .post("/posts")
        .header(ContentType::JSON)
        .body(serialized);
    let mut response = req.dispatch();

    assert_eq!(Status::Ok, response.status());

    assert_eq!(
        ContentType::JSON,
        response
            .content_type()
            .expect("Could not read content type header")
    );

    let _: Id = serde_json::from_str(
        &response.body_string().expect("Could not read the string body from response")
    ).expect("Could not parse Id object from body");
}

#[test]
fn test_get_post() {
    let (id, post) = create_post();
    let rocket = create_app();
    let client = Client::new(rocket).expect("Valid rocket instance");
    let req = client.get(format!("/posts/{}", id));
    let mut response = req.dispatch();

    assert_eq!(Status::Ok, response.status());
    assert_eq!(ContentType::JSON, response.content_type().unwrap());

    let data: Post = serde_json::from_str(&response.body_string().unwrap()).unwrap();
    assert_eq!(id, data.id);
    assert_eq!(post.body, data.body);
    assert_eq!(post.title, data.title);
}