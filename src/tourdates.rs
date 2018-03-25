use blake2::{Blake2b, Digest};
use select::document::Document;
use select::predicate::Class;
use failure::Error;
use select;
use reqwest;

pub fn fetch_tourdates() -> Result<Vec<Tourdate>, Error> {
    let html = reqwest::get("http://lordoftheringsinconcert.com/tour-dates/")?.text()?;
    let doc = Document::from(html.as_ref());

    let mut all_tourdates = Vec::new();

    for tourdate_node in doc.find(Class("tourdate-full")) {
        if let Some(tourdate) = extract_tourdate(tourdate_node) {
            all_tourdates.push(tourdate);
        }
    }

    Ok(all_tourdates)
}

pub fn compute_updated_hash(tourdates: &[Tourdate]) -> Result<Vec<u8>, Error> {
    let mut hasher = Blake2b::new();

    for tourdate in tourdates {
        tourdate.push_hash(&mut hasher);
    }

    Ok(hasher.result().into_iter().collect())
}

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

fn extract_tourdate(node: select::node::Node) -> Option<Tourdate> {
    let city = node.find(Class("tourdate-venue")).next()?;
    let date = node.find(Class("tourdate-date")).next()?;
    let link_str = node.find(Class("tourdate-details"))
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
