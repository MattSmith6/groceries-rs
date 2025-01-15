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
    Number of servings: {:?}\n\
    \n\
    {:?} calories\n\
    {:?} carbs\n\
    {:?} fats\n\
    {:?} protein\n\
    \n\
    Ingredients: {:#?}", name, url, recipe_yield, servings, calories, carbs, fats, protein, ingredients)]
pub struct ScrapedRecipe {

    pub name: Option<String>,

    pub url: String,

    pub recipe_yield: Option<String>,

    pub servings: Option<String>,

    pub calories: Option<String>,

    pub carbs: Option<String>,

    pub fats: Option<String>,

    pub protein: Option<String>,

    pub ingredients: Option<Vec<String>>,

}

pub struct Recipe {

    pub name: String,

    pub source: Option<String>,

    pub recipe_yield: String,

    pub servings: u8,

    pub calories: Option<u16>,

    pub carbs: Option<f32>,

    pub fats: Option<f32>,

    pub protein: Option<f32>,

    pub ingredients: Vec<Ingredient>,

}