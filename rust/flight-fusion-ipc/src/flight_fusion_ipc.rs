// This file was automatically generated through the build.rs script, and should not be edited.
#[rustfmt::skip]

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaReference {
    #[prost(string, tag="1")]
    pub location: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingReference {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
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
pub struct DropDatasetRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropDatasetResponse {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlTicket {
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KqlTicket {
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
}
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutRemoteTableRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutRemoteTableResponse {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
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
        Drop(super::DropDatasetRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightDoGetRequest {
    #[prost(oneof="flight_do_get_request::Operation", tags="1, 2")]
    pub operation: ::core::option::Option<flight_do_get_request::Operation>,
}
/// Nested message and enum types in `FlightDoGetRequest`.
pub mod flight_do_get_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag="1")]
        Sql(super::SqlTicket),
        #[prost(message, tag="2")]
        Kql(super::KqlTicket),
    }
}
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
        Remote(super::PutRemoteTableRequest),
        #[prost(message, tag="3")]
        Delta(super::DeltaOperationRequest),
    }
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
    Read = 0,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceAction {
    Read = 0,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpressionReference {
    #[prost(string, tag="1")]
    pub uid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub expression: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub input_frame: ::core::option::Option<SignalFrame>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelReference {
    #[prost(string, tag="1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub input_frame: ::core::option::Option<SignalFrame>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signal {
    #[prost(string, tag="1")]
    pub uid: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
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
    #[prost(oneof="signal_provider::Source", tags="100, 101")]
    pub source: ::core::option::Option<signal_provider::Source>,
}
/// Nested message and enum types in `SignalProvider`.
pub mod signal_provider {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        #[prost(message, tag="100")]
        Table(super::TableReference),
        #[prost(message, tag="101")]
        Model(super::ModelReference),
    }
}
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
