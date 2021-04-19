#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableClustersList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableCluster>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableCluster {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<ClusterSku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<cluster::Properties>,
}
pub mod cluster {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "createdAt", skip_serializing)]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", skip_serializing)]
        pub updated_at: Option<String>,
        #[serde(rename = "metricId", skip_serializing)]
        pub metric_id: Option<String>,
        #[serde(skip_serializing)]
        pub status: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterSku {
    pub name: cluster_sku::Name,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
pub mod cluster_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Dedicated,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EhNamespaceIdListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EhNamespaceIdContainer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EhNamespaceIdContainer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EhNamespaceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EhNamespace>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EhNamespace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<eh_namespace::Properties>,
}
pub mod eh_namespace {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "provisioningState", skip_serializing)]
        pub provisioning_state: Option<String>,
        #[serde(rename = "createdAt", skip_serializing)]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", skip_serializing)]
        pub updated_at: Option<String>,
        #[serde(rename = "serviceBusEndpoint", skip_serializing)]
        pub service_bus_endpoint: Option<String>,
        #[serde(rename = "clusterArmId", skip_serializing_if = "Option::is_none")]
        pub cluster_arm_id: Option<String>,
        #[serde(rename = "metricId", skip_serializing)]
        pub metric_id: Option<String>,
        #[serde(rename = "isAutoInflateEnabled", skip_serializing_if = "Option::is_none")]
        pub is_auto_inflate_enabled: Option<bool>,
        #[serde(rename = "maximumThroughputUnits", skip_serializing_if = "Option::is_none")]
        pub maximum_throughput_units: Option<i32>,
        #[serde(rename = "kafkaEnabled", skip_serializing_if = "Option::is_none")]
        pub kafka_enabled: Option<bool>,
        #[serde(rename = "zoneRedundant", skip_serializing_if = "Option::is_none")]
        pub zone_redundant: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub encryption: Option<Encryption>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Basic,
        Standard,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Basic,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(rename = "principalId", skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encryption {
    #[serde(rename = "keyVaultProperties", skip_serializing_if = "Vec::is_empty")]
    pub key_vault_properties: Vec<KeyVaultProperties>,
    #[serde(rename = "keySource", skip_serializing_if = "Option::is_none")]
    pub key_source: Option<encryption::KeySource>,
}
pub mod encryption {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.KeyVault")]
        MicrosoftKeyVault,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultProperties {
    #[serde(rename = "keyName", skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "keyVaultUri", skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[serde(rename = "keyVersion", skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState", skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<ConnectionState>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection_properties::ProvisioningState>,
}
pub mod private_endpoint_connection_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectionState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<connection_state::Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
pub mod connection_state {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        Approved,
        Rejected,
        Disconnected,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkResourcesListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationRuleListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AuthorizationRule>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationRule {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<authorization_rule::Properties>,
}
pub mod authorization_rule {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        pub rights: Vec<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessKeys {
    #[serde(rename = "primaryConnectionString", skip_serializing)]
    pub primary_connection_string: Option<String>,
    #[serde(rename = "secondaryConnectionString", skip_serializing)]
    pub secondary_connection_string: Option<String>,
    #[serde(rename = "aliasPrimaryConnectionString", skip_serializing)]
    pub alias_primary_connection_string: Option<String>,
    #[serde(rename = "aliasSecondaryConnectionString", skip_serializing)]
    pub alias_secondary_connection_string: Option<String>,
    #[serde(rename = "primaryKey", skip_serializing)]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", skip_serializing)]
    pub secondary_key: Option<String>,
    #[serde(rename = "keyName", skip_serializing)]
    pub key_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateAccessKeyParameters {
    #[serde(rename = "keyType")]
    pub key_type: regenerate_access_key_parameters::KeyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
pub mod regenerate_access_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        PrimaryKey,
        SecondaryKey,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UnavailableReason {
    None,
    InvalidName,
    SubscriptionIsDisabled,
    NameInUse,
    NameInLockdown,
    TooManyNamespaceInCurrentSubscription,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameter {
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityResult {
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(rename = "nameAvailable", skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<UnavailableReason>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumerGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<consumer_group::Properties>,
}
pub mod consumer_group {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "createdAt", skip_serializing)]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", skip_serializing)]
        pub updated_at: Option<String>,
        #[serde(rename = "userMetadata", skip_serializing_if = "Option::is_none")]
        pub user_metadata: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsumerGroupListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConsumerGroup>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing)]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing)]
        pub provider: Option<String>,
        #[serde(skip_serializing)]
        pub resource: Option<String>,
        #[serde(skip_serializing)]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaptureDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<capture_description::Encoding>,
    #[serde(rename = "intervalInSeconds", skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[serde(rename = "sizeLimitInBytes", skip_serializing_if = "Option::is_none")]
    pub size_limit_in_bytes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(rename = "skipEmptyArchives", skip_serializing_if = "Option::is_none")]
    pub skip_empty_archives: Option<bool>,
}
pub mod capture_description {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Encoding {
        Avro,
        AvroDeflate,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Eventhub {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<eventhub::Properties>,
}
pub mod eventhub {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "partitionIds", skip_serializing)]
        pub partition_ids: Vec<String>,
        #[serde(rename = "createdAt", skip_serializing)]
        pub created_at: Option<String>,
        #[serde(rename = "updatedAt", skip_serializing)]
        pub updated_at: Option<String>,
        #[serde(rename = "messageRetentionInDays", skip_serializing_if = "Option::is_none")]
        pub message_retention_in_days: Option<i64>,
        #[serde(rename = "partitionCount", skip_serializing_if = "Option::is_none")]
        pub partition_count: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[serde(rename = "captureDescription", skip_serializing_if = "Option::is_none")]
        pub capture_description: Option<CaptureDescription>,
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Active,
            Disabled,
            Restoring,
            SendDisabled,
            ReceiveDisabled,
            Creating,
            Deleting,
            Renaming,
            Unknown,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Eventhub>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Destination {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<destination::Properties>,
}
pub mod destination {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "storageAccountResourceId", skip_serializing_if = "Option::is_none")]
        pub storage_account_resource_id: Option<String>,
        #[serde(rename = "blobContainer", skip_serializing_if = "Option::is_none")]
        pub blob_container: Option<String>,
        #[serde(rename = "archiveNameFormat", skip_serializing_if = "Option::is_none")]
        pub archive_name_format: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagingRegions {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<messaging_regions::Properties>,
}
pub mod messaging_regions {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(skip_serializing)]
        pub code: Option<String>,
        #[serde(rename = "fullName", skip_serializing)]
        pub full_name: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagingRegionsListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MessagingRegions>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}