use failure::Error;
use lettre::smtp::authentication::Credentials;
use lettre::{Transport, SmtpClient, SendableEmail};
use lettre_email::Email;

use tourdates::Tourdate;
use config::Config;

pub fn send_notification(config: &Config, tourdates: &[Tourdate]) -> Result<(), Error> {
    let email = build_email(config, tourdates)?;

    let credentials = Credentials::new(
        config.email_username.to_string(),
        config.email_password.to_string(),
    );

    let mut mailer = SmtpClient::new_simple(&config.email_server)?
        .credentials(credentials)
        .smtp_utf8(true)
        .transport();

    mailer.send(email)?;

    Ok(())
}

fn build_email(config: &Config, tourdates: &[Tourdate]) -> Result<SendableEmail, Error> {
    let mut builder = Email::builder()
        .subject("New LOTR in concert tour dates!")
        .from(config.email_sender.as_str())
        .html(build_body(tourdates));

    for recipient in &config.email_recipients {
        builder = builder.to(recipient.as_str());
    }

    Ok(builder.build()?.into())
}

fn build_body(tourdates: &[Tourdate]) -> String {
    let table_head =
        "<table><thead><th>Date</th><th>City</th><th>Link</th></thead><tbody>".to_string();
    let table_footer = "</tbody></table>".to_string();

    let table_body = tourdates.iter().map(|tourdate| {
        let link = match tourdate.buy_link {
            Some(ref link) => format!("<a href=\"{}\">Buy</a>", link),
            None => "Not available".to_string(),
        };

        format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
            tourdate.date, tourdate.city, link
        )
    });

    let mut body = vec![table_head];
    body.extend(table_body);
    body.push(table_footer);

    body.join("")
}
