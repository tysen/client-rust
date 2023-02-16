/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.13
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// CreateJsonWebKeySet : Create JSON Web Key Set Request Body



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateJsonWebKeySet {
    /// JSON Web Key Algorithm  The algorithm to be used for creating the key. Supports `RS256`, `ES256`, `ES512`, `HS512`, and `HS256`.
    #[serde(rename = "alg")]
    pub alg: String,
    /// JSON Web Key ID  The Key ID of the key to be created.
    #[serde(rename = "kid")]
    pub kid: String,
    /// JSON Web Key Use  The \"use\" (public key use) parameter identifies the intended use of the public key. The \"use\" parameter is employed to indicate whether a public key is used for encrypting data or verifying the signature on data. Valid values are \"enc\" and \"sig\".
    #[serde(rename = "use")]
    pub _use: String,
}


impl CreateJsonWebKeySet {
    /// Create JSON Web Key Set Request Body
    pub fn new(alg: String, kid: String, _use: String) -> CreateJsonWebKeySet {
        CreateJsonWebKeySet {
                alg,
                kid,
                _use,
        }
    }
}


