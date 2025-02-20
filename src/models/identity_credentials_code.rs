/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// IdentityCredentialsCode : CredentialsCode represents a one time login/registration code



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityCredentialsCode {
    #[serde(rename = "address_type", skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    #[serde(rename = "used_at", skip_serializing_if = "Option::is_none")]
    pub used_at: Option<String>,
}

impl Default for IdentityCredentialsCode {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityCredentialsCode {
    /// CredentialsCode represents a one time login/registration code
    pub fn new() -> IdentityCredentialsCode {
        IdentityCredentialsCode {
                address_type: None,
                used_at: None,
        }
    }
}


