use std::collections::HashMap;
use reqwest::{Client, Error, Response};
use reqwest::Url;
use reqwest::header::HeaderMap;
use app_struct::User;
use app_struct::App;
use crate::app_struct;


fn init_client() -> Client {
    let user_agent = "toot-rs/0.0.1";
    let client = Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap();
    return client;
}

pub async fn get(app: App, user: User, path: &str, headers: HeaderMap, params: &[(&'static str, &'static str); 2]) {
    let client = init_client();
    let _url = format!("{}/{}", app.base_url, path);
    let url = Url::parse_with_params(&_url, params);
    let resp = client.get(url.unwrap())
        .headers(headers)
        .bearer_auth(user.access_token)
        .send();
    let response = resp.await.unwrap();
    if response.status().is_success() {
        return response.json().await.unwrap();
    } else {
        println!("Error: {}", response.status());
    }
}