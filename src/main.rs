mod anki;
mod metruyenchu;
mod sanfoundry;
mod subtitle;
mod util;
use std::path::PathBuf;
#[tokio::main]
async fn main() {
    // let base_url = "https://metruyencv.com/truyen/som-dang-luc-the-gioi-tro-choi-bat-dau-thong-gia-nu-de/chuong-301";
    // download_truyencv(&args[1]).await;
    // metruyenchu::download::download_truyencv(base_url).await;
    // merge_txt();

    let url = "https://www.sanfoundry.com/1000-csharp-questions-answers/";
    let path = PathBuf::from("/home/mintori/Desktop/CSharp-MCQ/");
    let vec = sanfoundry::get_sanfoundry::get_url(url).unwrap_or_default();
    for element in vec {
        let _ = sanfoundry::sanfoundry::scrap(&element, path.clone());
    }
}
