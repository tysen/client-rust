/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.142
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// IdentityState : The state can either be `active` or `inactive`.

/// The state can either be `active` or `inactive`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdentityState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,

}

impl ToString for IdentityState {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("active"),
            Self::Inactive => String::from("inactive"),
        }
    }
}




