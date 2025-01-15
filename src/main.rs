use std::sync::Arc;
use derive_more::Display;
use iced::{Alignment, Color, Element, Length, Task};
use iced::widget::{button, column, container, row, scrollable, text, text_editor, text_input, Space};
use iced::widget::text_editor::Edit;
use regex::Regex;
use crate::err::ScrapeError;
use crate::recipes::ScrapedRecipe;
use crate::scrapers::recipe_scraper::scrape_recipe;
use crate::util::NumParser;

mod err;
mod gui;
mod scrapers;
mod groceries;
mod recipes;
mod util;

fn main() -> iced::Result {
    iced::application("Recipes to Groceries", update, view)
        .theme(|_| iced::Theme::Dark)
        .centered()
        .run()
}

#[derive(Default, Display)]
#[display(
    "Recipe Name: {:?}\n\
    Source: {}\n\
    \n\
    Yields {:?}\n\
    Number of servings: {:?}\n\
    \n\
    {:?} calories\n\
    {:?} grams carbs\n\
    {:?} grams fats\n\
    {:?} grams protein\n\
    \n\
    Ingredients:\n\
    {}", recipe_name, recipe_source, recipe_yield, servings, calories, carbs, fats, protein, ingredients.text())]
struct State {

    num_parser: NumParser,

    recipe_url: String,
    currently_scraping: bool,
    scraped_recipe: Option<ScrapedRecipe>,
    scrape_error: Option<ScrapeError>,

    recipe_name: String,
    recipe_source: String,
    recipe_yield: String,
    servings: u8,
    calories: Option<u16>,
    carbs: Option<f32>,
    fats: Option<f32>,
    protein: Option<f32>,
    ingredients: text_editor::Content,

}

#[derive(Debug, Clone)]
enum Message {

    RecipeUrlChanged(String),
    StartScrape,
    EndScrape,
    ScrapeSuccess(ScrapedRecipe),
    ScrapeError(ScrapeError),

}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::RecipeUrlChanged(url) => {
            state.recipe_url = url.to_string();
            Task::none()
        }
        Message::StartScrape => {
            let recipe_url = state.recipe_url.clone();

            state.currently_scraping = true;
            state.scraped_recipe = None;
            state.scrape_error = None;

            Task::perform(
                async {
                    scrape_recipe(recipe_url)
                }, |scrape_result| match scrape_result {
                    Ok(scraped_recipe) => Message::ScrapeSuccess(scraped_recipe),
                    Err(scrape_error) => Message::ScrapeError(scrape_error)
                }
            ).chain(Task::done(Message::EndScrape))
        }
        Message::EndScrape => {
            state.recipe_url.clear();
            state.currently_scraping = false;

            Task::none()
        }
        Message::ScrapeSuccess(recipe) => {
            //state.scraped_recipe = Some(scraped_recipe);

            let parse_positive_integer = |input| state.num_parser.parse_positive_integer(input);
            let parse_positive_decimal = |input| state.num_parser.parse_positive_decimal(input);

            let as_u8 = |i| i as u8;
            let as_u16 = |i| i as u16;
            let as_f32 = |f| f as f32;

            fn parse<'a, T, R, F1, F2>(input: &'a Option<String>, parse_f: &F1, cast_f: &F2) -> Option<R>
            where F1: FnOnce(&'a str) -> Option<T> + Copy, F2: FnOnce(T) -> R + Copy {
                let parse_result = input.as_ref().map_or(None, |input| {
                    parse_f(input.as_str())
                });
                parse_result.map(|t| cast_f(t))
            }

            state.recipe_name = recipe.name.unwrap_or_default();
            state.recipe_source = recipe.url;
            state.recipe_yield = recipe.recipe_yield.unwrap_or_default();
            state.servings = parse(&recipe.servings, &parse_positive_integer, &as_u8).unwrap_or_default();
            state.calories = parse(&recipe.calories, &parse_positive_integer, &as_u16);
            state.carbs = parse(&recipe.carbs, &parse_positive_decimal, &as_f32);
            state.fats = parse(&recipe.fats, &parse_positive_decimal, &as_f32);
            state.protein = parse(&recipe.protein, &parse_positive_decimal, &as_f32);

            // Contents of the text editor are not modifiable - so replace the component entirely
            state.ingredients = text_editor::Content::new();

            let ingredients = recipe.ingredients.map(|vec| vec.join("\n")).unwrap_or_default();
            state.ingredients.perform(text_editor::Action::Edit(Edit::Paste(Arc::new(ingredients))));

            println!("{}", state);

            Task::none()
        }
        Message::ScrapeError(scraped_error) => {
            state.scrape_error = Some(scraped_error);
            Task::none()
        }
    }
}



fn view(state: &State) -> Element<'_, Message> {
    let is_not_scraping = !state.currently_scraping;
    let can_scrape = is_not_scraping && !state.recipe_url.is_empty();

    let scrape_recipe_row = row!(
        text("Scrape a website:").center(),
        text_input("Enter recipe URL", &state.recipe_url)
            .on_input_maybe(is_not_scraping.then(|| {
                |string| Message::RecipeUrlChanged(string)
            }))
            .on_submit_maybe(can_scrape.then(|| Message::StartScrape))
            .padding(5),
        button("Scrape Recipe").on_press_maybe(can_scrape.then(|| Message::StartScrape)),
    ).spacing(10).align_y(Alignment::Center);

    let scrape_error = text(state.scrape_error.as_ref()
        .map_or(
            String::new(),
            |error| error.to_string())
        )
        .color(Color::from_rgba(0.95, 0.05, 0.05, 0.9))
        .width(Length::Fill)
        .center();

    let recipe_text = text(state.to_string());

    scrollable(
        column!(
            scrape_recipe_row,
            container(scrape_error),
            container(recipe_text),
        ).padding(20).spacing(20)
    ).into()
}

