use diesel::prelude::*;

use crate::modules::carte::core::domain::item::Category;
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

#[derive(Insertable)]
#[diesel(table_name = carte)]
pub struct NewCarteItem {
    pub name: String,
    pub description: String,
    pub category: String,
    pub price: String
}
