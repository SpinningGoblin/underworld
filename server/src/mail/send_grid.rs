use std::env;

use serde::Serialize;

#[derive(Serialize)]
pub struct SendGridRequest {
    pub personalizations: Vec<Personalization>,
    pub from: EmailInfo,
    pub template_id: String,
}

#[derive(Serialize)]
pub struct Personalization {
    pub to: Vec<EmailInfo>,
    pub dynamic_template_data: TemplateData,
}

#[derive(Serialize)]
pub struct TemplateData {
    callback: String,
}

#[derive(Serialize)]
pub struct EmailInfo {
    pub email: String,
}

pub async fn send_mail(to: &str, from: &str, callback: &str) {
    let to_info = EmailInfo {
        email: to.to_string(),
    };
    let from_info = EmailInfo {
        email: from.to_string(),
    };
    let personalization = Personalization {
        to: vec![to_info],
        dynamic_template_data: TemplateData {
            callback: callback.to_string(),
        },
    };
    let request = SendGridRequest {
        personalizations: vec![personalization],
        from: from_info,
        template_id: env::var("SENDGRID_TEMPLATE_ID").unwrap(),
    };

    let api_key = env::var("SENDGRID_API_KEY").unwrap();

    reqwest::Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await
        .unwrap();
}
