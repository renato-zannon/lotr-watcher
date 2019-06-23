use blake2::{Blake2b, Digest};
use failure::Error;
use reqwest;
use select::document::Document;

pub mod cellar_darling;
pub mod lotr_in_concert;
pub mod soen;

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

pub trait TourdateSource {
    const NAME: &'static str;
    const HTML_PAGE: &'static str;
    const S3_KEY: &'static str;

    fn fetch_tourdates() -> Result<Vec<Tourdate>, Error> {
        let html = reqwest::get(Self::HTML_PAGE)?.text()?;
        let doc = Document::from(html.as_ref());

        Ok(Self::extract_tourdates(&doc))
    }

    fn enabled() -> bool {
        true
    }

    fn extract_tourdates(doc: &Document) -> Vec<Tourdate>;
}

pub fn compute_updated_hash(tourdates: &[Tourdate]) -> Result<Vec<u8>, Error> {
    let mut hasher = Blake2b::new();

    for tourdate in tourdates {
        tourdate.push_hash(&mut hasher);
    }

    Ok(hasher.result().into_iter().collect())
}
