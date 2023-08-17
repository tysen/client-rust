/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.49
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateInviteResponse {
    /// A list of all invites for this resource
    #[serde(rename = "all_invites")]
    pub all_invites: Vec<crate::models::MemberInvite>,
    #[serde(rename = "created_invite")]
    pub created_invite: Box<crate::models::MemberInvite>,
}


impl CreateInviteResponse {
    pub fn new(all_invites: Vec<crate::models::MemberInvite>, created_invite: crate::models::MemberInvite) -> CreateInviteResponse {
        CreateInviteResponse {
                all_invites,
                created_invite: Box::new(created_invite),
        }
    }
}


