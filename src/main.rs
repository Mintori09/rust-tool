mod metruyenchu;
mod subtitle;
#[tokio::main]
async fn main() {
    // let base_url = "https://metruyencv.com/truyen/som-dang-luc-the-gioi-tro-choi-bat-dau-thong-gia-nu-de/chuong-301";
    // download_truyencv(&args[1]).await;
    // metruyenchu::download::download_truyencv(base_url).await;
    // merge_txt();

    let blocks =
        subtitle::parserVtt::parse_vtt("/home/mintori/Downloads/Youtube/Unit24 西山先生1.ja.vtt");
    for entry in blocks {
        let text = subtitle::parserVtt::clean_vtt_line(&entry);
        if !text.trim().is_empty() {
            println!("{}", text.trim());
        }
    }
}
