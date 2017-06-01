extern crate hyper;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::thread;
use std::sync::Arc;
use hyper::client::Client;
use hyper::header::UserAgent;
use std::io::Read;
use serde_json::Value;

const API_URL: &'static str = "https://api.telegram.org";
const POLL_INTERVAL: u64 = 60;

#[derive(Deserialize, Debug)]
struct TGUpdate {
    result: Vec<TGResult>
}

#[derive(Deserialize, Debug, Clone)]
struct TGResult {
    message: TGMessage,
    update_id: u64
}

#[derive(Deserialize, Debug, Clone)]
struct TGMessage {
    from: TGFrom,
    message_id: u64,
    text: Option<String>,

}

#[derive(Deserialize, Debug, Clone)]
struct TGFrom {
    first_name: String,
    id: u64,
    username: String
}

fn get_github_image_url(client: &Client, text: &str) -> String {
    let url = format!("https://api.github.com/users/{}", text);
    let request = client
        .get(&url)
        .header(UserAgent("JeevesMasteringRustBot-0.1".into()));
    let mut response = request.send().unwrap();

    let mut response_string = String::new();
    let _ = response.read_to_string(&mut response_string);
    let data: Value = serde_json::from_str(&response_string).unwrap();

    let github_url = data
        .as_object().unwrap().get("avatar_url").unwrap();
    github_url.as_str().unwrap().into()
}

fn reply_with_image(client: &Client, token: &str, id: u64, image_url: &str) {
    println!("Replying to {}", id);
    let post_url = format!("{}/bot{}/{}?chat_id={}&photo={}",
                           API_URL,
                           token,
                           "sendPhoto",
                           id,
                           image_url
    );

    let request = client.post(&post_url);
    let _ = request.send();
}

fn find_largest_offset(update: &TGUpdate) -> u64 {
    let mut i = 0;
    for r in &update.result {
        if r.update_id > i {
            i = r.update_id
        }
    }
    i
}

fn get_updates(client: &Client, token: &str, next_offset: u64) -> (TGUpdate, u64) {
    let post_url = format!("{}/bot{}/{}?offset={}&timeout={}",
                           API_URL,
                           token,
                           "getUpdates",
                           next_offset,
                           POLL_INTERVAL);
    let request = client.post(&post_url);
    let mut response = request.send().unwrap();

    let mut response_string = String::new();
    let _ = response.read_to_string(&mut response_string);
    let data: TGUpdate = serde_json::from_str(&response_string).unwrap();

    let last_seen_offset = find_largest_offset(&data);
    (data, last_seen_offset)
}

fn main() {
    let token = "276934321:AAG_4BHalBCTSIA4Z-3Auwv7MmoYC0rIK8k";
    let c = Arc::new(Client::new());
    let mut next_offset = 0u64;

    loop {
        let (updates, last_seen_offset) = get_updates(&c, token, next_offset);
        next_offset = last_seen_offset + 1;

        for result in updates.result {
            let c = c.clone();
            let github_user = result.message.text.clone().unwrap();

            thread::spawn(move || {
                let github_image_url = get_github_image_url(&c, &github_user);
                reply_with_image(
                    &c,
                    token,
                    result.message.from.id,
                    &github_image_url)
            });
        }
    }
}

