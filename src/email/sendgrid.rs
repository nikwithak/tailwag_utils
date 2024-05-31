use std::{
    collections::HashMap,
    env::{self, VarError},
    fmt::Display,
};

use reqwest;
use serde::Serialize;

use crate::templates::{self, TemplateError};

pub struct EmailTemplate {
    subject: String,
    body_template: String,
}
type TemplateId = String;
pub type Templates = HashMap<TemplateId, EmailTemplate>;
pub struct SendGridEmailClient {
    api_key: String,
    sender_email: String,
    templates: Templates,
}

impl SendGridEmailClient {
    pub fn new(
        api_key: String,
        sender_email: String,
        templates: Templates,
    ) -> Self {
        Self {
            api_key,
            sender_email,
            templates,
        }
    }

    pub fn from_env() -> Result<Self, SendGridError> {
        let ret = Self {
            api_key: std::env::var("SENDGRID_API_KEY")?,
            sender_email: std::env::var("SENDGRID_SENDER_EMAIL")?,
            templates: Default::default(),
        };
        Ok(ret)
    }

    pub async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body_content: &str,
    ) -> Result<(), SendGridError> {
        let response = reqwest::Client::new()
            .post("https://api.sendgrid.com/v3/mail/send")
            .header("Authorization", format!("BEARER {}", &self.api_key))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&SendGridEmailRequest::new(
                to.to_string(),
                self.sender_email.to_string(),
                subject.to_string(),
                body_content.to_string(),
            ))?)
            .send()
            .await?;

        response.error_for_status()?;
        Ok(())
    }

    pub async fn send_template(
        &self,
        to: &str,
        template_id: &TemplateId,
        template_params: &HashMap<String, String>,
    ) -> Result<(), SendGridError> {
        let template = self
            .templates
            .get(template_id)
            .ok_or(SendGridError::TemplateNotFound(template_id.clone()))?;
        let filled_template = templates::fill_template(&template.body_template, template_params)?;
        self.send_email(to, &template.subject, &filled_template).await
    }
}

#[derive(Serialize)]
struct SendGridEmailRequest {
    personalizations: Vec<SendGridPersonalization>,
    from: SendGridEmailDetail,
    subject: String,
    content: Vec<SendGridContent>,
}

#[derive(Serialize)]
struct SendGridEmailDetail {
    email: String,
}

#[derive(Serialize)]
struct SendGridPersonalization {
    to: Vec<SendGridEmailDetail>,
}

#[derive(Serialize)]
enum SendGridContentType {
    #[serde(rename = "text/plain")]
    Plaintext,
    #[serde(rename = "text/html")]
    Html,
}

#[derive(Serialize)]
struct SendGridContent {
    #[serde(rename = "type")]
    content_type: SendGridContentType,
    value: String,
}

#[derive(Debug)]
pub enum SendGridError {
    Serialization(serde_json::Error),
    Http(reqwest::Error),
    Initialization(VarError),
    TemplateNotFound(TemplateId),
    TemplateError(TemplateError),
}
impl Display for SendGridError {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<TemplateError> for SendGridError {
    fn from(value: TemplateError) -> Self {
        SendGridError::TemplateError(value)
    }
}
impl From<VarError> for SendGridError {
    fn from(value: VarError) -> Self {
        SendGridError::Initialization(value)
    }
}
impl From<reqwest::Error> for SendGridError {
    fn from(value: reqwest::Error) -> Self {
        SendGridError::Http(value)
    }
}
impl From<serde_json::Error> for SendGridError {
    fn from(value: serde_json::Error) -> Self {
        SendGridError::Serialization(value)
    }
}

// Reference SendGrid API for details
impl SendGridEmailRequest {
    fn new(
        to_email: String,
        from_email: String,
        subject: String,
        email_body: String,
    ) -> Self {
        // Quick and dirty - TODO revamp this later
        SendGridEmailRequest {
            personalizations: vec![SendGridPersonalization {
                to: vec![SendGridEmailDetail {
                    email: to_email,
                }],
            }],
            from: SendGridEmailDetail {
                email: from_email,
            },
            subject,
            content: vec![SendGridContent {
                content_type: SendGridContentType::Plaintext,
                value: email_body,
            }],
        }
    }
}
