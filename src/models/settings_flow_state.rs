/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// SettingsFlowState : show_form: No user data has been collected, or it is invalid, and thus the form should be shown. success: Indicates that the settings flow has been updated successfully with the provided data. Done will stay true when repeatedly checking. If set to true, done will revert back to false only when a flow with invalid (e.g. \"please use a valid phone number\") data was sent.

/// show_form: No user data has been collected, or it is invalid, and thus the form should be shown. success: Indicates that the settings flow has been updated successfully with the provided data. Done will stay true when repeatedly checking. If set to true, done will revert back to false only when a flow with invalid (e.g. \"please use a valid phone number\") data was sent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SettingsFlowState {
    #[serde(rename = "show_form")]
    ShowForm,
    #[serde(rename = "success")]
    Success,

}

impl ToString for SettingsFlowState {
    fn to_string(&self) -> String {
        match self {
            Self::ShowForm => String::from("show_form"),
            Self::Success => String::from("success"),
        }
    }
}




