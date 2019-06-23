use super::{Tourdate, TourdateSource};
use select::{
    document::Document,
    predicate::{And, Attr, Name},
};
use serde_json::Value;

pub struct CellarDarling;

impl TourdateSource for CellarDarling {
    const NAME: &'static str = "Cellar Darling";
    const HTML_PAGE: &'static str = "https://www.bandsintown.com/a/13612202";
    const S3_KEY: &'static str = "cellar-darling-hash";

    fn extract_tourdates(doc: &Document) -> Vec<Tourdate> {
        let selector = And(Name("script"), Attr("type", "application/ld+json"));
        let tourdate_items = doc
            .find(selector)
            .filter_map(|node| {
                let json = node.text();
                let items = match serde_json::from_str(&json).ok()? {
                    Value::Array(items) => items,
                    _ => return None,
                };

                match items.get(0).and_then(|item| item.get("@type")) {
                    Some(Value::String(s)) if s == "MusicEvent" => Some(items),
                    _ => None,
                }
            })
            .flat_map(|items| {
                items.into_iter().filter_map(|item| match item {
                    Value::Object(map) => Some(map),
                    _ => None,
                })
            });

        tourdate_items
            .filter_map(|item| Self::extract_tourdate(item))
            .collect()
    }
}

impl CellarDarling {
    fn extract_tourdate(node: serde_json::Map<String, Value>) -> Option<Tourdate> {
        let city = node.get("location").and_then(|location| {
            match (location.get("name"), location.get("address")) {
                (Some(Value::String(name)), Some(Value::String(address))) => {
                    Some(format!("{} - {}", name, address))
                }

                _ => None,
            }
        })?;

        let date = node
            .get("startDate")
            .and_then(|date| date.as_str())
            .and_then(|url| format_datetime(url))?;

        let buy_link = node
            .get("url")
            .and_then(|url| url.as_str())
            .map(|url| url.to_string());

        Some(Tourdate {
            buy_link,
            date,
            city,
        })
    }
}

fn format_datetime(s: &str) -> Option<String> {
    use chrono::{Datelike, NaiveDate};

    let dt = NaiveDate::parse_from_str(s.trim(), "%Y-%m-%dT%H:%M:%S").ok()?;

    Some(format!("{:02}/{:02}/{}", dt.day(), dt.month(), dt.year()))
}
