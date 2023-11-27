use reqwest;

pub struct WebCrawler;

impl WebCrawler {
    pub fn new() -> Self {
        WebCrawler
    }

    pub async fn crawl(&self, url: &str) -> Result<String, reqwest::Error> {
        let response = reqwest::get(url).await?;
        Ok(response.text().await?)
    }
}
