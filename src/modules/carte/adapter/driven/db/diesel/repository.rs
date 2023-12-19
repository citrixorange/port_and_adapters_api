use std::future::Future;
use std::pin::Pin;

use diesel::prelude::*;
use diesel::insert_into;

use crate::modules::carte::adapter::driven::db::diesel::{
    entity::{CarteItem, NewCarteItem},
    schema::carte
};

use crate::modules::carte::core::{
    applications::ports::interface::ICarte,
    core::domain::{
        item::{
            Category,
            IItem
        },
        error::RepositoryError
    }
};


struct CarteRepository {
    database_connection: Option<PgConnection>
}

impl CarteRepository {
    pub fn new() -> Self {
        Self {
            database_connection: None
        }
    }

    pub fn connect(&mut self) -> Result<(), RepositoryError> {
        let database_url = dotenv::var("DATABASE_URL").map_err(|_| RepositoryError::ConnectionError);
        let mut connection = PgConnection::establish(&database_url).map_err(|_| RepositoryError::ConnectionError);
        self.database_connection = Some(connection);
        return Ok(());
    }
}

impl ICarte for CarteRepository {

    fn create_item(name: String, description: String, category: Category, price: String) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + 'static>> {

        let future = async {
            let new_item = NewCarteItem {
                name: name,
                description: description,
                category: category.to_string(),
                price: price
            };
    
            let inserted_item: CarteItem = insert_into(carte::table)
                .values(&new_item)
                .returning(CarteItem::as_returning())
                .get_result(&mut connection)
                .map_err(|_| RepositoryError::InsertionError)
    
            println!("Inserted item: {:?}", inserted_item);
            return Ok(());
        }

        return Box::pin(future);
    }

    fn edit_item(name: String, new_name: Option<String>, description: Option<String>, category: Option<Category>, price: Option<String>) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + 'static>> {
        let future = async {
            return Ok(());
        }

        return Box:pin(future);
    }

    fn remove_item(name:String) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + 'static>> {
        let future = async {
            return Ok(());
        }

        return Box:pin(future);
    }

    fn list_itens(category: Option<Category>) -> Pin<Box<dyn Future<Output = Result<Box<dyn IItem>, RepositoryError>> + 'static>> {
        let future = async {
            return Err(RepositoryError::ConnectionError);
        }

        return Box:pin(future);
    }
}