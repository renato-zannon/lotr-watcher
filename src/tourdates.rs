use blake2::{Blake2b, Digest};
use failure::Error;
use reqwest;
use select::document::Document;
use select::predicate::Class;

#[derive(Debug)]
pub struct Tourdate {
    pub city: String,
    pub date: String,
    pub buy_link: Option<String>,
}

impl Tourdate {
    fn push_hash(&self, hasher: &mut Blake2b) {
        hasher.input(self.city.as_bytes());
        hasher.input(self.date.as_bytes());

        if let Some(ref link) = self.buy_link {
            hasher.input(link.as_bytes());
        }
    }
}

pub struct LotrInConcert;

pub trait TourdateSource {
    const NAME: &'static str;
    const HTML_PAGE: &'static str;
    const S3_KEY: &'static str;

    fn fetch_tourdates() -> Result<Vec<Tourdate>, Error> {
        let html = reqwest::get(Self::HTML_PAGE)?.text()?;
        let doc = Document::from(html.as_ref());

        Ok(Self::extract_tourdates(&doc))
    }

    fn extract_tourdates(doc: &Document) -> Vec<Tourdate>;
}

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

pub fn compute_updated_hash(tourdates: &[Tourdate]) -> Result<Vec<u8>, Error> {
    let mut hasher = Blake2b::new();

    for tourdate in tourdates {
        tourdate.push_hash(&mut hasher);
    }

    Ok(hasher.result().into_iter().collect())
}
