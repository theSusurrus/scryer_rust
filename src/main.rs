use clap::Parser;

mod http_handling;
mod scryfall;

#[derive(Parser)]
struct Cli {
    query: String,
    print: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let cards = scryfall::query(&cli.query.as_str()).unwrap();

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
