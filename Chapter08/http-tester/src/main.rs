extern crate hyper;

use hyper::client::Client;
use hyper::status::StatusCode;

macro_rules! http_test {
    ($url:tt GET => $code:expr) => {
        let client = Client::new();
        let res = client.get($url).send().unwrap();
        println!("GET {} => {}", $url, $code);
        assert_eq!(res.status, $code);
    };
    ($url:tt POST => $code:expr) => {
        let client = Client::new();
        let res = client.post($url).send().unwrap();
        println!("POST {} => {}", $url, $code);
        assert_eq!(res.status, $code);
    };
}

fn main() {
    println!("Hello, world!");
    http_test!("http://google.com" GET => StatusCode::Ok);
    http_test!("http://google.com" POST => StatusCode::MethodNotAllowed);
    http_test!("http://google.com" POST => StatusCode::Ok);
}
