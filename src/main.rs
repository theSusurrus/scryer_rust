use futures::executor::block_on;
use serde_json;
use serde::Deserialize;
use serde::Deserializer;
use std::fmt;

async fn query(uri: String) -> String{
    let response = reqwest::get(uri).await.unwrap().text().await.unwrap();
    return response;
}

fn deserialize_integer<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<u64>, D::Error> {
    Ok(match serde_json::Value::deserialize(deserializer)? {
        // serde_json::Value::String(s) => s.parse().map_err(serde::de::Error::custom)?,
        serde_json::Value::Number(num) => Some(num.as_f64().ok_or(serde::de::Error::custom("Invalid integers"))? as u64),
        // _ => return Err(serde::de::Error::custom("wrong type"))
        _ => None
    })
}

#[derive(Debug, Deserialize)]
struct CardPrices {
    eur: String,
    usd: String
}

#[derive(Debug, Deserialize)]
struct Card {
    artist: String,
    #[serde(deserialize_with = "deserialize_integer")]
    cmc: Option<u64>,
    color_identity: Vec<String>,
    #[serde(default)]
    colors: Vec<String>, 
    name: String,
    // keywords: Vec<String>,
    // mana_cost: String,
    // #[serde(deserialize_with = "deserialize_integer")]
    // power: Option<u64>,
    // #[serde(deserialize_with = "deserialize_integer")]
    // toughness: Option<u64>,
    // rarity: String,
    // set_name: String,
    // type_line: String,
    // prices: CardPrices,
}

#[derive(Debug, Deserialize)]
struct CardCollection {
    #[serde(deserialize_with = "deserialize_integer", default)]
    total_cards: Option<u64>,
    object: String,
    data: Vec<Card>
}



impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.name);
        Ok(())
    }
}

impl fmt::Display for CardCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.total_cards) {
            Some(total_cards) => write!(f, "{} cards\n", total_cards),
            None => Ok(())
        };

        for card in self.data.iter() {
            write!(f, "{}", card);
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut scryfall_uri: String = "https://api.scryfall.com/cards/search?q=".to_owned();
    let scryfall_query: &str = "set:lci";

    scryfall_uri.push_str(scryfall_query);

    let response = block_on(query(scryfall_uri));

    println!("{}", response);

    let cards: CardCollection = serde_json::from_str(&response).expect("JSON format error");

    println!("{:?}", cards);
    println!("{}", cards);
}
