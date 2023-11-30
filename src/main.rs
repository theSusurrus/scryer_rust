use futures::executor::block_on;

async fn query(uri: String) -> String{
    let response = reqwest::get(uri).await.unwrap().text().await.unwrap();
    return response;
}


#[tokio::main]
async fn main() {
    let mut scryfall_uri: String = "https://api.scryfall.com/cards/search?q=".to_owned();
    let scryfall_query: &str = "Akal";

    scryfall_uri.push_str(scryfall_query);

    let response = block_on(query(scryfall_uri));

    println!("{}", response);
}
