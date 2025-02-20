/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// OrganizationBody : Create B2B SSO Organization Request Body



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationBody {
    /// Domains contains the list of organization's domains.
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// Label contains the organization's label.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl Default for OrganizationBody {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationBody {
    /// Create B2B SSO Organization Request Body
    pub fn new() -> OrganizationBody {
        OrganizationBody {
                domains: None,
                label: None,
        }
    }
}


