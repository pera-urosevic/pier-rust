use sendgrid::{Mail, SGClient, SendgridError};

pub fn send(subject: &str, text: &str) -> Result<(), SendgridError> {
    let key = std::env::var("SENDGRID_KEY").expect("SENDGRID_KEY env");
    let email_to = std::env::var("SENDGRID_TO").expect("SENDGRID_TO env");
    let email_name = std::env::var("SENDGRID_TO_NAME").expect("SENDGRID_TO_NAME env");
    let email_from = std::env::var("SENDGRID_FROM").expect("SENDGRID_FROM env");
    let mail = Mail::new()
        .add_to((email_to.as_str(), email_name.as_str()).into())
        .add_from(email_from.as_str())
        .add_subject(subject)
        .add_text(text);
    let _response = SGClient::new(key).send(mail)?;
    Ok(())
}
