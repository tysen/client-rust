/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// EmailTemplateData : Contains the data of the email template, including the subject and body in HTML and plaintext variants



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailTemplateData {
    #[serde(rename = "body")]
    pub body: Box<crate::models::EmailTemplateDataBody>,
    #[serde(rename = "subject")]
    pub subject: String,
}


impl EmailTemplateData {
    /// Contains the data of the email template, including the subject and body in HTML and plaintext variants
    pub fn new(body: crate::models::EmailTemplateDataBody, subject: String) -> EmailTemplateData {
        EmailTemplateData {
                body: Box::new(body),
                subject,
        }
    }
}


