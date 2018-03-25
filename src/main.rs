extern crate blake2;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate openssl_probe;
extern crate reqwest;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate select;
extern crate lettre;
extern crate lettre_email;

use failure::Error;

mod s3;
mod tourdates;
mod config;
mod email;

fn main() {
    env_logger::init();
    openssl_probe::init_ssl_cert_env_vars();
    run().unwrap();
}

fn run() -> Result<(), Error> {
    let config = config::from_env()?;

    let s3_client = s3::new_client(&config);
    let hash = tourdates::compute_updated_hash()?;

    let matches = s3_client.matches_existing_hash(&hash[..])?;

    if matches {
        println!("No new tour dates");
    } else {
        println!("New tour dates!");
        s3_client.update_hash(hash)?;
        email::send_notification(&config)?;
    }

    Ok(())
}
