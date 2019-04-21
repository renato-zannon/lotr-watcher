use super::{Tourdate, TourdateSource};
use select::{document::Document, predicate::Class};

pub struct Soen;

impl TourdateSource for Soen {
    const NAME: &'static str = "Soen";
    const HTML_PAGE: &'static str = "https://soenmusic.com/tourdates/";
    const S3_KEY: &'static str = "soen-hash";

    fn extract_tourdates(doc: &Document) -> Vec<Tourdate> {
        let mut all_tourdates = Vec::new();

        for tourdate_node in doc.find(Class("tourwidget-item")) {
            if let Some(tourdate) = Self::extract_tourdate(tourdate_node) {
                all_tourdates.push(tourdate);
            }
        }

        all_tourdates
    }
}

impl Soen {
    fn extract_tourdate(node: select::node::Node<'_>) -> Option<Tourdate> {
        let city = node
            .find(Class("city"))
            .next()
            .map(|c| c.text().trim().to_string())?;

        let day = node.find(Class("tour-day-w")).next()?;
        let month = node.find(Class("tour-month-w")).next()?;
        let date = format!("{} {}", day.text(), month.text());

        let buy_link = node
            .find(Class("tour-button"))
            .next()
            .and_then(|n| n.attr("href"))
            .filter(|l| l.trim().len() > 0)
            .map(|l| l.to_string());

        Some(Tourdate {
            buy_link,
            date,
            city,
        })
    }
}
