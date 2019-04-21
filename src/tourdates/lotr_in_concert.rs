use super::{Tourdate, TourdateSource};
use select::{document::Document, predicate::Class};

pub struct LotrInConcert;

impl TourdateSource for LotrInConcert {
    const NAME: &'static str = "LOTR in Concert";
    const HTML_PAGE: &'static str = "http://lordoftheringsinconcert.com/tour-dates/";
    const S3_KEY: &'static str = "last-hash";

    fn extract_tourdates(doc: &Document) -> Vec<Tourdate> {
        let mut all_tourdates = Vec::new();

        for tourdate_node in doc.find(Class("tourdate-full")) {
            if let Some(tourdate) = Self::extract_tourdate(tourdate_node) {
                all_tourdates.push(tourdate);
            }
        }

        all_tourdates
    }
}

impl LotrInConcert {
    fn extract_tourdate(node: select::node::Node<'_>) -> Option<Tourdate> {
        let city = node.find(Class("tourdate-venue")).next()?;
        let date = node.find(Class("tourdate-date")).next()?;
        let link_str = node
            .find(Class("tourdate-details"))
            .next()
            .and_then(|n| n.attr("href"))?;

        let link;
        if link_str.trim().len() == 0 {
            link = None;
        } else {
            link = Some(link_str.to_string());
        }

        Some(Tourdate {
            buy_link: link,
            date: date.text(),
            city: city.text(),
        })
    }
}
