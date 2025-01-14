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
        
            /// RedshiftConfig : Configuration parameters for defining a Redshift sink.
                #[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
                pub struct RedshiftConfig {
                        #[serde(rename = "accessKeyId")]
                        pub access_key_id: String,
                        #[serde(rename = "clusterIdentifier")]
                        pub cluster_identifier: String,
                        /// Database name.  Only required if not using transformations.
                        #[serde(rename = "dbName", skip_serializing_if = "Option::is_none")]
                        pub db_name: Option<String>,
                        #[serde(rename = "dbUser")]
                        pub db_user: String,
                        #[serde(rename = "region")]
                        pub region: String,
                        /// Schema name.  Only used if not using transformations.
                        #[serde(rename = "schemaName", skip_serializing_if = "Option::is_none")]
                        pub schema_name: Option<String>,
                        #[serde(rename = "secretAccessKey")]
                        pub secret_access_key: String,
                        /// Table name.  Only required if not using transformations.
                        #[serde(rename = "tableName", skip_serializing_if = "Option::is_none")]
                        pub table_name: Option<String>,
                    }

                    impl RedshiftConfig {
                        /// Configuration parameters for defining a Redshift sink.
                    pub fn new(access_key_id: String, cluster_identifier: String, db_user: String, region: String, secret_access_key: String) -> RedshiftConfig {
                RedshiftConfig {
                    access_key_id,
                    cluster_identifier,
                    db_name: None,
                    db_user,
                    region,
                    schema_name: None,
                    secret_access_key,
                    table_name: None,
                    }
                    }
                    }

