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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventTypeIn {
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "featureFlag", skip_serializing_if = "Option::is_none")]
    pub feature_flag: Option<String>,
    /// The event type group's name
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// The event type's name
    #[serde(rename = "name")]
    pub name: String,
    /// The schema for the event type for a specific version as a JSON schema.
    #[serde(rename = "schemas", skip_serializing_if = "Option::is_none")]
    pub schemas: Option<serde_json::Value>,
}

impl EventTypeIn {
    pub fn new(description: String, name: String) -> EventTypeIn {
        EventTypeIn {
            archived: None,
            deprecated: None,
            description,
            feature_flag: None,
            group_name: None,
            name,
            schemas: None,
        }
    }
}
