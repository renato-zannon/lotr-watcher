use failure::Error;
use lettre::smtp::authentication::Credentials;
use lettre::{EmailTransport, SmtpTransport};
use lettre_email::{Email, EmailBuilder};

use config::Config;

pub fn send_notification(config: &Config) -> Result<(), Error> {
    let email = build_email(config)?;

    let credentials = Credentials::new(
        config.email_username.to_string(),
        config.email_password.to_string()
    );

    let mut mailer = SmtpTransport::simple_builder(config.email_server.clone())?
        .credentials(credentials)
        .smtp_utf8(true)
        .build();

    mailer.send(&email)?;

    Ok(())
}

fn build_email(config: &Config) -> Result<Email, Error> {
    let mut builder = EmailBuilder::new();
    builder.set_subject("New LOTR in concert tour dates!");
    builder.add_from(config.email_sender.as_str());

    for recipient in &config.email_recipients {
        builder.add_to(recipient.as_str());
    }

    Ok(builder.build()?)
}
