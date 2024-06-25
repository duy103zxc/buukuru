use scraper::{Html, Selector};
use crate::utils;

pub fn fetch_novel(novel_id: &str)  {
    // element selectors
    let base_url = String::from("https://ncode.syosetu.com/");
    let novel_url = base_url + novel_id;
    let body = utils::get_body_from_url(&novel_url);
    let document = Html::parse_document(&body);
    // Metadata
    let title_selector = Selector::parse("p.novel_title").unwrap();
    let author = Selector::parse("div.novel_writername").unwrap();
    // List of chapters
    let list_selector = Selector::parse("div.index_box").unwrap();
    let id_selector = Selector::parse("a").unwrap();
            
    let name = document.select(&title_selector).next().unwrap().text().collect::<Vec<_>>().join("");
    let author = document.select(&author).next().unwrap().text().collect::<Vec<_>>().join("");
    let ul = document.select(&list_selector).next().unwrap();
    let each_chap_url = ul.select(&id_selector).into_iter().map(|element| element.value().attr("href").unwrap()).collect::<Vec<_>>();
    
    for chap in each_chap_url {
        let current_url =  format!("https://ncode.syosetu.com/{}", chap);
        fetch_chapter(current_url.as_str());
    }
    
}

fn fetch_chapter(chap_link: &str) -> (String, Vec<String>) {
    // Fetch from the page
    let body = utils::get_body_from_url(&chap_link);
    let document = Html::parse_document(&body);
    // Selector
    let title_selector = Selector::parse("p.novel_subtitle").unwrap();
    let novel_selector = Selector::parse("div.novel_view").unwrap();
    let p_selector = Selector::parse("p").unwrap();
        
    let chap_name = document.select(&title_selector).next().unwrap().text().collect::<Vec<_>>().join("");    
    let ul = document.select(&novel_selector).next().unwrap();
    
    let text = ul.select(&p_selector)
    .map(|txt| txt.html())
    .collect::<Vec<_>>();
     
    (chap_name, text) 
}
