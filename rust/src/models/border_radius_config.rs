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
                pub struct BorderRadiusConfig {
                        #[serde(rename = "button", skip_serializing_if = "Option::is_none")]
                        pub button: Option<models::BorderRadiusEnum>,
                        #[serde(rename = "card", skip_serializing_if = "Option::is_none")]
                        pub card: Option<models::BorderRadiusEnum>,
                        #[serde(rename = "input", skip_serializing_if = "Option::is_none")]
                        pub input: Option<models::BorderRadiusEnum>,
                    }

                    impl BorderRadiusConfig {
                    pub fn new() -> BorderRadiusConfig {
                BorderRadiusConfig {
                    button: None,
                    card: None,
                    input: None,
                    }
                    }
                    }

