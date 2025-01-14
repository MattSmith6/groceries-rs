use scraper::{Html, Selector};
use crate::recipes::Ingredient;
use crate::groceries::GroceryItem;
use crate::scrapers::grocery_list_scraper::GroceryListScraper;
use crate::scrapers::site_scraper;

pub struct WalmartScraper;

impl WalmartScraper {

    fn scrape_aisle_by_product_page(product_page_url: String) -> Option<String> {
        let html = match site_scraper::scrape_url(&product_page_url) {
            Ok(html) => html,
            Err(_) => return None,
        };

        println!("{}", html);
        let document = Html::parse_document(&html);

        let aisle_selector_tag = r#"div[data-testid="product-aisle-location]"#;
        let aisle_selector = Selector::parse(aisle_selector_tag).unwrap();

        if let Some(aisle_div) = document.select(&aisle_selector).next() {
            println!("{}", &aisle_div.inner_html());

            let aisle_span = Selector::parse("span").unwrap();
            let aisle_html = Html::parse_fragment(&aisle_div.inner_html());

            return aisle_html.select(&aisle_span).next().map(|aisle| aisle.inner_html())
        }

        None
        // Set the store location in the cookies for aisle information to be present.
    }

}

impl GroceryListScraper for WalmartScraper {

    fn scrape_item_by_query(&self, query: String) -> Option<GroceryItem> {
        let url = format!("https://www.walmart.com/search?q={}", query);
        let html = match site_scraper::scrape_url(&url) {
            Ok(html) => html,
            Err(_) => return None,
        };

        println!("{}", html);
        let document = Html::parse_document(&html);

        let product_suggestions_tag = r#"div[data-stack-index="0"]"#;
        let product_suggestions_selector = Selector::parse(product_suggestions_tag).unwrap();

        let product_container = document.select(&product_suggestions_selector).next();
        if product_container.is_none() {
            println!("Data-stack-index 0 does not exist (no products match this search query)");
            return None
        }

        let product_container_html = product_container.unwrap().inner_html();
        println!("{}", product_container_html);

        let product_container = Html::parse_fragment(&product_container_html);

        let item_stack_tag = r#"div[data-testid="item-stack"]"#;
        let item_stack_selector = Selector::parse(item_stack_tag).unwrap();

        let item_container = product_container.select(&item_stack_selector).next();
        if item_container.is_none() {
            println!("Item-stack does not exist");
            return None
        }

        let item_container_html = item_container.unwrap().inner_html();
        println!("{}", item_container_html);

        let item_container = Html::parse_fragment(&item_container_html);

        let div_selector = Selector::parse("div").unwrap();
        let mut item_selector = item_container.select(&div_selector);

        let product_link_selector = Selector::parse("a").unwrap();

        let product_price_tag = r#"div[data-automation-id="product-price"]"#;
        let product_price_selector = Selector::parse(product_price_tag).unwrap();

        let fulfillment_badge_tag = r#"div[data-automation-id="fulfillment-badge"]"#;
        let fulfillment_badge = Selector::parse(fulfillment_badge_tag).unwrap();

        while let Some(item) = item_selector.next() {
            println!("{}", item.inner_html());
            let item_html = Html::parse_document(&item.inner_html());

            // Check to see if this item is currently in stock, skip if it is not
            if let Some(fulfillment) = item_html.select(&fulfillment_badge).next() {
                todo!("Search fulfillment for Pickup today to see if the item is immediately available");

                continue;
            }

            todo!("Search the product price for the current price $<> tag using regex");

            let item_link_elem = item_html.select(&product_link_selector).next().unwrap();
            let item_link = item_link_elem.value().attr("href").unwrap();

            let aisle = Self::scrape_aisle_by_product_page(item_link.to_string());
        }

        None
    }


    fn transform_search_queries(&self, ingredient: Ingredient) -> Vec<String> {
        ingredient.to_search_strings()
            .iter()
            .map(|s| s.replace(" ", "+"))
            .collect()
    }

}

#[cfg(test)]
mod test {
    use crate::scrapers::walmart_scraper::WalmartScraper;

    #[test]
    fn scrapes_aisle_correctly() {
        assert_eq!(
            Some("Aisle A1".to_string()),
            WalmartScraper::scrape_aisle_by_product_page(
                "https://www.walmart.com/ip/Fresh-Celery-Hearts-Each/10402651".to_string()
            )
        )
    }

}
