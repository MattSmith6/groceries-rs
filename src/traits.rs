use crate::ingredient::Ingredient;
use crate::shoppingitem::ShoppingItem;

trait ShoppingListCreator {

    fn new(ingredients: Vec<Ingredient>) -> Self;

    fn create_shopping_list(&self) -> Vec<ShoppingItem>;

    fn transform_search_queries(&self, ingredient: Ingredient) -> Vec<String>;

}