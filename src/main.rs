use regex::Regex;
use scraper::{Html, Selector};
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::task::JoinSet;

const NTHREAD: usize = 10;
#[allow(dead_code)]

fn get_url_metruyenchu(url: &str, num: i32) -> String {
    let re = Regex::new(r"(https://metruyencv\.com/truyen/.*/chuong-)\d+").unwrap();
    re.replace(url, |caps: &regex::Captures| format!("{}{}", &caps[1], num))
        .into_owned()
}

async fn fetch_content_metruyenchu(url: String) -> Result<String, Box<dyn Error + Send + Sync>> {
    let resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&resp);
    let content_selector = Selector::parse("div.break-words").unwrap();
    let content = document
        .select(&content_selector)
        .next()
        .map(|node| node.inner_html())
        .unwrap_or_else(|| "Không tìm thấy nội dung!".to_string());

    let re_br = Regex::new(r"(?i)<br\s*/?><br\s*/?>").unwrap();
    let replaced_content = re_br.replace_all(&content, "\n");
    let re_div = Regex::new(r"(?s)<div.*?>.*?</div>").unwrap();
    let final_result = re_div.replace_all(&replaced_content, "");

    let re = Regex::new(r"(Chapter )(\d+)").unwrap();
    let first_line = final_result.lines().next().unwrap_or("");
    let title = re
        .replace(first_line, |caps: &regex::Captures| {
            format!("{}{:3}", &caps[1], caps[2].parse::<i32>().unwrap_or(0))
        })
        .to_string();

    println!("Đang lưu chương: {}", title);

    let mut path = PathBuf::from(env::var("HOME").unwrap_or_else(|_| ".".to_string()));
    path.push("Desktop/Novel/");
    fs::create_dir_all(&path)?;
    path.push(format!("{}.txt", &title));

    let mut file = File::create(path)?;
    file.write_all(final_result.as_bytes())?;
    Ok(title)
}

async fn download_truyencv(base_url: &str) -> Result<(), Box<dyn Error>> {
    let index = Arc::new(Mutex::new(1));
    let mut set = JoinSet::new();
    let mut consecutive_errors = 0;

    loop {
        while set.len() < NTHREAD {
            let mut num = index.lock().unwrap();
            let current_chap = *num;
            *num += 1;
            drop(num);

            let current_url = get_url_metruyenchu(base_url, current_chap);
            set.spawn(async move { fetch_content_metruyenchu(current_url).await });
        }

        if let Some(res) = set.join_next().await {
            match res {
                Ok(Ok(title)) => {
                    println!("Đã tải chương {}", title);
                    consecutive_errors = 0;
                }
                Ok(Err(_)) => {
                    println!("Không tìm thấy chương!");
                    break;
                }
                Err(e) => println!("🔥 Lỗi hệ thống: {:?}", e),
            }
        }
    }
    Ok(())
}

fn extract_chapter_number(path: &PathBuf) -> Option<u32> {
    // Tìm số chương từ tên file
    let file_name = path.file_name()?.to_str()?;
    let re = Regex::new(r"(\d+)").unwrap(); // Tìm số đầu tiên trong tên file
    re.captures(file_name)
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse().ok())
}
fn merge_txt() {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let path = PathBuf::from(home.clone()).join("Desktop/Novel");
    let target_ext = OsStr::new("txt");

    let files = fs::read_dir(&path).expect("Failed to read directory");

    let mut txt_paths: Vec<PathBuf> = files
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(OsStr::to_str) == Some("txt"))
        .collect();

    txt_paths.sort_by(|a, b| {
        let num_a = extract_chapter_number(a).unwrap_or(0);
        let num_b = extract_chapter_number(b).unwrap_or(0);
        num_a.cmp(&num_b)
    });

    for path in &txt_paths {
        println!("Found txt file: {:?}", path);
    }

    let file_bytes: Vec<u8> = txt_paths
        .into_iter()
        .flat_map(|path| {
            let mut content = fs::read(&path).unwrap_or_else(|_| {
                eprintln!("Failed to read {:?}", path);
                vec![]
            });
            content.extend(b"\n");
            content
        })
        .collect();

    // Tạo file mới để ghi nội dung
    let merge_file_path = format!("{}/Desktop/merge.txt", home);
    let mut file = File::create(&merge_file_path).expect("Failed to create file");

    // Ghi nội dung vào file
    if let Err(e) = file.write_all(&file_bytes) {
        eprintln!("Failed to write to file: {}", e);
    } else {
        println!("Successfully merged into {}", merge_file_path);
    }
}

// #[tokio::main]
// async fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let base_url = "https://metruyencv.com/truyen/som-dang-luc-the-gioi-tro-choi-bat-dau-thong-gia-nu-de/chuong-301";
//     // download_truyencv(&args[1]).await;
//      download_truyencv(base_url).await;
//
//     // merge_txt();
// }
//
mod subtitle;

fn main() {
    let blocks =
        subtitle::parserVtt::parse_vtt("/home/mintori/Downloads/Youtube/Unit24 西山先生1.ja.vtt");
    for entry in blocks {
        let text = subtitle::parserVtt::clean_vtt_line(&entry);
        if !text.trim().is_empty() {
            println!("{}", text.trim());
        }
    }

    // subtitle::writeVtt::write_vtt("/home/mintori/Downloads/Youtube/Unit24 西山先生2.ja.vtt", &blocks);
}
