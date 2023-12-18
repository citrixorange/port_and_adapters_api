use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Category {
    MainCourse,
    Accompaniment,
    Drink,
    Dessert
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::MainCourse => write!(f,"Category::MainCourse"),
            Category::Accompaniment => write!(f,"Category::Accompaniment"),
            Category::Drink => write!(f,"Category::Drink"),
            Category::Dessert => write!(f,"Category::Dessert"),
        }
    }
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Category::MainCourse" => Ok(Category::MainCourse),
            "Category::Accompaniment" => Ok(Category::Accompaniment),
            "Category::Drink" => Ok(Category::Drink),
            "Category::Dessert" => Ok(Category::Dessert),
            _ => Err(()),
        }
    }
}

