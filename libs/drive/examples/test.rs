use std::iter::Map;
use reqwest::Url;

fn main() {
    let token = drive::models::auth::Token::from(String::from("sb"));
    let json = serde_json::to_string_pretty(&token);
    println!("{:?}", json);
}
