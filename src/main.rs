use diesel::prelude::*;
use diesel::insert_into;

use crate::modules::carte::adapter::driven::db::diesel::{entity::{CarteItem, NewCarteItem}, schema::carte};
use crate::modules::carte::core::domain::item::Category;

mod modules;

fn main() {

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Establish a connection to the database
    let mut connection = PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // Insert a new user into the users table
    let new_item = NewCarteItem {
        name: "hamburguer".to_string(),
        description: "Joy burguer".to_string(),
        category: Category::MainCourse.to_string(),
        price: "R$15.00".to_string()
    };

    // Use Diesel's insert_into to insert data
    // The `returning` function is used to get the inserted row back
    let inserted_item: CarteItem = insert_into(carte::table)
        .values(&new_item)
        .returning(CarteItem::as_returning())
        .get_result(&mut connection)
        .expect("Error inserting carte item");

    println!("Inserted item: {:?}", inserted_item);
}
