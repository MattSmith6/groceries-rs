use crate::ingredient::Ingredient;

fn get_search_results(ingredient: &Ingredient) {
    let search_queries = ingredient.to_search_strings()
        .iter()
        .map(|s| s.replace(" ", "+"));
}
