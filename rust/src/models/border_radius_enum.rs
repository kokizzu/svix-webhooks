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
        
                /// 
                #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
                pub enum BorderRadiusEnum {
                        #[serde(rename = "none")]
                        None,
                                            #[serde(rename = "lg")]
                        Lg,
                                            #[serde(rename = "md")]
                        Md,
                                            #[serde(rename = "sm")]
                        Sm,
                                            #[serde(rename = "full")]
                        Full,
                    
                }

                impl std::fmt::Display for BorderRadiusEnum {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                        Self::None => write!(f, "none"),
                        Self::Lg => write!(f, "lg"),
                        Self::Md => write!(f, "md"),
                        Self::Sm => write!(f, "sm"),
                        Self::Full => write!(f, "full"),
                }
                }
                }

            impl Default for BorderRadiusEnum {
            fn default() -> BorderRadiusEnum {
                Self::None
            }
            }

