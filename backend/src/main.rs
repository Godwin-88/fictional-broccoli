#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Financial Econometrics and Derivatives Pricing Platform"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

