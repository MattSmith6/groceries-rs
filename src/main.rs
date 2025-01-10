mod ingredient;
mod walmart_scraper;
mod shoppinglist;
mod shoppingitem;
mod traits;
mod site_scraper;
mod recipe;
mod err;

fn main() {
    // https://www.allrecipes.com/recipe/202975/potstickers-chinese-dumplings/
    // https://www.skinnytaste.com/coconut-chicken-rice-bowl/#recipe
    // https://www.foodnetwork.com/recipes/giada-de-laurentiis/chicken-parmesan-recipe-1942593
    // https://asimplepalate.com/blog/chicken-parmigiana/

    let url = "https://asimplepalate.com/blog/chicken-parmigiana/";

    println!("{}", match recipe::recipe_scraper::scrape_recipe(url) {
        Ok(recipe) => recipe,
        Err(_) => return ()
    });
}

