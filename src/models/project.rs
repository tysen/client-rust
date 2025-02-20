/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "cors_admin")]
    pub cors_admin: Box<crate::models::ProjectCors>,
    #[serde(rename = "cors_public")]
    pub cors_public: Box<crate::models::ProjectCors>,
    /// The project's ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the project.
    #[serde(rename = "name")]
    pub name: String,
    /// The configuration revision ID.
    #[serde(rename = "revision_id")]
    pub revision_id: String,
    #[serde(rename = "services")]
    pub services: Box<crate::models::ProjectServices>,
    /// The project's slug
    #[serde(rename = "slug")]
    pub slug: String,
    /// The state of the project. running Running halted Halted deleted Deleted
    #[serde(rename = "state")]
    pub state: StateEnum,
}


impl Project {
    pub fn new(cors_admin: crate::models::ProjectCors, cors_public: crate::models::ProjectCors, id: String, name: String, revision_id: String, services: crate::models::ProjectServices, slug: String, state: StateEnum) -> Project {
        Project {
                cors_admin: Box::new(cors_admin),
                cors_public: Box::new(cors_public),
                id,
                name,
                revision_id,
                services: Box::new(services),
                slug,
                state,
        }
    }
}

/// The state of the project. running Running halted Halted deleted Deleted
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StateEnum {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "halted")]
    Halted,
    #[serde(rename = "deleted")]
    Deleted,
}

