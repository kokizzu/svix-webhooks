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

/// struct for passing parameters to the method [`create_message_attempt_for_endpoint`]
#[derive(Clone, Debug)]
pub struct CreateMessageAttemptForEndpointParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The ep's ID or UID
    pub endpoint_id: String,
    pub message_in: models::MessageIn,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`v1_period_events_public`]
#[derive(Clone, Debug)]
pub struct V1PeriodEventsPublicParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The ep's ID or UID
    pub sink_id: String,
    /// Limit the number of returned items
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,
    /// Filters messages sent with this event type (optional).
    pub event_type: Option<String>,
    /// Filters messages sent with this channel (optional).
    pub channel: Option<String>,
    pub after: Option<String>
}

/// struct for passing parameters to the method [`v1_period_message_period_create`]
#[derive(Clone, Debug)]
pub struct V1PeriodMessagePeriodCreateParams {
    /// The app's ID or UID
    pub app_id: String,
    pub message_in: models::MessageIn,
    /// When `true`, message payloads are included in the response.
    pub with_content: Option<bool>,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`v1_period_message_period_events`]
#[derive(Clone, Debug)]
pub struct V1PeriodMessagePeriodEventsParams {
    /// The app's ID or UID
    pub app_id: String,
    /// Limit the number of returned items
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,
    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>,
    /// Filter response based on the event type.
    pub channels: Option<Vec<String>>,
    pub after: Option<String>
}

/// struct for passing parameters to the method [`v1_period_message_period_events_subscription`]
#[derive(Clone, Debug)]
pub struct V1PeriodMessagePeriodEventsSubscriptionParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The esub's ID or UID
    pub subscription_id: String,
    /// Limit the number of returned items
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,
    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>,
    /// Filter response based on the event type.
    pub channels: Option<Vec<String>>,
    pub after: Option<String>
}

/// struct for passing parameters to the method [`v1_period_message_period_events_subscription_period_create_token`]
#[derive(Clone, Debug)]
pub struct V1PeriodMessagePeriodEventsSubscriptionPeriodCreateTokenParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The esub's ID or UID
    pub subscription_id: String,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`v1_period_message_period_expunge_content`]
#[derive(Clone, Debug)]
pub struct V1PeriodMessagePeriodExpungeContentParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The msg's ID or UID
    pub msg_id: String
}

/// struct for passing parameters to the method [`v1_period_message_period_get`]
#[derive(Clone, Debug)]
pub struct V1PeriodMessagePeriodGetParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The msg's ID or UID
    pub msg_id: String,
    /// When `true` message payloads are included in the response.
    pub with_content: Option<bool>
}

/// struct for passing parameters to the method [`v1_period_message_period_get_raw_payload`]
#[derive(Clone, Debug)]
pub struct V1PeriodMessagePeriodGetRawPayloadParams {
    /// The app's ID or UID
    pub app_id: String,
    /// The msg's ID or UID
    pub msg_id: String
}

/// struct for passing parameters to the method [`v1_period_message_period_list`]
#[derive(Clone, Debug)]
pub struct V1PeriodMessagePeriodListParams {
    /// The app's ID or UID
    pub app_id: String,
    /// Limit the number of returned items
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,
    /// Filter response based on the channel.
    pub channel: Option<String>,
    /// Only include items created before a certain date.
    pub before: Option<String>,
    /// Only include items created after a certain date.
    pub after: Option<String>,
    /// When `true` message payloads are included in the response.
    pub with_content: Option<bool>,
    /// Filter messages matching the provided tag.
    pub tag: Option<String>,
    /// Filter response based on the event type
    pub event_types: Option<Vec<String>>
}


/// struct for typed errors of method [`create_message_attempt_for_endpoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageAttemptForEndpointError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_events_public`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodEventsPublicError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_message_period_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodMessagePeriodCreateError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status413(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_message_period_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodMessagePeriodEventsError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_message_period_events_subscription`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodMessagePeriodEventsSubscriptionError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_message_period_events_subscription_period_create_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodMessagePeriodEventsSubscriptionPeriodCreateTokenError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_message_period_expunge_content`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodMessagePeriodExpungeContentError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_message_period_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodMessagePeriodGetError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_message_period_get_raw_payload`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodMessagePeriodGetRawPayloadError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`v1_period_message_period_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum V1PeriodMessagePeriodListError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}


/// Creates and sends a message to the specified endpoint.  The message attempt and response from the endpoint is returned.
pub async fn create_message_attempt_for_endpoint(configuration: &Configuration, params: CreateMessageAttemptForEndpointParams) -> Result<models::MessageAttemptOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let endpoint_id = params.endpoint_id;
    let message_in = params.message_in;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/app/{app_id}/endpoint/{endpoint_id}/msg/test-attempt".to_string())
    ;
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    req = req.with_path_param("endpoint_id".to_string(), endpoint_id.to_string());
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key".to_string(), param_value.to_string());
    }
    req = req.with_body_param(message_in);

    req.execute(configuration).await
}

/// Reads the stream of created messages for an application, filtered on the Sink's event types and Channels.
pub async fn v1_period_events_public(configuration: &Configuration, params: V1PeriodEventsPublicParams) -> Result<models::MessageEventsOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let sink_id = params.sink_id;
    let limit = params.limit;
    let iterator = params.iterator;
    let event_type = params.event_type;
    let channel = params.channel;
    let after = params.after;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::GET, "/api/v1/app/{app_id}/poller/{sink_id}".to_string())
    ;
    if let Some(ref s) = limit {
        let query_value = s.to_string();
        req = req.with_query_param("limit".to_string(), query_value);
    }
    if let Some(ref s) = iterator {
        let query_value = s.to_string();
        req = req.with_query_param("iterator".to_string(), query_value);
    }
    if let Some(ref s) = event_type {
        let query_value = s.to_string();
        req = req.with_query_param("event_type".to_string(), query_value);
    }
    if let Some(ref s) = channel {
        let query_value = s.to_string();
        req = req.with_query_param("channel".to_string(), query_value);
    }
    if let Some(ref s) = after {
        let query_value = s.to_string();
        req = req.with_query_param("after".to_string(), query_value);
    }
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    req = req.with_path_param("sink_id".to_string(), sink_id.to_string());

    req.execute(configuration).await
}

/// Creates a new message and dispatches it to all of the application's endpoints.  The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made. If a message with the same `eventId` already exists for the application, a 409 conflict error will be returned.  The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types. Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.  The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
pub async fn v1_period_message_period_create(configuration: &Configuration, params: V1PeriodMessagePeriodCreateParams) -> Result<models::MessageOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let message_in = params.message_in;
    let with_content = params.with_content;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/app/{app_id}/msg".to_string())
    ;
    if let Some(ref s) = with_content {
        let query_value = s.to_string();
        req = req.with_query_param("with_content".to_string(), query_value);
    }
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key".to_string(), param_value.to_string());
    }
    req = req.with_body_param(message_in);

    req.execute(configuration).await
}

/// Reads the stream of created messages for an application.
pub async fn v1_period_message_period_events(configuration: &Configuration, params: V1PeriodMessagePeriodEventsParams) -> Result<models::MessageEventsOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let limit = params.limit;
    let iterator = params.iterator;
    let event_types = params.event_types;
    let channels = params.channels;
    let after = params.after;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::GET, "/api/v1/app/{app_id}/events".to_string())
    ;
    if let Some(ref s) = limit {
        let query_value = s.to_string();
        req = req.with_query_param("limit".to_string(), query_value);
    }
    if let Some(ref s) = iterator {
        let query_value = s.to_string();
        req = req.with_query_param("iterator".to_string(), query_value);
    }
    if let Some(ref s) = event_types {
        let query_value = s.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
        req = req.with_query_param("event_types".to_string(), query_value);
    }
    if let Some(ref s) = channels {
        let query_value = s.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
        req = req.with_query_param("channels".to_string(), query_value);
    }
    if let Some(ref s) = after {
        let query_value = s.to_string();
        req = req.with_query_param("after".to_string(), query_value);
    }
    req = req.with_path_param("app_id".to_string(), app_id.to_string());

    req.execute(configuration).await
}

/// Reads the stream of created messages for an application, but using server-managed iterator tracking.
pub async fn v1_period_message_period_events_subscription(configuration: &Configuration, params: V1PeriodMessagePeriodEventsSubscriptionParams) -> Result<models::MessageEventsOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let subscription_id = params.subscription_id;
    let limit = params.limit;
    let iterator = params.iterator;
    let event_types = params.event_types;
    let channels = params.channels;
    let after = params.after;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::GET, "/api/v1/app/{app_id}/events/subscription/{subscription_id}".to_string())
    ;
    if let Some(ref s) = limit {
        let query_value = s.to_string();
        req = req.with_query_param("limit".to_string(), query_value);
    }
    if let Some(ref s) = iterator {
        let query_value = s.to_string();
        req = req.with_query_param("iterator".to_string(), query_value);
    }
    if let Some(ref s) = event_types {
        let query_value = s.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
        req = req.with_query_param("event_types".to_string(), query_value);
    }
    if let Some(ref s) = channels {
        let query_value = s.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
        req = req.with_query_param("channels".to_string(), query_value);
    }
    if let Some(ref s) = after {
        let query_value = s.to_string();
        req = req.with_query_param("after".to_string(), query_value);
    }
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    req = req.with_path_param("subscription_id".to_string(), subscription_id.to_string());

    req.execute(configuration).await
}

/// Creates an auth token that can be used with the `v1.message.events-subscription` endpoint.
pub async fn v1_period_message_period_events_subscription_period_create_token(configuration: &Configuration, params: V1PeriodMessagePeriodEventsSubscriptionPeriodCreateTokenParams) -> Result<models::MessageSubscriberAuthTokenOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let subscription_id = params.subscription_id;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/app/{app_id}/events/subscription/{subscription_id}/create-token".to_string())
    ;
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    req = req.with_path_param("subscription_id".to_string(), subscription_id.to_string());
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key".to_string(), param_value.to_string());
    }

    req.execute(configuration).await
}

/// Delete the given message's payload.  Useful in cases when a message was accidentally sent with sensitive content. The message can't be replayed or resent once its payload has been deleted or expired.
pub async fn v1_period_message_period_expunge_content(configuration: &Configuration, params: V1PeriodMessagePeriodExpungeContentParams) -> Result<(), Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let msg_id = params.msg_id;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::DELETE, "/api/v1/app/{app_id}/msg/{msg_id}/content".to_string())
    ;
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    req = req.with_path_param("msg_id".to_string(), msg_id.to_string());
    req = req.returns_nothing();

    req.execute(configuration).await
}

/// Get a message by its ID or eventID.
pub async fn v1_period_message_period_get(configuration: &Configuration, params: V1PeriodMessagePeriodGetParams) -> Result<models::MessageOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let msg_id = params.msg_id;
    let with_content = params.with_content;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::GET, "/api/v1/app/{app_id}/msg/{msg_id}".to_string())
    ;
    if let Some(ref s) = with_content {
        let query_value = s.to_string();
        req = req.with_query_param("with_content".to_string(), query_value);
    }
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    req = req.with_path_param("msg_id".to_string(), msg_id.to_string());

    req.execute(configuration).await
}

/// Get a message raw payload by its ID or eventID.
pub async fn v1_period_message_period_get_raw_payload(configuration: &Configuration, params: V1PeriodMessagePeriodGetRawPayloadParams) -> Result<models::MessageRawPayloadOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let msg_id = params.msg_id;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::GET, "/api/v1/app/{app_id}/msg/{msg_id}/raw".to_string())
    ;
    req = req.with_path_param("app_id".to_string(), app_id.to_string());
    req = req.with_path_param("msg_id".to_string(), msg_id.to_string());

    req.execute(configuration).await
}

/// List all of the application's messages.  The `before` and `after` parameters let you filter all items created before or after a certain date. These can be used alongside an iterator to paginate over results within a certain window.  Note that by default this endpoint is limited to retrieving 90 days' worth of data relative to now or, if an iterator is provided, 90 days before/after the time indicated by the iterator ID. If you require data beyond those time ranges, you will need to explicitly set the `before` or `after` parameter as appropriate. 
pub async fn v1_period_message_period_list(configuration: &Configuration, params: V1PeriodMessagePeriodListParams) -> Result<models::ListResponseMessageOut, Error> {
    // unbox the parameters
    let app_id = params.app_id;
    let limit = params.limit;
    let iterator = params.iterator;
    let channel = params.channel;
    let before = params.before;
    let after = params.after;
    let with_content = params.with_content;
    let tag = params.tag;
    let event_types = params.event_types;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::GET, "/api/v1/app/{app_id}/msg".to_string())
    ;
    if let Some(ref s) = limit {
        let query_value = s.to_string();
        req = req.with_query_param("limit".to_string(), query_value);
    }
    if let Some(ref s) = iterator {
        let query_value = s.to_string();
        req = req.with_query_param("iterator".to_string(), query_value);
    }
    if let Some(ref s) = channel {
        let query_value = s.to_string();
        req = req.with_query_param("channel".to_string(), query_value);
    }
    if let Some(ref s) = before {
        let query_value = s.to_string();
        req = req.with_query_param("before".to_string(), query_value);
    }
    if let Some(ref s) = after {
        let query_value = s.to_string();
        req = req.with_query_param("after".to_string(), query_value);
    }
    if let Some(ref s) = with_content {
        let query_value = s.to_string();
        req = req.with_query_param("with_content".to_string(), query_value);
    }
    if let Some(ref s) = tag {
        let query_value = s.to_string();
        req = req.with_query_param("tag".to_string(), query_value);
    }
    if let Some(ref s) = event_types {
        let query_value = s.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",");
        req = req.with_query_param("event_types".to_string(), query_value);
    }
    req = req.with_path_param("app_id".to_string(), app_id.to_string());

    req.execute(configuration).await
}

