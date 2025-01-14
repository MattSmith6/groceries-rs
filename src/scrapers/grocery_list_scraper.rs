use crate::recipes::Ingredient;
use crate::groceries::GroceryItem;

pub trait GroceryListScraper {

    fn scrape_shopping_list(&self, ingredients: Vec<Ingredient>) -> Vec<GroceryItem> {
        let mut shopping_list: Vec<GroceryItem> = Vec::new();
        for ingredient in ingredients {
            shopping_list.push(self.scrape_shopping_item(ingredient).unwrap())
        }

        shopping_list
    }

    fn scrape_shopping_item(&self, ingredient: Ingredient) -> Option<GroceryItem> {
        let search_queries = self.transform_search_queries(ingredient);

        for search_query in search_queries {
            let scraped_item = self.scrape_item_by_query(search_query);

            if scraped_item.is_some() {
                return scraped_item
            }
        }

        None
    }

    fn scrape_item_by_query(&self, query: String) -> Option<GroceryItem>;

    fn transform_search_queries(&self, ingredient: Ingredient) -> Vec<String>;

}