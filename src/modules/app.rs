use rocket_codegen::routes;

use crate::modules::carte::adapter::driver::api::controller::{
    index
};

pub async fn run_app() {
    let _ = rocket::build().mount("/", routes![
        index
    ]).launch().await;
}