use futures::executor::block_on;
use serde_json;
use serde::Deserialize;
use serde::Deserializer;
use std::fmt;
use clap::{Parser};

async fn query(uri: String) -> String{
    let response = reqwest::get(uri).await.unwrap().text().await.unwrap();
    return response;
}

fn deserialize_integer<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<u64>, D::Error> {
    Ok(match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Number(num) => Some(num.as_f64().ok_or(serde::de::Error::custom("Invalid integers"))? as u64),
        _ => None
    })
}

fn deserialize_float <'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<f64>, D::Error> {
    Ok(match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Number(num) => Some(num.as_f64().ok_or(serde::de::Error::custom("Invalid integers"))?),
        serde_json::Value::String(num) => Some(num.parse::<f64>().unwrap()),
        _ => None
    })
}

#[derive(Debug, Deserialize)]
struct CardPrices {
    #[serde(deserialize_with = "deserialize_float")]
    eur: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct CardFace {
    name: String,
    type_line: String,
    #[serde(default)]
    oracle_text: String,
    #[serde(default)]
    mana_cost: String,
}

#[derive(Debug, Deserialize)]
struct Card {
    // #[serde(deserialize_with = "deserialize_integer")]
    // cmc: Option<u64>,
    // color_identity: Vec<String>,
    // #[serde(default)]
    // colors: Vec<String>, 
    #[serde(default)]
    mana_cost: String,
    name: String,
    layout: String,
    #[serde(default)]
    card_faces: Option<Vec<CardFace>>,
    #[serde(default)]
    oracle_text: String,
    prices: CardPrices,
}

#[derive(Debug, Deserialize)]
struct CardCollection {
    #[serde(deserialize_with = "deserialize_integer", default)]
    total_cards: Option<u64>,
    data: Vec<Card>
}

impl fmt::Display for CardFace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}\n{}", self.name, self.mana_cost, self.type_line, self.oracle_text).unwrap();
        Ok(())
    }
}


fn write_faces(f: &mut fmt::Formatter<'_>, faces: &Vec<CardFace>) -> Result<(), std::io::Error> {
    for face in faces.iter() {
        write!(f, "{}\n", face).unwrap();
    }
    Ok(())
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self.name).unwrap();
        match self.layout.as_str() {
            "transform" => write_faces(f, self.card_faces.as_ref().unwrap()).unwrap(),
            "adventure" => write_faces(f, self.card_faces.as_ref().unwrap()).unwrap(),
            "modal_dfc" => write_faces(f, self.card_faces.as_ref().unwrap()).unwrap(),
            _ => write!(f, "{}\n{}", self.mana_cost, self.oracle_text).unwrap(),
        }
        Ok(())
    }
}

impl fmt::Display for CardCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} cards\n\n", self.total_cards.unwrap_or_default()).unwrap();

        for card in self.data.iter() {
            write!(f, "{}\n\n", card).unwrap();
        }

        Ok(())
    }
}

fn sum_prices(collection: CardCollection) -> f64 {
    let mut sum: f64 = 0.0;

    for card in collection.data.iter() {
        sum += card.prices.eur.unwrap_or_default();
    }

    sum
}

#[derive(Parser)]
struct Cli {
    query: String
}

#[tokio::main]
async fn main() {
    let mut scryfall_uri: String = "https://api.scryfall.com/cards/search?q=".to_owned();

    let cli = Cli::parse();

    scryfall_uri.push_str(cli.query.as_str());

    let response = block_on(query(scryfall_uri));

    let cards: CardCollection = serde_json::from_str(&response).expect("JSON format error");

    println!("{}", cards);

    println!("{} EUR", sum_prices(cards));
}
