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
pub struct EnvironmentSettingsOut {
    #[serde(rename = "colorPaletteDark", skip_serializing_if = "Option::is_none")]
    pub color_palette_dark: Option<Box<models::CustomColorPalette>>,
    #[serde(rename = "colorPaletteLight", skip_serializing_if = "Option::is_none")]
    pub color_palette_light: Option<Box<models::CustomColorPalette>>,
    #[serde(rename = "customColor", skip_serializing_if = "Option::is_none")]
    pub custom_color: Option<String>,
    #[serde(rename = "customFontFamily", skip_serializing_if = "Option::is_none")]
    pub custom_font_family: Option<String>,
    #[serde(
        rename = "customFontFamilyUrl",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_font_family_url: Option<String>,
    #[serde(rename = "customLogoUrl", skip_serializing_if = "Option::is_none")]
    pub custom_logo_url: Option<String>,
    #[serde(
        rename = "customStringsOverride",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_strings_override: Option<Box<models::CustomStringsOverride>>,
    #[serde(
        rename = "customThemeOverride",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_theme_override: Option<Box<models::CustomThemeOverride>>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "enableChannels", skip_serializing_if = "Option::is_none")]
    pub enable_channels: Option<bool>,
    #[serde(
        rename = "enableIntegrationManagement",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_integration_management: Option<bool>,
    #[serde(
        rename = "enableMessageStream",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_message_stream: Option<bool>,
    #[serde(rename = "enableMessageTags", skip_serializing_if = "Option::is_none")]
    pub enable_message_tags: Option<bool>,
    #[serde(
        rename = "enableTransformations",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_transformations: Option<bool>,
    #[serde(rename = "showUseSvixPlay", skip_serializing_if = "Option::is_none")]
    pub show_use_svix_play: Option<bool>,
    #[serde(
        rename = "wipeSuccessfulPayload",
        skip_serializing_if = "Option::is_none"
    )]
    pub wipe_successful_payload: Option<bool>,
}

impl EnvironmentSettingsOut {
    pub fn new() -> EnvironmentSettingsOut {
        EnvironmentSettingsOut {
            color_palette_dark: None,
            color_palette_light: None,
            custom_color: None,
            custom_font_family: None,
            custom_font_family_url: None,
            custom_logo_url: None,
            custom_strings_override: None,
            custom_theme_override: None,
            display_name: None,
            enable_channels: None,
            enable_integration_management: None,
            enable_message_stream: None,
            enable_message_tags: None,
            enable_transformations: None,
            show_use_svix_play: None,
            wipe_successful_payload: None,
        }
    }
}
