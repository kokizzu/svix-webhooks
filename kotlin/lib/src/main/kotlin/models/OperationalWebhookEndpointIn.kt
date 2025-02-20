// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class OperationalWebhookEndpointIn(
    val description: String? = null,
    val disabled: Boolean? = null,
    val filterTypes: Set<String>? = null,
    val metadata: Map<String, String>? = null,
    val rateLimit: UShort? = null,
    val secret: String? = null,
    val uid: String? = null,
    val url: String,
)
