use env_logger;
#[macro_use]
extern crate failure;

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
    let tourdates = tourdates::fetch_tourdates()?;
    let hash = tourdates::compute_updated_hash(&tourdates[..])?;

    let matches = s3_client.matches_existing_hash(&hash[..])?;

    if matches {
        println!("No new tour dates");
    } else {
        println!("New tour dates!");
        s3_client.update_hash(hash)?;
        email::send_notification(&config, &tourdates[..])?;
    }

    Ok(())
}
