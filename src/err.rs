use derive_more::Display;

#[derive(Debug, Clone, Display)]
pub enum ScrapeError {

    Selector(String),

    Reqwest(String),

    Json(String),

    Generic(&'static str),

}

