use crate::modules::carte::core::domain::{
    item::{
        Category,
        IItem
    },
    errors::RepositoryError
};
use std::future::Future;
use std::pin::Pin;

pub trait ICarte {
    fn create_item(name: String, description: String, category: Category, price: String) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + 'static>>;
    fn edit_item(name: String, new_name: Option<String>, description: Option<String>, category: Option<Category>, price: Option<String>) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + 'static>>;
    fn remove_item(name:String) -> Pin<Box<dyn Future<Output = Result<(), RepositoryError>> + 'static>>;
    fn list_itens(category: Option<Category>) -> Pin<Box<dyn Future<Output = Result<Box<dyn IItem>, RepositoryError>> + 'static>>;
}