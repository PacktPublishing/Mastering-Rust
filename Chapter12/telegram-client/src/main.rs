extern crate hyper;
extern crate hyper_native_tls;

use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;

const API_URL: &'static str = "https://api.telegram.org";

fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let token = "276934321:AAG_4BHalBCTSIA4Z-3Auwv7MmoYC0rIK8k";

    let post_url = format!("{}/bot{}/{}", API_URL, token, "getMe");
    let request = client.post(&post_url);
    println!("PU: {}", post_url);
    let mut response = request.send().unwrap();

    let mut response_string = String::new();
    let _ = response.read_to_string(&mut response_string);

    println!("Response status: {}", response.status);
    println!("Response headers: {}", response.headers);
    println!("Response body: {}", response_string);
}
