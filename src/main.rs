#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// #[get("/world")]
// fn world() -> &'static str {
//     "Hello, world!"
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}