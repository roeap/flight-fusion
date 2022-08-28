// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileReference {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(enumeration="FileFormat", tag="2")]
    pub format: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableReference {
    #[prost(oneof="table_reference::Table", tags="3")]
    pub table: ::core::option::Option<table_reference::Table>,
}
/// Nested message and enum types in `TableReference`.
pub mod table_reference {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Table {
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
    pub uri: ::prost::alloc::string::String,
}
///  area identifier
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceCollection {
    #[prost(message, repeated, tag="1")]
    pub sources: ::prost::alloc::vec::Vec<AreaSourceReference>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
///  File format for a file stroed on disk
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FileFormat {
    ///  Undefined file format
    Unspecified = 0,
    ///  Stored in parquet
    Parquet = 1,
    ///  Avro
    Avro = 2,
    ///  Csv
    Csv = 3,
}
impl FileFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FileFormat::Unspecified => "FILE_FORMAT_UNSPECIFIED",
            FileFormat::Parquet => "FILE_FORMAT_PARQUET",
            FileFormat::Avro => "FILE_FORMAT_AVRO",
            FileFormat::Csv => "FILE_FORMAT_CSV",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SaveMode {
    Unspecified = 0,
    Append = 1,
    Overwrite = 2,
    ErrorIfExists = 3,
}
impl SaveMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SaveMode::Unspecified => "SAVE_MODE_UNSPECIFIED",
            SaveMode::Append => "SAVE_MODE_APPEND",
            SaveMode::Overwrite => "SAVE_MODE_OVERWRITE",
            SaveMode::ErrorIfExists => "SAVE_MODE_ERROR_IF_EXISTS",
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaCreateOperation {
    #[prost(enumeration="SaveMode", tag="1")]
    pub save_mode: i32,
    #[prost(string, tag="2")]
    pub metadata: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaWriteOperation {
    #[prost(enumeration="SaveMode", tag="1")]
    pub save_mode: i32,
    #[prost(string, repeated, tag="2")]
    pub partition_by: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub predicate: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaReadOperation {
    ///  version of delta table to load
    #[prost(int64, tag="1")]
    pub version: i64,
    ///  load delta version from point in time
    #[prost(string, tag="2")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub predicate: ::prost::alloc::string::String,
    ///  column selection to load
    #[prost(string, repeated, tag="4")]
    pub column_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaOperationRequest {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<AreaSourceReference>,
    #[prost(oneof="delta_operation_request::Operation", tags="10, 11, 12")]
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
        #[prost(message, tag="12")]
        Read(super::DeltaReadOperation),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaOperationResponse {
    #[prost(string, tag="1")]
    pub stats: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceConnection {
    #[prost(string, tag="1")]
    pub host: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub port: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentInfo {
    #[prost(string, tag="1")]
    pub server_version: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub artifact_service: ::core::option::Option<ServiceConnection>,
    #[prost(message, optional, tag="3")]
    pub model_serving: ::core::option::Option<ServiceConnection>,
    #[prost(message, optional, tag="4")]
    pub data_sets: ::core::option::Option<ServiceConnection>,
    #[prost(message, optional, tag="5")]
    pub pipelines_ui: ::core::option::Option<ServiceConnection>,
    #[prost(message, optional, tag="6")]
    pub models_ui: ::core::option::Option<ServiceConnection>,
}
///  Metadata associated with an area source
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AreaSourceMetadata {
    ///  globally unique idetifier for the source
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    ///  A human readable name for the source
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    ///  A short descrptive text that describes the content
    ///  and purpose of the data source
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    ///  wether the table supports versioning
    #[prost(bool, tag="4")]
    pub is_versioned: bool,
    ///  source identifier
    #[prost(message, optional, tag="5")]
    pub source: ::core::option::Option<AreaSourceReference>,
    ///  tags associated with source
    #[prost(message, repeated, tag="9")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
    ///  user defined properties
    #[prost(map="string, string", tag="10")]
    pub properties: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
///  Detialed metadata and statistics about a source
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AreaSourceDetails {
    ///  globally unique idetifier for the source
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    ///  Metadata associated with the source
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<AreaSourceMetadata>,
}
///
///  Statistics for a physical plan node
///  Fields are optional and can be inexact because the sources
///  sometimes provide approximate estimates for performance reasons
///  and the transformations output are not always predictable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchStatistics {
    ///  The number of table rows
    #[prost(int64, tag="1")]
    pub record_count: i64,
    ///  total byte of the table rows
    #[prost(int64, tag="2")]
    pub total_byte_size: i64,
    ///  Statistics on a column level
    #[prost(message, repeated, tag="3")]
    pub column_statistics: ::prost::alloc::vec::Vec<ColumnStatistics>,
    ///  If true, any field that is defined is the actual value in the data provided
    ///  by the operator (it is not an estimate). Any or all other fields might
    ///  still be None, in which case no information is known. if false, any field
    ///  that is has a value may contain an inexact estimate and may not be the
    ///  actual value.
    #[prost(bool, tag="4")]
    pub is_exact: bool,
}
///  This table statistics are estimates about column properties
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnStatistics {
    ///  Number of null values on column
    #[prost(int64, tag="1")]
    pub null_count: i64,
    ///  Maximum value of column
    #[prost(string, tag="2")]
    pub max_value: ::prost::alloc::string::String,
    ///  Minimum value of column
    #[prost(string, tag="3")]
    pub min_value: ::prost::alloc::string::String,
    ///  Number of distinct values
    #[prost(int64, tag="4")]
    pub distinct_count: i64,
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
    #[prost(enumeration="DataType", tag="4")]
    pub data_type: i32,
    #[prost(bool, tag="5")]
    pub nullable: bool,
    #[prost(message, repeated, tag="10")]
    pub traits: ::prost::alloc::vec::Vec<SignalTrait>,
    #[prost(map="string, string", tag="11")]
    pub metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
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
///  A SignalFrame defines the context for a specialized query across
///  multiple data sources
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
impl SignalType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignalType::Unspecified => "SIGNAL_TYPE_UNSPECIFIED",
            SignalType::Observation => "SIGNAL_TYPE_OBSERVATION",
            SignalType::Constant => "SIGNAL_TYPE_CONSTANT",
            SignalType::Expression => "SIGNAL_TYPE_EXPRESSION",
            SignalType::Model => "SIGNAL_TYPE_MODEL",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    Unspecified = 0,
}
impl DataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataType::Unspecified => "DATA_TYPE_UNSPECIFIED",
        }
    }
}
///  Describes a KQL query operation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandKqlOperation {
    ///  name of the Kusto service to be queried
    #[prost(string, tag="1")]
    pub service_name: ::prost::alloc::string::String,
    ///  The KQL syntax.
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
}
//  Commands

///  List all sources defined under an area node
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandListSources {
    ///  If true, all sources in child nodes are listed as well
    #[prost(bool, tag="1")]
    pub recursive: bool,
}
///  Read entire table from storage
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandReadDataset {
    ///  source identifier
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<AreaSourceReference>,
    ///  column selection to load
    #[prost(string, repeated, tag="2")]
    pub column_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  Drop a source (e.g. a Table) from the service
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandDropSource {
    ///  source identifier
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<AreaSourceReference>,
}
///  Request to write data to area storage
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandWriteIntoDataset {
    ///  source identifier
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<AreaSourceReference>,
    ///  denotes how to beahve for existing data - defaults to append
    #[prost(enumeration="SaveMode", tag="3")]
    pub save_mode: i32,
}
///  Execute a query against a given context
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandExecuteQuery {
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
    #[prost(oneof="command_execute_query::Context", tags="10, 11, 12")]
    pub context: ::core::option::Option<command_execute_query::Context>,
}
/// Nested message and enum types in `CommandExecuteQuery`.
pub mod command_execute_query {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Context {
        #[prost(message, tag="10")]
        Source(super::AreaSourceReference),
        #[prost(message, tag="11")]
        Frame(super::SignalFrame),
        #[prost(message, tag="12")]
        Collection(super::SourceCollection),
    }
}
///  result when a source is dropped
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultActionStatus {
    #[prost(enumeration="ActionStatus", tag="1")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultDoPutUpdate {
    #[prost(message, optional, tag="1")]
    pub statistics: ::core::option::Option<BatchStatistics>,
}
//  Results

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionStatus {
    Unspecified = 0,
    Success = 1,
    Failure = 2,
}
impl ActionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ActionStatus::Unspecified => "ACTION_STATUS_UNSPECIFIED",
            ActionStatus::Success => "ACTION_STATUS_SUCCESS",
            ActionStatus::Failure => "ACTION_STATUS_FAILURE",
        }
    }
}
///  Requests submitted against the `do_get` endpoint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightDoGetRequest {
    #[prost(oneof="flight_do_get_request::Command", tags="1, 2, 3, 4")]
    pub command: ::core::option::Option<flight_do_get_request::Command>,
}
/// Nested message and enum types in `FlightDoGetRequest`.
pub mod flight_do_get_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        ///  execute a KQL query against a registered Kusto cluster
        #[prost(message, tag="1")]
        Kql(super::CommandKqlOperation),
        ///  Read data from a registered source
        #[prost(message, tag="2")]
        Read(super::CommandReadDataset),
        ///  Execute a query against a pre-defined context
        #[prost(message, tag="3")]
        Query(super::CommandExecuteQuery),
        ///  Perform a read operation against Delta table
        #[prost(message, tag="4")]
        Delta(super::DeltaOperationRequest),
    }
}
///  Requests submitted against the `do_put` endpoint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightDoPutRequest {
    #[prost(oneof="flight_do_put_request::Command", tags="2, 3")]
    pub command: ::core::option::Option<flight_do_put_request::Command>,
}
/// Nested message and enum types in `FlightDoPutRequest`.
pub mod flight_do_put_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        ///  Write data into a registered source
        #[prost(message, tag="2")]
        Storage(super::CommandWriteIntoDataset),
        ///  Write data into delta table
        #[prost(message, tag="3")]
        Delta(super::DeltaOperationRequest),
    }
}
///  Response recieved from `do_put` operations`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightDoPutResponse {
    #[prost(oneof="flight_do_put_response::Payload", tags="1")]
    pub payload: ::core::option::Option<flight_do_put_response::Payload>,
}
/// Nested message and enum types in `FlightDoPutResponse`.
pub mod flight_do_put_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        ///  statistics for data written to source
        #[prost(message, tag="1")]
        Update(super::ResultDoPutUpdate),
    }
}
///  Requests submitted against the `do_action` endpoint
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightActionRequest {
    ///  parameters for the specific action to be executed.
    #[prost(oneof="flight_action_request::Action", tags="2")]
    pub action: ::core::option::Option<flight_action_request::Action>,
}
/// Nested message and enum types in `FlightActionRequest`.
pub mod flight_action_request {
    ///  parameters for the specific action to be executed.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        ///  command to remove a dataset from the area store
        #[prost(message, tag="2")]
        Drop(super::CommandDropSource),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightActionResponse {
    #[prost(oneof="flight_action_response::Payload", tags="1")]
    pub payload: ::core::option::Option<flight_action_response::Payload>,
}
/// Nested message and enum types in `FlightActionResponse`.
pub mod flight_action_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        ///  Result when actions reports its execution status
        #[prost(message, tag="1")]
        Status(super::ResultActionStatus),
    }
}
// @@protoc_insertion_point(module)
