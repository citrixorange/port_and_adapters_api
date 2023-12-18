use crate::modules::carte::core::domain::item::Category;

pub trait ICarte {
    fn create_item(name: String, description: String, category: Category, price: String);
    fn edit_item(name: String, new_name: Option<String>, description: Option<String>, category: Option<Category>, price: Option<String>);
    fn remove_item(name:String);
    fn list_itens(category: Option<Category>);
}