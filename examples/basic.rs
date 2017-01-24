extern crate pbr;
extern crate dotenv;

use dotenv::dotenv;
use std::env;


fn get_pb_token(vars: env::Vars) -> Option<String> {
    for (key, value) in vars {
        if key == "TOKEN" {
            return String::from(value).into();
        }
    }
    None
}

pub fn main() {
    dotenv().ok();
    let token = get_pb_token(env::vars()).expect("No TOKEN env::var found");

    // create a client with your token
    let client = pbr::Client::new(&token);

    let me = client.whoami().unwrap();
    println!("name: {}, email: {}", me["name"], me["email"]);

    let pushes = client.pushes().modified_after(pbr::now_sec()).send().unwrap();
    println!("{}", pushes.pretty(2));

    let pushes = client.pushes().limit(5).send().unwrap();
    println!("{}", pushes.pretty(2));
}
