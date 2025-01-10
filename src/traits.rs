use crate::ingredient::Ingredient;
use crate::shoppingitem::ShoppingItem;

pub trait ShoppingListCreator {

    fn scrape_shopping_list(&self, ingredients: Vec<Ingredient>) -> Vec<ShoppingItem> {
        let mut shopping_list: Vec<ShoppingItem> = Vec::new();
        for ingredient in ingredients {
            shopping_list.push(self.scrape_shopping_item(ingredient).unwrap())
        }

        shopping_list
    }

    fn scrape_shopping_item(&self, ingredient: Ingredient) -> Option<ShoppingItem> {
        let search_queries = self.transform_search_queries(ingredient);

        for search_query in search_queries {
            let scraped_item = self.scrape_item_by_query(search_query);

            if scraped_item.is_some() {
                return scraped_item
            }
        }

        None
    }

    fn scrape_item_by_query(&self, query: String) -> Option<ShoppingItem>;

    fn transform_search_queries(&self, ingredient: Ingredient) -> Vec<String>;

}