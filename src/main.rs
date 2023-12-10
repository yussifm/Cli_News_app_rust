use std::error::Error;



fn main() -> Result<(), Box<dyn Error>> {

    let url = "https://newsapi.org/v2/top-headlines?country=us&category=business&apiKey=789c58dc4026459fb27b1f142a9844f1";
    let articles = get_articles(url)?;

    render_articles(&articles); // Call the correct function name 'render_articles'

    Ok(())
}
