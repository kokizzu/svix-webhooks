/*
 * Svix API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use crate::Configuration;
use crate::models;
use crate::error::Error;
#[allow(unused_imports)]
use crate::models::*;

/// struct for passing parameters to the method [`v1_period_authentication_period_app_portal_access`]
#[derive(Clone, Debug)]
pub struct V1PeriodAuthenticationPeriodAppPortalAccessParams {
    /// The app's ID or UID
    pub app_id: String,
    pub app_portal_access_in: models::AppPortalAccessIn,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`v1_period_authentication_period_create_message_token`]
#[derive(Clone, Debug)]
pub struct V1PeriodAuthenticationPeriodCreateMessageTokenParams {
    /// The app's ID or UID
    pub app_id: String,
    pub create_token_in: models::CreateTokenIn,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`v1_period_authentication_period_dashboard_access`]
#[derive(Clone, Debug)]
pub struct V1PeriodAuthenticationPeriodDashboardAccessParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`v1_period_authentication_period_exchange_one_time_token`]
#[derive(Clone, Debug)]
pub struct V1PeriodAuthenticationPeriodExchangeOneTimeTokenParams {
    pub one_time_token_in: models::OneTimeTokenIn,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`v1_period_authentication_period_expire_all`]
#[derive(Clone, Debug)]
pub struct V1PeriodAuthenticationPeriodExpireAllParams {
    /// The app's ID or UID
    pub app_id: String,
    pub application_token_expire_in: models::ApplicationTokenExpireIn,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`v1_period_authentication_period_get_poller_token`]
#[derive(Clone, Debug)]
pub struct V1PeriodAuthenticationPeriodGetPollerTokenParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The ep's ID or UID
    pub endpoint_id: String
}

/// struct for passing parameters to the method [`v1_period_authentication_period_logout`]
#[derive(Clone, Debug)]
pub struct V1PeriodAuthenticationPeriodLogoutParams {
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`v1_period_authentication_period_rotate_poller_token`]
#[derive(Clone, Debug)]
pub struct V1PeriodAuthenticationPeriodRotatePollerTokenParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The ep's ID or UID
    pub endpoint_id: String,
    pub rotate_poller_token_in: models::RotatePollerTokenIn,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}


/// struct for typed errors of method [`v1_period_authentication_period_app_portal_access`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodAuthenticationPeriodAppPortalAccessError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_authentication_period_create_message_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodAuthenticationPeriodCreateMessageTokenError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_authentication_period_dashboard_access`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodAuthenticationPeriodDashboardAccessError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_authentication_period_exchange_one_time_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodAuthenticationPeriodExchangeOneTimeTokenError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_authentication_period_expire_all`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodAuthenticationPeriodExpireAllError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_authentication_period_get_poller_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodAuthenticationPeriodGetPollerTokenError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_authentication_period_logout`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodAuthenticationPeriodLogoutError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_authentication_period_rotate_poller_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodAuthenticationPeriodRotatePollerTokenError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}


/// Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
pub async fn v1_period_authentication_period_app_portal_access(configuration: &Configuration, params: V1PeriodAuthenticationPeriodAppPortalAccessParams) -> Result<models::AppPortalAccessOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let app_portal_access_in = params.app_portal_access_in;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/auth/app-portal-access/{app_id}".to_string())
    ;
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key".to_string(), param_value.to_string());
    }
    req = req.with_body_param(app_portal_access_in);

    req.execute(configuration).await
}

/// Create a new access token that only allows creating messages inside this application.
pub async fn v1_period_authentication_period_create_message_token(configuration: &Configuration, params: V1PeriodAuthenticationPeriodCreateMessageTokenParams) -> Result<models::AuthTokenOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let create_token_in = params.create_token_in;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/auth/app/{app_id}/create-message-token".to_string())
    ;
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key".to_string(), param_value.to_string());
    }
    req = req.with_body_param(create_token_in);

    req.execute(configuration).await
}

/// DEPRECATED: Please use `app-portal-access` instead.  Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
pub async fn v1_period_authentication_period_dashboard_access(configuration: &Configuration, params: V1PeriodAuthenticationPeriodDashboardAccessParams) -> Result<models::DashboardAccessOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/auth/dashboard-access/{app_id}".to_string())
    ;
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key".to_string(), param_value.to_string());
    }

    req.execute(configuration).await
}

/// This is a one time token.
pub async fn v1_period_authentication_period_exchange_one_time_token(configuration: &Configuration, params: V1PeriodAuthenticationPeriodExchangeOneTimeTokenParams) -> Result<models::OneTimeTokenOut, Error> {
    // unbox the parameters
    let one_time_token_in = params.one_time_token_in;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/auth/one-time-token".to_string())
    ;
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key".to_string(), param_value.to_string());
    }
    req = req.with_body_param(one_time_token_in);

    req.execute(configuration).await
}

/// Expire all of the tokens associated with a specific application.
pub async fn v1_period_authentication_period_expire_all(configuration: &Configuration, params: V1PeriodAuthenticationPeriodExpireAllParams) -> Result<(), Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let application_token_expire_in = params.application_token_expire_in;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/auth/app/{app_id}/expire-all".to_string())
    ;
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key".to_string(), param_value.to_string());
    }
    req = req.with_body_param(application_token_expire_in);
    req = req.returns_nothing();

    req.execute(configuration).await
}

/// Get the current auth token for the poller.
pub async fn v1_period_authentication_period_get_poller_token(configuration: &Configuration, params: V1PeriodAuthenticationPeriodGetPollerTokenParams) -> Result<models::AuthTokenOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let endpoint_id = params.endpoint_id;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::GET, "/api/v1/auth/app/{app_id}/poller/{endpoint_id}/token".to_string())
    ;
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    req = req.with_path_param("endpoint_id".to_string(), endpoint_id.to_string());

    req.execute(configuration).await
}

/// Logout an app token.  Trying to log out other tokens will fail.
pub async fn v1_period_authentication_period_logout(configuration: &Configuration, params: V1PeriodAuthenticationPeriodLogoutParams) -> Result<(), Error> {
    // unbox the parameters
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/auth/logout".to_string())
    ;
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key".to_string(), param_value.to_string());
    }
    req = req.returns_nothing();

    req.execute(configuration).await
}

/// Create a new auth token that can for the poller API.
pub async fn v1_period_authentication_period_rotate_poller_token(configuration: &Configuration, params: V1PeriodAuthenticationPeriodRotatePollerTokenParams) -> Result<models::AuthTokenOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let endpoint_id = params.endpoint_id;
    let rotate_poller_token_in = params.rotate_poller_token_in;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/auth/app/{app_id}/poller/{endpoint_id}/token/rotate".to_string())
    ;
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    req = req.with_path_param("endpoint_id".to_string(), endpoint_id.to_string());
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key".to_string(), param_value.to_string());
    }
    req = req.with_body_param(rotate_poller_token_in);

    req.execute(configuration).await
}

