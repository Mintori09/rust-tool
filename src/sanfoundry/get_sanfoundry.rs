use super::sanfoundry;
use scraper::{Html, Selector};

#[allow(dead_code)]
pub fn get_url(url: &str) -> anyhow::Result<Vec<String>> {
    let content = sanfoundry::fetch_data(url)?;

    let html = Html::parse_document(&content);
    let section_selector = Selector::parse("div.sf-section").unwrap();
    let link_selector = Selector::parse("a[href]").unwrap();

    let mut hrefs = Vec::new();

    for section in html.select(&section_selector) {
        for link in section.select(&link_selector) {
            if let Some(href) = link.value().attr("href") {
                hrefs.push(href.to_string());
            }
        }
    }

    for href in &hrefs {
        println!("\"{}\",", href);
    }

    Ok(hrefs)
}
