use serde_json::{Map, Value};
use crate::site_scraper;
use crate::err::ScrapeError;
use crate::recipe::ScrapedRecipe;

pub fn scrape_recipe(website: &str) -> Result<ScrapedRecipe, ScrapeError> {
    let html_content = match site_scraper::scrape_url(website) {
        Ok(html) => html,
        Err(err) => return Err(ScrapeError::Reqwest(err))
    };
    let document = scraper::Html::parse_document(&html_content);

    let ld_json_script = r#"script[type="application/ld+json"]"#;
    let selector = match scraper::Selector::parse(ld_json_script) {
        Ok(selector) => selector,
        Err(err) => return Err(ScrapeError::Selector(err))
    };

    let ld_json_contents = document.select(&selector).next();
    if ld_json_contents.is_none() {
        return Err(ScrapeError::Generic("This site does not have a JSON LD compatible recipe."))
    }

    let ld_json_html = ld_json_contents.unwrap().inner_html();
    let ld_json_node = match serde_json::from_str(&ld_json_html) {
        Ok(ld_json_node) => ld_json_node,
        Err(err) => return Err(ScrapeError::Json(err))
    };

    dbg!("{}", match serde_json::to_string_pretty(&ld_json_node) {
        Ok(json) => json,
        Err(err) => return Err(ScrapeError::Json(err))
    });

    let recipe_node = find_recipe_node(&ld_json_node);

    if recipe_node.is_none_or(|node| !node.is_object()) {
        return Err(ScrapeError::Generic("Auto-scraping of recipes are not supported on this site."))
    }

    let recipe_node = recipe_node.unwrap().as_object().unwrap();
    let nutrition_object = find_nutrition_object(recipe_node);

    Ok(ScrapedRecipe {
        name: find_recipe_name(recipe_node),
        servings: find_nutrition_field(nutrition_object, "servingSize"),
        calories: find_nutrition_field(nutrition_object, "calories"),
        carbs: find_nutrition_field(nutrition_object, "carbohydrateContent"),
        fats: find_nutrition_field(nutrition_object, "fatContent"),
        protein: find_nutrition_field(nutrition_object, "proteinContent"),
        ingredients: find_recipe_ingredients(recipe_node),
    })
}

fn find_recipe_name(recipe_node: &Map<String, Value>) -> Option<String> {
    recipe_node.get("name")
        .map(|val| val.as_str())
        .flatten()
        .map(|name| name.to_string())
}

fn find_nutrition_object(recipe_node: &Map<String, Value>) -> Option<&Map<String, Value>> {
    let nutrition_object = recipe_node.get("nutrition");

    if nutrition_object.is_none_or(|node| !node.is_object()) {
        return None
    }

    nutrition_object.unwrap().as_object()
}

fn find_nutrition_field(nutrition_object: Option<&Map<String, Value>>, key: &str)
    -> Option<String> {
    let key = key.to_string();
    nutrition_object?.get(&key)
        .map(|val| val.as_str())
        .flatten()
        .map(|s| s.to_string())
}

fn find_recipe_ingredients(recipe_node: &Map<String, Value>) -> Option<Vec<String>> {
    let recipe_ingredients = recipe_node.get("recipeIngredient");

    if recipe_ingredients.is_none_or(|node| !node.is_array()) {
        return None
    }

    let recipe_ingredients = recipe_ingredients.unwrap().as_array().unwrap();
    let mut ingredient_vec = Vec::new();

    for ingredient in recipe_ingredients {
        ingredient_vec.push(ingredient.as_str().unwrap().to_string());
    }

    Some(ingredient_vec)
}

fn find_recipe_node(node: &Value) -> Option<&Value> {
    if is_recipe_node(&node) {
        return Some(node)
    }

    match node {
        Value::Array(arr) => arr.iter()
            .map(|node| find_recipe_node(node))
            .find(|node| node.is_some())
            .flatten(),
        Value::Object(obj) => obj.iter()
            .map(|(_, val)| find_recipe_node(val))
            .find(|node| node.is_some())
            .flatten(),
        _ => None
    }
}

fn is_recipe_node(node: &Value) -> bool {
    match node {
        Value::Object(obj) => {
            let optional_type = obj.get("@type");
            if optional_type.is_none_or(|type_value| type_value.is_null()) {
                return false
            }

            let node_type = optional_type.unwrap();
            let expected_type = "Recipe";

            if node_type.as_str() == Some(expected_type) {
                return true
            }

            let expected_array_value = &Value::String(expected_type.to_string());
            node_type.as_array()
                .is_some_and(|array| array.contains(expected_array_value))
        }
        _ => false,
    }
}
