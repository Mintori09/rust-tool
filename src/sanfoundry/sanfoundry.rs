use headless_chrome::{Browser, LaunchOptionsBuilder};
use regex::Regex;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn get_title(url: &str) -> String {
    let re = Regex::new(r"https://www\.sanfoundry\.com/([^/]+)/").unwrap();
    if let Some(caps) = re.captures(url) {
        if let Some(m) = caps.get(1) {
            let mut res = m.as_str().to_string();
            res.push_str(".md");
            return res;
        }
    }
    String::new()
}

fn clean_text(text: &str) -> String {
    let text = text.replace('\t', "");
    let text = text.replace("  ", " ");

    let re_multinewline = Regex::new(r"\n{2,}").unwrap();

    let text = re_multinewline.replace_all(&text, "\n");

    text.trim().to_string()
}

fn get_content(content: &str) -> String {
    let html = Html::parse_fragment(content);
    let html = remove_from_sf_section(html);

    let selector = Selector::parse("div.entry-content")
        .map_err(|e| format!("Selector parse error: {:?}", e))
        .unwrap();

    let text = html
        .select(&selector)
        .flat_map(|el| el.text())
        .collect::<Vec<_>>()
        .join(" ");

    clean_text(&text)
}

fn save_to_path(path: &Path, content: &str) -> anyhow::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn remove_from_sf_section(html: Html) -> Html {
    let selector = Selector::parse("div.sf-section").unwrap();

    if let Some(element) = html.select(&selector).next() {
        let html_str = html.root_element().html();
        let snippet = element.html();

        if let Some(start_index) = html_str.find(&snippet) {
            let truncated_html = &html_str[..start_index];
            return Html::parse_fragment(truncated_html);
        }
    }

    html
}

pub fn fetch_data(url: &str) -> anyhow::Result<String> {
    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .args(vec![
            "--disable-gpu".as_ref(),
            "--no-sandbox".as_ref(),
            "--disable-dev-shm-usage".as_ref(),
        ])
        .build()?;

    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    tab.set_user_agent(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
        AppleWebKit/537.36 (KHTML, like Gecko) \
        Chrome/113.0.0.0 Safari/537.36",
        Some("en-US,en;q=0.9"),
        Some("Windows"),
    )?;

    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;

    let content = tab.get_content().unwrap_or_default();
    Ok(content)
}

#[allow(dead_code)]
pub fn scrap(url: &str, mut path: PathBuf) -> anyhow::Result<()> {
    path.push(get_title(url));

    let text = get_content(&fetch_data(url).unwrap_or_default());

    match save_to_path(&path, &text) {
        Ok(()) => {
            println!("Saved {path:?}");
            Ok(())
        }
        Err(e) => {
            println!("{:?}", e);
            Err(e)
        }
    }
}
