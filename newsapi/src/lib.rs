
use std::error::Error;

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
     
    }
}