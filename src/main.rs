mod data;
mod version;
pub mod error;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    let routes = routes![
        data::delete
    ];

    rocket::build()
        .mount("/api/data/", routes)
}