use std::collections::HashMap;

use crate::{email::error::EmailError, templates};

pub mod error;
#[cfg(feature = "sendgrid")]
pub mod sendgrid;

pub type EmailAddress = str;

pub struct EmailTemplate {
    subject: String,
    body_template: String,
}
type TemplateId = String;
pub type Templates = HashMap<TemplateId, EmailTemplate>;

pub trait EmailProvider {
    fn send_email(
        &self,
        to: &EmailAddress,
        subject: &str,
        body_content: &str,
    ) -> impl std::future::Future<Output = Result<(), EmailError>> + Send;
}

pub struct EmailClient<T: EmailProvider> {
    provider: T,
    templates: Templates,
}

impl<T: EmailProvider> EmailProvider for EmailClient<T> {
    fn send_email(
        &self,
        to: &EmailAddress,
        subject: &str,
        body_content: &str,
    ) -> impl std::future::Future<Output = Result<(), EmailError>> + Send {
        self.provider.send_email(to, subject, body_content)
    }
}

impl<T: EmailProvider> EmailClient<T> {
    pub fn new(
        provider: T,
        templates: Templates,
    ) -> Self {
        Self {
            provider,
            templates,
        }
    }
    pub async fn send_template(
        &self,
        to: &str,
        template_id: &TemplateId,
        template_params: &HashMap<String, String>,
    ) -> Result<(), EmailError> {
        let template = self
            .templates
            .get(template_id)
            .ok_or(EmailError::TemplateNotFound(template_id.clone()))?;
        let filled_template = templates::fill_template(&template.body_template, template_params)?;
        self.send_email(to, &template.subject, &filled_template).await
    }
}
