/*
 * Svix API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * API version: 1.1.1
 */

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
)

// EndpointMtlsConfigIn struct for EndpointMtlsConfigIn
type EndpointMtlsConfigIn struct {
	// A PEM encoded private key and X509 certificate to identify the webhook sender.
	Identity string `json:"identity"`
	// A PEM encoded X509 certificate used to verify the webhook receiver's certificate.
	ServerCaCert NullableString `json:"serverCaCert,omitempty"`
}

// NewEndpointMtlsConfigIn instantiates a new EndpointMtlsConfigIn object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewEndpointMtlsConfigIn(identity string) *EndpointMtlsConfigIn {
	this := EndpointMtlsConfigIn{}
	this.Identity = identity
	return &this
}

// NewEndpointMtlsConfigInWithDefaults instantiates a new EndpointMtlsConfigIn object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewEndpointMtlsConfigInWithDefaults() *EndpointMtlsConfigIn {
	this := EndpointMtlsConfigIn{}
	return &this
}

// GetIdentity returns the Identity field value
func (o *EndpointMtlsConfigIn) GetIdentity() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Identity
}

// GetIdentityOk returns a tuple with the Identity field value
// and a boolean to check if the value has been set.
func (o *EndpointMtlsConfigIn) GetIdentityOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.Identity, true
}

// SetIdentity sets field value
func (o *EndpointMtlsConfigIn) SetIdentity(v string) {
	o.Identity = v
}

// GetServerCaCert returns the ServerCaCert field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *EndpointMtlsConfigIn) GetServerCaCert() string {
	if o == nil || o.ServerCaCert.Get() == nil {
		var ret string
		return ret
	}
	return *o.ServerCaCert.Get()
}

// GetServerCaCertOk returns a tuple with the ServerCaCert field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *EndpointMtlsConfigIn) GetServerCaCertOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return o.ServerCaCert.Get(), o.ServerCaCert.IsSet()
}

// HasServerCaCert returns a boolean if a field has been set.
func (o *EndpointMtlsConfigIn) HasServerCaCert() bool {
	if o != nil && o.ServerCaCert.IsSet() {
		return true
	}

	return false
}

// SetServerCaCert gets a reference to the given NullableString and assigns it to the ServerCaCert field.
func (o *EndpointMtlsConfigIn) SetServerCaCert(v string) {
	o.ServerCaCert.Set(&v)
}
// SetServerCaCertNil sets the value for ServerCaCert to be an explicit nil
func (o *EndpointMtlsConfigIn) SetServerCaCertNil() {
	o.ServerCaCert.Set(nil)
}

// UnsetServerCaCert ensures that no value is present for ServerCaCert, not even an explicit nil
func (o *EndpointMtlsConfigIn) UnsetServerCaCert() {
	o.ServerCaCert.Unset()
}

func (o EndpointMtlsConfigIn) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if true {
		toSerialize["identity"] = o.Identity
	}
	if o.ServerCaCert.IsSet() {
		toSerialize["serverCaCert"] = o.ServerCaCert.Get()
	}
	return json.Marshal(toSerialize)
}

type NullableEndpointMtlsConfigIn struct {
	value *EndpointMtlsConfigIn
	isSet bool
}

func (v NullableEndpointMtlsConfigIn) Get() *EndpointMtlsConfigIn {
	return v.value
}

func (v *NullableEndpointMtlsConfigIn) Set(val *EndpointMtlsConfigIn) {
	v.value = val
	v.isSet = true
}

func (v NullableEndpointMtlsConfigIn) IsSet() bool {
	return v.isSet
}

func (v *NullableEndpointMtlsConfigIn) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableEndpointMtlsConfigIn(val *EndpointMtlsConfigIn) *NullableEndpointMtlsConfigIn {
	return &NullableEndpointMtlsConfigIn{value: val, isSet: true}
}

func (v NullableEndpointMtlsConfigIn) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableEndpointMtlsConfigIn) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


