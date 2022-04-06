/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.155
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiText {
    /// The message's context. Useful when customizing messages.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    #[serde(rename = "id")]
    pub id: i64,
    /// The message text. Written in american english.
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "type")]
    pub _type: String,
}

impl UiText {
    pub fn new(id: i64, text: String, _type: String) -> UiText {
        UiText {
            context: None,
            id,
            text,
            _type,
        }
    }
}


