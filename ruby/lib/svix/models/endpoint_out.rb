# frozen_string_literal: true
# This file is @generated
require "json"

module Svix
  class EndpointOut
    # List of message channels this endpoint listens to (omit for all).
    attr_accessor :channels
    attr_accessor :created_at
    # An example endpoint name.
    attr_accessor :description
    attr_accessor :disabled
    attr_accessor :filter_types
    # The Endpoint's ID.
    attr_accessor :id
    attr_accessor :metadata
    attr_accessor :rate_limit
    # Optional unique identifier for the endpoint.
    attr_accessor :uid
    attr_accessor :updated_at
    attr_accessor :url
    attr_accessor :version

    ALL_FIELD ||= [
      "channels",
      "created_at",
      "description",
      "disabled",
      "filter_types",
      "id",
      "metadata",
      "rate_limit",
      "uid",
      "updated_at",
      "url",
      "version"
    ].freeze
    private_constant :ALL_FIELD

    def initialize(attributes = {})
      unless attributes.is_a?(Hash)
        fail(ArgumentError, "The input argument (attributes) must be a hash in `Svix::EndpointOut` new method")
      end

      attributes.each do |k, v|
        unless ALL_FIELD.include?(k.to_s)
          fail(ArgumentError, "The field #{k} is not part of Svix::EndpointOut")
        end

        instance_variable_set("@#{k}", v)
        instance_variable_set("@__#{k}_is_defined", true)
      end
    end

    def self.deserialize(attributes = {})
      attributes = attributes.transform_keys(&:to_s)
      attrs = Hash.new
      attrs["channels"] = attributes["channels"]
      attrs["created_at"] = DateTime.rfc3339(attributes["createdAt"]).to_time
      attrs["description"] = attributes["description"]
      attrs["disabled"] = attributes["disabled"]
      attrs["filter_types"] = attributes["filterTypes"]
      attrs["id"] = attributes["id"]
      attrs["metadata"] = attributes["metadata"]
      attrs["rate_limit"] = attributes["rateLimit"]
      attrs["uid"] = attributes["uid"]
      attrs["updated_at"] = DateTime.rfc3339(attributes["updatedAt"]).to_time
      attrs["url"] = attributes["url"]
      attrs["version"] = attributes["version"]
      new(attrs)
    end

    def serialize
      out = Hash.new
      out["channels"] = Svix::serialize_primitive(@channels) if @channels
      out["createdAt"] = Svix::serialize_primitive(@created_at) if @created_at
      out["description"] = Svix::serialize_primitive(@description) if @description
      out["disabled"] = Svix::serialize_primitive(@disabled) if @disabled
      out["filterTypes"] = Svix::serialize_primitive(@filter_types) if @filter_types
      out["id"] = Svix::serialize_primitive(@id) if @id
      out["metadata"] = Svix::serialize_primitive(@metadata) if @metadata
      out["rateLimit"] = Svix::serialize_primitive(@rate_limit) if @rate_limit
      out["uid"] = Svix::serialize_primitive(@uid) if @uid
      out["updatedAt"] = Svix::serialize_primitive(@updated_at) if @updated_at
      out["url"] = Svix::serialize_primitive(@url) if @url
      out["version"] = Svix::serialize_primitive(@version) if @version
      out
    end

    # Serializes the object to a json string
    # @return String
    def to_json
      JSON.dump(serialize)
    end
  end
end
