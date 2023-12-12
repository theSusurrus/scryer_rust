use clap::Parser;

mod http_handling;
mod scryfall;

#[derive(Parser)]
struct Cli {
    get_http: String,
    print: Option<String>,
}

#[tokio::main]
async fn main() {
    let mut scryfall_uri: String = "https://api.scryfall.com/cards/search?q=".to_owned();

    let cli = Cli::parse();

    scryfall_uri.push_str(cli.get_http.as_str());

    let response = http_handling::get_http(scryfall_uri);

    let cards: scryfall::CardCollection =
        serde_json::from_str(&response).expect("JSON format error");

    let cards_clone = cards.clone();
    let full_print = || {
        println!("{}", cards_clone);
        println!("{} EUR", cards_clone.sum_prices());
    };

    match cli.print {
        Some(print_string) => match print_string.as_str() {
            "prices" => println!("{} EUR", cards.sum_prices()),
            "names" => {
                for card in cards.get_cards().iter() {
                    println!("{}", card.get_name())
                }
            }
            "full" => full_print(),
            _ => panic!("Unknown print format"),
        },
        None => {
            full_print();
        }
    }
}
