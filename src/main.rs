extern crate rocket;
use crate::modules::app::run_app;

mod modules;

#[rocket::main]
async fn main() {
    run_app().await;
}
