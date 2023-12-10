use std::error::Error;

use colour::{dark_green_ln, yellow_ln};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct  Articles {
    articles: Vec<Article>
}


#[derive(Deserialize, Debug)]
struct Article {
    title: String, 
    url: String
}


fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {

    let response = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}


fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green_ln!("> {}", i.title);
        yellow_ln!("> {}\n\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let url = "https://newsapi.org/v2/top-headlines?country=us&category=business&apiKey=789c58dc4026459fb27b1f142a9844f1";
    let articles = get_articles(url)?;

    render_articles(&articles); // Call the correct function name 'render_articles'

    Ok(())
}
