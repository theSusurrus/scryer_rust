use clap::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde_json;
use std::fmt;
use crate::http_handling as http_handling;

fn deserialize_integer<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<u64>, D::Error> {
    let value = serde_json::Value::deserialize(deserializer)?;
    let optional_integer = match value {
        serde_json::Value::Number(num) => Some(num.as_u64().unwrap()),
        serde_json::Value::String(num_string) => Some(num_string.parse::<u64>().unwrap()),
        _ => None,
    };
    Ok(optional_integer)
}

fn deserialize_float<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<f64>, D::Error> {
    Ok(match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Number(num) => Some(
            num.as_f64()
                .ok_or(serde::de::Error::custom("Invalid integers"))?,
        ),
        serde_json::Value::String(num) => Some(num.parse::<f64>().unwrap()),
        _ => None,
    })
}

#[derive(Debug, Deserialize, Clone)]
pub struct CardPrices {
    #[serde(deserialize_with = "deserialize_float")]
    eur: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CardFace {
    name: String,
    type_line: String,
    #[serde(default)]
    oracle_text: String,
    #[serde(default)]
    mana_cost: String,
    power: Option<String>,
    toughness: Option<String>,
}

impl fmt::Display for CardFace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}",
            self.name, self.type_line, self.mana_cost, self.oracle_text
        )
        .unwrap();
        if self.toughness.is_some() && self.power.is_some() {
            write!(
                f,
                "\n{}/{}",
                self.power.as_ref().unwrap(),
                self.toughness.as_ref().unwrap()
            )
            .unwrap();
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Card {
    // #[serde(deserialize_with = "deserialize_integer")]
    // cmc: Option<u64>,
    // color_identity: Vec<String>,
    // #[serde(default)]
    // colors: Vec<String>,
    #[serde(default)]
    mana_cost: String,
    name: String,
    type_line: String,
    layout: String,
    #[serde(default)]
    card_faces: Option<Vec<CardFace>>,
    #[serde(default)]
    oracle_text: String,
    prices: CardPrices,
    power: Option<String>,
    toughness: Option<String>,
}

impl Card {
    pub fn get_name(&self) -> String {
        let card_clone = self.clone();
        card_clone.name
    }
}

fn write_faces(f: &mut fmt::Formatter<'_>, faces: &Vec<CardFace>) -> Result<(), std::io::Error> {
    for face in faces.iter() {
        write!(f, "{}\n", face).unwrap();
    }
    Ok(())
}

fn write_normal(f: &mut fmt::Formatter<'_>, card: &Card) -> Result<(), std::io::Error> {
    write!(
        f,
        "{}\n{}\n{}",
        card.type_line, card.mana_cost, card.oracle_text
    )
    .unwrap();
    if card.toughness.is_some() && card.power.is_some() {
        write!(
            f,
            "\n{}/{}",
            card.power.as_ref().unwrap(),
            card.toughness.as_ref().unwrap()
        )
        .unwrap();
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
            _ => write_normal(f, self).unwrap(),
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CardCollection {
    #[serde(deserialize_with = "deserialize_integer", default)]
    total_cards: Option<u64>,
    data: Vec<Card>,
    has_more: bool,
}

impl CardCollection {
    pub fn get_cards(&self) -> Vec<Card> {
        let data_clone = self.data.clone();
        data_clone
    }

    pub fn sum_prices(&self) -> f64 {
        let mut sum: f64 = 0.0;
    
        for card in self.data.iter() {
            sum += card.prices.eur.unwrap_or_default();
        }
    
        sum
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

pub fn query_scryfall(query: &str) -> Result<CardCollection, ()> {
    let mut scryfall_uri: String = "https://api.scryfall.com/cards/search?q=".to_owned();

    scryfall_uri.push_str(query);

    let response = http_handling::get_http(scryfall_uri);

    let cards: CardCollection =
        serde_json::from_str(&response).expect("JSON format error");

    Ok(cards)
}
