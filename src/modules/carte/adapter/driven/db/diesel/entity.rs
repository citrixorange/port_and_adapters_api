use diesel::prelude::*;

use crate::modules::carte::core::domain::item::{
    Category, 
    IItem
};

use crate::modules::carte::adapter::driven::db::diesel::schema::carte;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = carte)]
pub struct CarteItem {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub category: String,
    pub price: String
}

impl IItem for CarteItem {
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_description(&mut self, description: String) {
        self.description = description;
    }

    fn set_category(&mut self, category: Category) {
        self.category = category.to_string();
    }
    
    fn set_price(&mut self, price: String) {
        self.price = price;
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }

    fn get_description(&self) -> String {
        return self.description.clone();
    }

    fn get_category(&self) -> Category {
        return self.category.parse::<Category>().unwrap();
    }

    fn get_price(&self) -> String {
        return self.price.clone();
    }
}

#[derive(Insertable)]
#[diesel(table_name = carte)]
pub struct NewCarteItem {
    pub name: String,
    pub description: String,
    pub category: String,
    pub price: String
}
