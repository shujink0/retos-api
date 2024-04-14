use anyhow::{anyhow, Result};
use reqwest::Client;
use reqwest::StatusCode;

use crate::dto::{
    SendGridBody, SendGridContent, SendGridFrom, SendGridPersonalization, SendGridTo,
};

use crate::SENDGRID_API_KEY;

pub async fn send_email(to: String, subject: String, html: String) -> Result<bool> {
    let client = Client::new();

    let body = SendGridBody {
        subject,
        from: SendGridFrom {
            email: "info@qwerty.com",
        },
        content: vec![SendGridContent {
            _type: "text/html",
            value: html,
        }],
        personalizations: vec![SendGridPersonalization {
            to: vec![SendGridTo { email: to }],
        }],
    };

    let response = client
        .post("https://api.sendgrid.com/v3/mail/send")
        .bearer_auth(SENDGRID_API_KEY)
        .json(&body)
        .send()
        .await?;

    match response.status() {
        StatusCode::ACCEPTED => Ok(true),
        s => Err(anyhow!(format!("Error al enviar el correo: {}", s))),
    }
}
