pub mod recipe_scraper;

use derive_more::Display;
use crate::ingredient::Ingredient;

#[derive(Debug, Display)]
#[display("{:?}\n{:?} servings\n{:?} calories\n{:?} carbs\n{:?} fats\n{:?} protein\nIngredients: {:#?}", name, servings, calories, carbs, fats, protein, ingredients)]
pub struct ScrapedRecipe {

    pub name: Option<String>,

    pub servings: Option<String>,

    pub calories: Option<String>,

    pub carbs: Option<String>,

    pub fats: Option<String>,

    pub protein: Option<String>,

    pub ingredients: Option<Vec<String>>,

}

pub struct Recipe {

    pub name: String,

    pub servings: u8,

    pub calories: u16,

    pub carbs: u8,

    pub fats: u8,

    pub protein: u8,

    pub ingredients: Vec<Ingredient>,

}