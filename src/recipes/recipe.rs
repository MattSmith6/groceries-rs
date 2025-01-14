use std::rc::Rc;
use std::sync::Arc;
use derive_more::Display;
use crate::recipes::Ingredient;

#[derive(Debug, Display, Clone)]
#[display(
    "Recipe Name: {:?}\n\
    Source: {}\n\
    \n\
    Yields {:?}\n\
    Serving size: {:?}\n\
    \n\
    {:?} calories\n\
    {:?} carbs\n\
    {:?} fats\n\
    {:?} protein\n\
    \n\
    Ingredients: {:#?}", name, url, recipe_yield, serving, calories, carbs, fats, protein, ingredients)]
pub struct ScrapedRecipe {

    pub name: Option<String>,

    pub url: String,

    pub recipe_yield: Option<String>,

    pub serving: Option<String>,

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