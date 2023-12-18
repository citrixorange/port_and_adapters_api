use crate::carte::core::applications::ports::interface::ICarte;

pub struct Carte<'a> {
    carte: &'a mut dyn ICarte;
}

impl <'a>Carte<'a> {
    pub fn new(carte: &'a mut dyn ICarte) -> Self {
        Self { carte }
    }

    pub fn create_item(&self, name: String, description: String, category: Category, price: String) {
        return self.carte.create_item(name, description, category, price);
    }

    pub fn edit_item(&self, name: String, new_name: Option<String>, description: Option<String>, category: Option<Category>, price: Option<String>) {
        return self.carte.edit_item(name, new_name, description, category, price);
    }

    pub fn remove_item(&self, name:String) {
        return self.carte.remove_item(name);
    }

    pub fn list_itens(&self, category: Option<Category>) {
        return self.list_itens(category);
    }
}