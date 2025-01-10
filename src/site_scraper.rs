pub fn scrape_url(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .gzip(true)
        .build()?;

    let request = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:124.0) Gecko/20100101 Firefox/124.0")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp")
        .header("Referer", "https://google.com")
        .header("Accept-Encoding", "gzip")
        .header("Connection", "keep-alive")
        .header("Cache-Control", "no-cache")
        .send()?;

    Ok(request.text().unwrap())
}