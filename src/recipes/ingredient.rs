use std::ops::Add;

pub struct Ingredient {

    // The quantity of this ingredient the recipes calls for
    pub quantity: u8,

    // The size of the ingredient: *heaping* (tablespoon), *large* (onion), *6 oz* (canned chiles)
    pub size: Option<String>,

    // The unit of the ingredient: *stick* (of butter), *bunch* (celery), *can* (green beans)
    pub unit: Option<String>,

    // The description of that type of ingredient: *yellow* onion, *ripe* banana, *no fat* yogurt
    // Usually, these are the things that could be omitted and replaced for substitutions
    pub descriptor: Option<String>,

    // The name of the ingredient: cream cheese, onion, canned tomatoes, italian sausage
    pub name: String,

    // How the ingredient is prepared *by the chef*: melted, chopped, diced, whole etc.
    pub preparation: Option<String>,

}

impl Ingredient {

    fn quote_string(optional: &Option<String>) -> Option<String> {
        let mut quoted_string = "\"".to_owned();

        optional.clone().map(|string| {
            quoted_string.push_str(string.as_str());
            quoted_string.push('"');
            quoted_string
        })
    }

    pub fn to_search_strings(&self) -> Vec<String> {
        let mut search_strings = Vec::new();
        let optional_search_parameters = vec![
            Self::quote_string(&self.unit),
            Self::quote_string(&self.descriptor),
            Self::quote_string(&self.size),
        ];

        for optional_search_parameter in optional_search_parameters {
            self.append_string(optional_search_parameter, &mut search_strings)
        }

        search_strings.reverse();
        search_strings
    }

    fn append_string(&self, new_append: Option<String>, string_vec: &mut Vec<String>) {
        if new_append.is_none() {
            return
        }

        let last_string = string_vec.last().unwrap().clone();
        let new_append = new_append.unwrap();

        let new_search_string = last_string.add(" AND ").add(new_append.as_str());
        string_vec.push(new_search_string);
    }

}