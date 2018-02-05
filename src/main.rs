extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct Greeting {
    message: String
}

fn main() {
    fn greeting(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting { message: "heeloooo".to_string() };
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    println!("Running on localhost:3000");
    Iron::new(greeting).http("0.0.0.0:3000").unwrap();
}
