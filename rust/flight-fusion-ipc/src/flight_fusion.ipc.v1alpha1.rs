// This file was automatically generated through the build.rs script, and should not be edited.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaReference {
    #[prost(string, tag="1")]
    pub location: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingReference {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(enumeration="FileFormat", tag="2")]
    pub format: i32,
    #[prost(string, repeated, tag="3")]
    pub partition_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileReference {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(enumeration="FileFormat", tag="2")]
    pub format: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableReference {
    #[prost(oneof="table_reference::Table", tags="1, 2, 3")]
    pub table: ::core::option::Option<table_reference::Table>,
}
/// Nested message and enum types in `TableReference`.
pub mod table_reference {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Table {
        #[prost(message, tag="1")]
        Delta(super::DeltaReference),
        #[prost(message, tag="2")]
        Listing(super::ListingReference),
        #[prost(message, tag="3")]
        File(super::FileReference),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AreaTableLocation {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub areas: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AreaTableId {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AreaTableUri {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AreaSourceReference {
    #[prost(oneof="area_source_reference::Table", tags="1, 2, 3")]
    pub table: ::core::option::Option<area_source_reference::Table>,
}
/// Nested message and enum types in `AreaSourceReference`.
pub mod area_source_reference {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Table {
        #[prost(message, tag="1")]
        Location(super::AreaTableLocation),
        #[prost(message, tag="2")]
        Id(super::AreaTableId),
        #[prost(message, tag="3")]
        Uri(super::AreaTableUri),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FileFormat {
    Unspecified = 0,
    Parquet = 1,
    Avro = 2,
    Csv = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatasetFormat {
    File = 0,
    Dataset = 1,
    Delta = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SaveMode {
    Unspecified = 0,
    Append = 1,
    Overwrite = 2,
    ErrorIfExists = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StorageType {
    Unspecified = 0,
    Local = 1,
    Hdfs = 2,
    AzureAdlsV2 = 3,
    AzureBlob = 4,
    S3 = 5,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpressionReference {
    #[prost(string, tag="1")]
    pub uid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub expression: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelReference {
    #[prost(string, tag="1")]
    pub uri: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signal {
    #[prost(string, tag="1")]
    pub uid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="10")]
    pub traits: ::prost::alloc::vec::Vec<SignalTrait>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignalTrait {
    #[prost(oneof="signal_trait::Trait", tags="1, 2, 3")]
    pub r#trait: ::core::option::Option<signal_trait::Trait>,
}
/// Nested message and enum types in `SignalTrait`.
pub mod signal_trait {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trait {
        #[prost(message, tag="1")]
        Sensitive(super::SensitiveDataTrait),
        #[prost(message, tag="2")]
        TimeSeries(super::TimeSeriesTrait),
        #[prost(message, tag="3")]
        EntityReference(super::EntityReferenceTrait),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SensitiveDataTrait {
    #[prost(string, tag="1")]
    pub level: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesTrait {
    #[prost(string, tag="1")]
    pub level: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityReferenceTrait {
    #[prost(string, tag="1")]
    pub level: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignalProvider {
    #[prost(string, tag="1")]
    pub uid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub signals: ::prost::alloc::vec::Vec<Signal>,
    #[prost(message, repeated, tag="5")]
    pub inputs: ::prost::alloc::vec::Vec<Signal>,
    #[prost(oneof="signal_provider::Source", tags="100, 101, 102")]
    pub source: ::core::option::Option<signal_provider::Source>,
}
/// Nested message and enum types in `SignalProvider`.
pub mod signal_provider {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        #[prost(message, tag="100")]
        Table(super::TableReference),
        #[prost(message, tag="101")]
        Expression(super::ExpressionReference),
        #[prost(message, tag="102")]
        Model(super::ModelReference),
    }
}
/// A SignalFrame defines the context for a specialized query across
/// multiple data sources
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignalFrame {
    #[prost(string, tag="1")]
    pub uid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub providers: ::prost::alloc::vec::Vec<SignalProvider>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignalType {
    Unspecified = 0,
    Observation = 1,
    Constant = 2,
    Expression = 3,
    Model = 4,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterDatasetRequest {
    #[prost(enumeration="DatasetFormat", tag="1")]
    pub format: i32,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterDatasetResponse {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropDatasetResponse {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Describes an SQL query operation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandSqlOperation {
    /// The SQL syntax.
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
}
/// Describes a KQL query operation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandKqlOperation {
    /// name of the Kusto service to be queried
    #[prost(string, tag="1")]
    pub service_name: ::prost::alloc::string::String,
    /// The KQL syntax.
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
}
// Commands

/// Read entire table from storage
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandReadDataset {
    /// table identifier
    #[prost(message, optional, tag="1")]
    pub table: ::core::option::Option<AreaSourceReference>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandDropDataset {
    /// table identifier
    #[prost(message, optional, tag="1")]
    pub table: ::core::option::Option<AreaSourceReference>,
}
/// Request to write data to area storage
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandWriteIntoDataset {
    /// table identifier
    #[prost(message, optional, tag="1")]
    pub table: ::core::option::Option<AreaSourceReference>,
    /// denotes how to beahve for existing data - defaults to overwrite
    #[prost(enumeration="SaveMode", tag="3")]
    pub save_mode: i32,
}
// Signals

/// Describes a signal frame operation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignalFrameOperation {
    #[prost(message, optional, tag="1")]
    pub frame: ::core::option::Option<SignalFrame>,
}
// Delta

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaCreateOperation {
    #[prost(enumeration="SaveMode", tag="1")]
    pub save_mode: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaWriteOperation {
    #[prost(enumeration="SaveMode", tag="1")]
    pub save_mode: i32,
    #[prost(string, repeated, tag="2")]
    pub partition_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub predicate: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaOperationRequest {
    #[prost(message, optional, tag="1")]
    pub table: ::core::option::Option<DeltaReference>,
    #[prost(oneof="delta_operation_request::Operation", tags="10, 11")]
    pub operation: ::core::option::Option<delta_operation_request::Operation>,
}
/// Nested message and enum types in `DeltaOperationRequest`.
pub mod delta_operation_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag="10")]
        Create(super::DeltaCreateOperation),
        #[prost(message, tag="11")]
        Write(super::DeltaWriteOperation),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaOperationResponse {
    #[prost(string, tag="1")]
    pub stats: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMemoryTableRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMemoryTableResponse {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
// Flight

/// Requests submitted against the `do_get` endpoint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightDoGetRequest {
    #[prost(oneof="flight_do_get_request::Operation", tags="1, 2, 3, 4")]
    pub operation: ::core::option::Option<flight_do_get_request::Operation>,
}
/// Nested message and enum types in `FlightDoGetRequest`.
pub mod flight_do_get_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag="1")]
        Sql(super::CommandSqlOperation),
        #[prost(message, tag="2")]
        Kql(super::CommandKqlOperation),
        #[prost(message, tag="3")]
        Frame(super::SignalFrameOperation),
        #[prost(message, tag="4")]
        Read(super::CommandReadDataset),
    }
}
/// Requests submitted against the `do_put` endpoint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightDoPutRequest {
    #[prost(oneof="flight_do_put_request::Operation", tags="1, 2, 3")]
    pub operation: ::core::option::Option<flight_do_put_request::Operation>,
}
/// Nested message and enum types in `FlightDoPutRequest`.
pub mod flight_do_put_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag="1")]
        Memory(super::PutMemoryTableRequest),
        #[prost(message, tag="2")]
        Storage(super::CommandWriteIntoDataset),
        #[prost(message, tag="3")]
        Delta(super::DeltaOperationRequest),
    }
}
/// Requests submitted against the `do_action` endpoint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightActionRequest {
    #[prost(oneof="flight_action_request::Action", tags="1, 2")]
    pub action: ::core::option::Option<flight_action_request::Action>,
}
/// Nested message and enum types in `FlightActionRequest`.
pub mod flight_action_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag="1")]
        Register(super::RegisterDatasetRequest),
        #[prost(message, tag="2")]
        Drop(super::CommandDropDataset),
    }
}
///
/// Returned from the RPC call DoPut when a CommandStatementUpdate
/// CommandPreparedStatementUpdate was in the request, containing
/// results from the update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoPutUpdateResult {
    #[prost(message, optional, tag="1")]
    pub statistics: ::core::option::Option<BatchStatistics>,
}
///
/// Statistics for a physical plan node
/// Fields are optional and can be inexact because the sources
/// sometimes provide approximate estimates for performance reasons
/// and the transformations output are not always predictable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchStatistics {
    /// The number of table rows
    #[prost(int64, tag="1")]
    pub record_count: i64,
    /// total byte of the table rows
    #[prost(int64, tag="2")]
    pub total_byte_size: i64,
    /// Statistics on a column level
    #[prost(message, repeated, tag="3")]
    pub column_statistics: ::prost::alloc::vec::Vec<ColumnStatistics>,
    /// If true, any field that is `Some(..)` is the actual value in the data provided by the operator (it is not
    /// an estimate). Any or all other fields might still be None, in which case no information is known.
    /// if false, any field that is `Some(..)` may contain an inexact estimate and may not be the actual value.
    #[prost(bool, tag="4")]
    pub is_exact: bool,
}
/// This table statistics are estimates about column
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnStatistics {
    /// Number of null values on column
    #[prost(int64, tag="1")]
    pub null_count: i64,
    /// Maximum value of column
    #[prost(string, tag="2")]
    pub max_value: ::prost::alloc::string::String,
    /// Minimum value of column
    #[prost(string, tag="3")]
    pub min_value: ::prost::alloc::string::String,
    /// Number of distinct values
    #[prost(int64, tag="4")]
    pub distinct_count: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Integrity {
    #[prost(int32, tag="1")]
    pub version: i32,
    #[prost(string, tag="2")]
    pub key_name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub hmac: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(string, tag="1")]
    pub service: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Passport {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub user_info: ::core::option::Option<UserInfo>,
    #[prost(message, optional, tag="3")]
    pub device_info: ::core::option::Option<DeviceInfo>,
    #[prost(message, optional, tag="4")]
    pub user_integrity: ::core::option::Option<Integrity>,
    #[prost(message, optional, tag="5")]
    pub device_integrity: ::core::option::Option<Integrity>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    #[prost(enumeration="Source", tag="1")]
    pub source: i32,
    #[prost(int64, tag="2")]
    pub created: i64,
    #[prost(int64, tag="3")]
    pub expires: i64,
    #[prost(int64, tag="4")]
    pub customer_id: i64,
    #[prost(enumeration="PassportAuthenticationLevel", tag="11")]
    pub authentication_level: i32,
    #[prost(enumeration="UserAction", repeated, tag="12")]
    pub actions: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    #[prost(enumeration="Source", tag="1")]
    pub source: i32,
    #[prost(int64, tag="2")]
    pub created: i64,
    #[prost(int64, tag="3")]
    pub expires: i64,
    #[prost(string, tag="4")]
    pub esn: ::prost::alloc::string::String,
    #[prost(int32, tag="5")]
    pub device_type: i32,
    #[prost(enumeration="DeviceAction", repeated, tag="7")]
    pub actions: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="PassportAuthenticationLevel", tag="8")]
    pub authentication_level: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Source {
    None = 0,
    Cookie = 1,
    CookieInsecure = 2,
    Msl = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PassportAuthenticationLevel {
    /// untrusted transport
    Low = 0,
    /// secure tokens over TLS
    High = 1,
    /// MSL or user credentials
    Highest = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserAction {
    Unspecified = 0,
    Read = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceAction {
    Unspecified = 0,
    Read = 1,
}
