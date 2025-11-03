use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let reponse_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())

    }
}

fn hello() -> String{
 return "hellow"
}

fn main(){
    let args: Vec<String> = std::env::args().collect();
    trpl::run(async {
    let url = &args[1];
    let s = 23;
    match page_title(url).await {
        Some(title) => println!("the title for {url} was {title}"),
        None => println!("{url} had not title"),
    }   
    println!(hello());
})}
