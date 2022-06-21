mod staticfiles;

use reqwest::{Client, header::HeaderMap};
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = Client::new();
    staticfiles::dumb_file_tests(&client).await;
    do_initial_setup(&client).await;
    setup_panel_dumb_test(&client).await;
    let token = get_token(&client).await;
    link_actions(&token).await;
}

async fn do_initial_setup(client: &Client) {
    client
        .post("http://localhost:8080/simpleshortener/api/create")
        .json(&json!({"username": "username", "password": "password"}))
        .send()
        .await
        .expect("Failed to setup panel!");
}

async fn setup_panel_dumb_test(client: &Client) {
    println!("Testing post-setup panel...");
    let panel = client
        .get("http://localhost:8080/simpleshortener")
        .send()
        .await
        .expect("Failed to get panel html");
    assert_eq!(
        panel.text().await.unwrap(),
        include_str!("../../src/resources/panel.html")
    );
    println!("Panel check succeeded!");
}

#[derive(serde::Deserialize)]
struct NewToken {
    token: String,
}

async fn get_token(client: &Client) -> String {
    println!("Getting token...");
    let token = client
        .post("http://localhost:8080/simpleshortener/api/token")
        .json(&json!({"username": "username", "password": "password"}))
        .send()
        .await
        .expect("Failed to get token")
        .json::<NewToken>()
        .await
        .unwrap()
        .token;
    println!("Got token {}!", token);
    token
}

async fn link_actions(token: &str) {
    let headers = HeaderMap::new();
    headers.append("Authorization", HeaderValue::from("Bearer".to_owned() + token));
    let client = Client::builder().default_headers(headers).build().unwrap();
    println!("Testing link addition")
    client.post("http://localhost:8080/simpleshortener/api/add")
}
