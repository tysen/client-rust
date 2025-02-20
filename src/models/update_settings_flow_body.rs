/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UpdateSettingsFlowBody : Update Settings Flow Request Body


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum UpdateSettingsFlowBody {
    #[serde(rename="lookup_secret")]
    UpdateSettingsFlowWithLookupMethod {
        /// CSRFToken is the anti-CSRF token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// If set to true will save the regenerated lookup secrets
        #[serde(rename = "lookup_secret_confirm", skip_serializing_if = "Option::is_none")]
        // false, false, , bool, false
        lookup_secret_confirm: Option<bool>,
        /// Disables this method if true.
        #[serde(rename = "lookup_secret_disable", skip_serializing_if = "Option::is_none")]
        // false, false, , bool, false
        lookup_secret_disable: Option<bool>,
        /// If set to true will regenerate the lookup secrets
        #[serde(rename = "lookup_secret_regenerate", skip_serializing_if = "Option::is_none")]
        // false, false, , bool, false
        lookup_secret_regenerate: Option<bool>,
        /// If set to true will reveal the lookup secrets
        #[serde(rename = "lookup_secret_reveal", skip_serializing_if = "Option::is_none")]
        // false, false, , bool, false
        lookup_secret_reveal: Option<bool>,
    },
    #[serde(rename="oidc")]
    UpdateSettingsFlowWithOidcMethod {
        /// Flow ID is the flow's ID.  in: query
        #[serde(rename = "flow", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        flow: Option<String>,
        /// Link this provider  Either this or `unlink` must be set.  type: string in: body
        #[serde(rename = "link", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        link: Option<String>,
        /// The identity's traits  in: body
        #[serde(rename = "traits", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        traits: Option<serde_json::Value>,
        /// Unlink this provider  Either this or `link` must be set.  type: string in: body
        #[serde(rename = "unlink", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        unlink: Option<String>,
        /// UpstreamParameters are the parameters that are passed to the upstream identity provider.  These parameters are optional and depend on what the upstream identity provider supports. Supported parameters are: `login_hint` (string): The `login_hint` parameter suppresses the account chooser and either pre-fills the email box on the sign-in form, or selects the proper session. `hd` (string): The `hd` parameter limits the login/registration process to a Google Organization, e.g. `mycollege.edu`. `prompt` (string): The `prompt` specifies whether the Authorization Server prompts the End-User for reauthentication and consent, e.g. `select_account`.
        #[serde(rename = "upstream_parameters", skip_serializing_if = "Option::is_none")]
        // false, false, , serde_json::Value, false
        upstream_parameters: Option<serde_json::Value>,
    },
    #[serde(rename="password")]
    UpdateSettingsFlowWithPasswordMethod {
        /// CSRFToken is the anti-CSRF token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// Password is the updated password
        #[serde(rename = "password")]
        // true, false, , String, false
        password: String,
    },
    #[serde(rename="profile")]
    UpdateSettingsFlowWithProfileMethod {
        /// The Anti-CSRF Token  This token is only required when performing browser flows.
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// Traits  The identity's traits.
        #[serde(rename = "traits")]
        // true, false, , serde_json::Value, false
        traits: serde_json::Value,
    },
    #[serde(rename="totp")]
    UpdateSettingsFlowWithTotpMethod {
        /// CSRFToken is the anti-CSRF token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// ValidationTOTP must contain a valid TOTP based on the
        #[serde(rename = "totp_code", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        totp_code: Option<String>,
        /// UnlinkTOTP if true will remove the TOTP pairing, effectively removing the credential. This can be used to set up a new TOTP device.
        #[serde(rename = "totp_unlink", skip_serializing_if = "Option::is_none")]
        // false, false, , bool, false
        totp_unlink: Option<bool>,
    },
    #[serde(rename="webauthn")]
    UpdateSettingsFlowWithWebAuthnMethod {
        /// CSRFToken is the anti-CSRF token
        #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        csrf_token: Option<String>,
        /// Register a WebAuthn Security Key  It is expected that the JSON returned by the WebAuthn registration process is included here.
        #[serde(rename = "webauthn_register", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        webauthn_register: Option<String>,
        /// Name of the WebAuthn Security Key to be Added  A human-readable name for the security key which will be added.
        #[serde(rename = "webauthn_register_displayname", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        webauthn_register_displayname: Option<String>,
        /// Remove a WebAuthn Security Key  This must contain the ID of the WebAuthN connection.
        #[serde(rename = "webauthn_remove", skip_serializing_if = "Option::is_none")]
        // false, false, , String, false
        webauthn_remove: Option<String>,
    },
}





