// This file is @generated
package com.svix.kotlin.models

import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

@Serializable data class ReplayIn(val since: Instant, val until: Instant? = null)
