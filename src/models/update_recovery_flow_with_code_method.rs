/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.2.5
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// UpdateRecoveryFlowWithCodeMethod : Update Recovery Flow with Code Method



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateRecoveryFlowWithCodeMethod {
    /// Code from the recovery email  If you want to submit a code, use this field, but make sure to _not_ include the email field, as well.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Sending the anti-csrf token is only required for browser login flows.
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// The email address of the account to recover  If the email belongs to a valid account, a recovery email will be sent.  If you want to notify the email address if the account does not exist, see the [notify_unknown_recipients flag](https://www.ory.sh/docs/kratos/self-service/flows/account-recovery-password-reset#attempted-recovery-notifications)  If a code was already sent, including this field in the payload will invalidate the sent code and re-send a new code.  format: email
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Method is the method that should be used for this recovery flow  Allowed values are `link` and `code`. link RecoveryStrategyLink code RecoveryStrategyCode
    #[serde(rename = "method")]
    pub method: MethodEnum,
}


impl UpdateRecoveryFlowWithCodeMethod {
    /// Update Recovery Flow with Code Method
    pub fn new(method: MethodEnum) -> UpdateRecoveryFlowWithCodeMethod {
        UpdateRecoveryFlowWithCodeMethod {
                code: None,
                csrf_token: None,
                email: None,
                method,
        }
    }
}

/// Method is the method that should be used for this recovery flow  Allowed values are `link` and `code`. link RecoveryStrategyLink code RecoveryStrategyCode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MethodEnum {
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "code")]
    Code,
}

