use failure::Error;
use std::env;

pub struct Config {
    pub bucket: String,
    pub email_sender: String,
    pub email_recipients: Vec<String>,
    pub email_server: String,
    pub email_username: String,
    pub email_password: String,
}

pub fn from_env() -> Result<Config, Error> {
    let bucket = get_env("AWS_S3_BUCKET")?;
    let email_sender = get_env("EMAIL_SENDER")?;
    let email_recipients = get_env("EMAIL_RECIPIENTS")?.split(',').map(String::from).collect();
    let email_server = get_env("EMAIL_SERVER")?;
    let email_username = get_env("EMAIL_USERNAME")?;
    let email_password = get_env("EMAIL_PASSWORD")?;

    Ok(Config {
        bucket,
        email_sender,
        email_recipients,
        email_server,
        email_username,
        email_password,
    })
}

fn get_env(var: &str) -> Result<String, Error> {
    env::var(var).map_err(|_| format_err!("{} env variable not present", var))
}
