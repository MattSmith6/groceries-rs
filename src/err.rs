#[derive(Debug)]
pub enum ScrapeError<'a> {

    Selector(scraper::error::SelectorErrorKind<'a>),

    Reqwest(reqwest::Error),

    Json(serde_json::Error),

    Generic(&'static str),

}

