/*
 * Svix API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use crate::models;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// EndpointDeletedEvent : Sent when an endpoint is deleted.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointDeletedEvent {
    #[serde(rename = "data")]
    pub data: Box<models::EndpointDeletedEventData>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl EndpointDeletedEvent {
    /// Sent when an endpoint is deleted.
    pub fn new(data: models::EndpointDeletedEventData, r#type: Type) -> EndpointDeletedEvent {
        EndpointDeletedEvent {
            data: Box::new(data),
            r#type,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "endpoint.deleted")]
    EndpointPeriodDeleted,
}

impl Default for Type {
    fn default() -> Type {
        Self::EndpointPeriodDeleted
    }
}
