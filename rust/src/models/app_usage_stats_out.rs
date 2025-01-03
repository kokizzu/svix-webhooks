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
                pub struct AppUsageStatsOut {
                        #[serde(rename = "id")]
                        pub id: String,
                        #[serde(rename = "status")]
                        pub status: models::BackgroundTaskStatus,
                        #[serde(rename = "task")]
                        pub task: models::BackgroundTaskType,
                        /// Any app IDs or UIDs received in the request that weren't found.  Stats will be produced for all the others.
                        #[serde(rename = "unresolvedAppIds")]
                        pub unresolved_app_ids: Vec<String>,
                    }

                    impl AppUsageStatsOut {
                    pub fn new(id: String, status: models::BackgroundTaskStatus, task: models::BackgroundTaskType, unresolved_app_ids: Vec<String>) -> AppUsageStatsOut {
                AppUsageStatsOut {
                    id,
                    status,
                    task,
                    unresolved_app_ids,
                    }
                    }
                    }
