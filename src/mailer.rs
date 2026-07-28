use crate::config::Config;
use anyhow::Result;
use lettre::message::MessageBuilder;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use log::{error, info, warn};

pub fn create_mailer(config: &Config) -> Option<SmtpTransport> {
    let host = config.smtp_host.as_ref()?;
    let username = config.smtp_user_name.as_ref()?;
    let password = config.smtp_passwd.as_ref()?;

    let creds = Credentials::new(username.clone(), password.clone());

    let mailer = if config.unsafe_deploy {
        warn!("Mailer using PLAIN CONTENT,pls DON'T use this config IN PRODUCTION!!!!");
        SmtpTransport::builder_dangerous(host)
            .credentials(creds)
            .build()
    } else {
        SmtpTransport::relay(host).ok()?.credentials(creds).build()
    };
    match mailer.test_connection() {
        Ok(yes) if yes => {
            info!("smtp Server connction success!");
        }
        Err(e) => {
            error!("Try connction smtp Server ERROR: {}", e);
            return None;
        }
        _ => {
            error!("Try connction smtp Server FAILED");
            return None;
        }
    }
    Some(mailer)
}

pub fn send_email(
    username: &str,
    message: String,
    to_: String,
    mailer: &SmtpTransport,
) -> Result<()> {
    let mail = MessageBuilder::new()
        .from(username.parse()?)
        .to(to_.parse()?)
        .body(message)?;
    mailer.send(&mail)?;
    Ok(())
}
