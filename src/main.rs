use iced::{Color, Element, Task};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use crate::err::ScrapeError;
use crate::recipes::ScrapedRecipe;
use crate::scrapers::recipe_scraper::scrape_recipe;

mod err;
mod gui;
mod scrapers;
mod groceries;
mod recipes;

fn main() -> iced::Result {
    iced::application("Recipes to Groceries", update, view)
        .theme(|_| iced::Theme::Dark)
        .centered()
        .run()
}

#[derive(Default)]
struct State {

    recipe_url: String,
    currently_scraping: bool,
    scraped_recipe: Option<ScrapedRecipe>,
    scrape_error: Option<ScrapeError>,

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
        Message::ScrapeSuccess(scraped_recipe) => {
            state.scraped_recipe = Some(scraped_recipe);
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

    let scrape_recipe_row = row![
        text("Scrape a website:").center(),
        Space::with_width(10),
        text_input("Enter recipe URL", &state.recipe_url)
            .on_input_maybe(is_not_scraping.then(|| {
                |string| Message::RecipeUrlChanged(string)
            }))
            .on_submit_maybe(can_scrape.then(|| Message::StartScrape)),
        Space::with_width(10),
        button("Scrape Recipe").on_press_maybe(can_scrape.then(|| Message::StartScrape)),
    ];

    let scrape_error = text(state.scrape_error.as_ref()
        .map_or(
            String::new(),
            |error| error.to_string())
        )
        .color(Color::from_rgba(0.95, 0.05, 0.05, 0.9))
        .center();

    let recipe_text = text(state.scraped_recipe.as_ref()
        .map_or(String::new(), |recipe| recipe.to_string()));

    scrollable(
        column![
            Space::with_height(10),
            scrape_recipe_row,
            Space::with_height(20),
            container(scrape_error),
            container(recipe_text),
        ].padding(20)
    ).into()
}

