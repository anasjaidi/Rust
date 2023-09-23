use bytes::Bytes;
use reqwest;
use scraper::{Html, Selector};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use tokio;

async fn get_page_html(link: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(link).await?.text().await?;
    Ok(body)
}

async fn get_image_bytes_by_link(link: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
    let body = reqwest::get(link).await?.bytes().await?;
    Ok(body)
}

async fn get_images(link: &str) -> Vec<String> {
    let mut imgs: Vec<String> = Vec::new();

    let body = get_page_html(link).await.unwrap();

    let html = Html::parse_document(&body);

    let div_selector = Selector::parse("div.reading-content").unwrap();

    let div = html.select(&div_selector).next().unwrap();

    let img_selector = Selector::parse("img").unwrap();

    for img in div.select(&img_selector) {
        imgs.push(img.value().attr("src").unwrap().to_string());
    }

    imgs
}

#[allow(unused)]
async fn save_image(link: &str, manga_name: &str, chapter: &str) {
    let image_bytes = get_image_bytes_by_link(link).await.unwrap();
    fs::create_dir_all("images").unwrap();
    let filname = format!("images/{}/{}.jpg", manga_name, chapter);
    let path = Path::new(&filname);
    let mut file = File::create(path).unwrap();
    file.write_all(&image_bytes);
    file.flush();
}

#[tokio::main]
#[allow(unused)]
async fn main() -> Result<(), io::Error> {
    
    

    Ok(())
}
