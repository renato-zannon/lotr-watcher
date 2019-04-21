use env_logger;
#[macro_use]
extern crate failure;

use crate::tourdates::{lotr_in_concert::LotrInConcert, soen::Soen, TourdateSource};
use failure::Error;
use openssl_probe;

mod config;
mod email;
mod s3;
mod tourdates;

fn main() {
    env_logger::init();
    openssl_probe::init_ssl_cert_env_vars();
    run().unwrap();
}

fn run() -> Result<(), Error> {
    let config = config::from_env()?;

    let s3_client = s3::new_client(&config);
    check_tourdates::<LotrInConcert>(&s3_client, &config)?;
    check_tourdates::<Soen>(&s3_client, &config)?;

    Ok(())
}

fn check_tourdates<S: TourdateSource>(
    s3_client: &s3::Client,
    config: &config::Config,
) -> Result<(), Error> {
    let source_client = s3_client.tourdate_client::<S>();

    let tourdates = S::fetch_tourdates()?;
    let hash = tourdates::compute_updated_hash(&tourdates[..])?;

    let matches = source_client.matches_existing_hash(&hash[..])?;

    if matches {
        println!("[{}] No new tour dates", S::NAME);
    } else {
        println!("[{}] New tour dates!", S::NAME);
        source_client.update_hash(hash)?;
        email::send_notification::<S>(&config, &tourdates[..])?;
    }

    Ok(())
}
