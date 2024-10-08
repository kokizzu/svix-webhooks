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

// StreamSinkInOneOf3 struct for StreamSinkInOneOf3
type StreamSinkInOneOf3 struct {
	Config S3Config `json:"config"`
	Type string `json:"type"`
}

// NewStreamSinkInOneOf3 instantiates a new StreamSinkInOneOf3 object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewStreamSinkInOneOf3(config S3Config, type_ string) *StreamSinkInOneOf3 {
	this := StreamSinkInOneOf3{}
	this.Config = config
	this.Type = type_
	return &this
}

// NewStreamSinkInOneOf3WithDefaults instantiates a new StreamSinkInOneOf3 object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewStreamSinkInOneOf3WithDefaults() *StreamSinkInOneOf3 {
	this := StreamSinkInOneOf3{}
	return &this
}

// GetConfig returns the Config field value
func (o *StreamSinkInOneOf3) GetConfig() S3Config {
	if o == nil {
		var ret S3Config
		return ret
	}

	return o.Config
}

// GetConfigOk returns a tuple with the Config field value
// and a boolean to check if the value has been set.
func (o *StreamSinkInOneOf3) GetConfigOk() (*S3Config, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.Config, true
}

// SetConfig sets field value
func (o *StreamSinkInOneOf3) SetConfig(v S3Config) {
	o.Config = v
}

// GetType returns the Type field value
func (o *StreamSinkInOneOf3) GetType() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Type
}

// GetTypeOk returns a tuple with the Type field value
// and a boolean to check if the value has been set.
func (o *StreamSinkInOneOf3) GetTypeOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.Type, true
}

// SetType sets field value
func (o *StreamSinkInOneOf3) SetType(v string) {
	o.Type = v
}

func (o StreamSinkInOneOf3) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if true {
		toSerialize["config"] = o.Config
	}
	if true {
		toSerialize["type"] = o.Type
	}
	return json.Marshal(toSerialize)
}

type NullableStreamSinkInOneOf3 struct {
	value *StreamSinkInOneOf3
	isSet bool
}

func (v NullableStreamSinkInOneOf3) Get() *StreamSinkInOneOf3 {
	return v.value
}

func (v *NullableStreamSinkInOneOf3) Set(val *StreamSinkInOneOf3) {
	v.value = val
	v.isSet = true
}

func (v NullableStreamSinkInOneOf3) IsSet() bool {
	return v.isSet
}

func (v *NullableStreamSinkInOneOf3) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableStreamSinkInOneOf3(val *StreamSinkInOneOf3) *NullableStreamSinkInOneOf3 {
	return &NullableStreamSinkInOneOf3{value: val, isSet: true}
}

func (v NullableStreamSinkInOneOf3) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableStreamSinkInOneOf3) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


