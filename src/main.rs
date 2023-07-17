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

// I want to mount portfolio at /; maybe use a submodule?
// ch-iii also needs to go somewhere... presumably /ch-iii
// The current handling is hot garbage. I could refactor the repo by hand?
// What about everything else?
// Also: I want a TUI version. No compromises on that one.
// Maybe make M-itscript, finally? That would be impressive...