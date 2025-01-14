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
                pub struct ReplayIn {
                        #[serde(rename = "since")]
                        pub since: String,
                        #[serde(rename = "until", skip_serializing_if = "Option::is_none")]
                        pub until: Option<String>,
                    }

                    impl ReplayIn {
                    pub fn new(since: String) -> ReplayIn {
                ReplayIn {
                    since,
                    until: None,
                    }
                    }
                    }

