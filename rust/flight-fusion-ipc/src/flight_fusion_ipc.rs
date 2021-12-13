#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatasetFormat {
    File = 0,
    Dataset = 1,
    Delta = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterDatasetRequest {
    #[prost(enumeration = "DatasetFormat", tag = "1")]
    pub format: i32,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterDatasetResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropDatasetRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropDatasetResponse {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlTicket {
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KqlTicket {
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMemoryTableRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutMemoryTableResponse {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutRemoteTableRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutRemoteTableResponse {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightActionRequest {
    #[prost(oneof = "flight_action_request::Action", tags = "1, 2")]
    pub action: ::core::option::Option<flight_action_request::Action>,
}
/// Nested message and enum types in `FlightActionRequest`.
pub mod flight_action_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag = "1")]
        Register(super::RegisterDatasetRequest),
        #[prost(message, tag = "2")]
        Drop(super::DropDatasetRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightDoGetRequest {
    #[prost(oneof = "flight_do_get_request::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<flight_do_get_request::Operation>,
}
/// Nested message and enum types in `FlightDoGetRequest`.
pub mod flight_do_get_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag = "1")]
        Sql(super::SqlTicket),
        #[prost(message, tag = "2")]
        Kql(super::KqlTicket),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightDoPutRequest {
    #[prost(oneof = "flight_do_put_request::Operation", tags = "1, 2")]
    pub operation: ::core::option::Option<flight_do_put_request::Operation>,
}
/// Nested message and enum types in `FlightDoPutRequest`.
pub mod flight_do_put_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        #[prost(message, tag = "1")]
        Memory(super::PutMemoryTableRequest),
        #[prost(message, tag = "2")]
        Remote(super::PutRemoteTableRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Integrity {
    #[prost(int32, tag = "1")]
    pub version: i32,
    #[prost(string, tag = "2")]
    pub key_name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub hmac: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Passport {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub user_info: ::core::option::Option<UserInfo>,
    #[prost(message, optional, tag = "3")]
    pub device_info: ::core::option::Option<DeviceInfo>,
    #[prost(message, optional, tag = "4")]
    pub user_integrity: ::core::option::Option<Integrity>,
    #[prost(message, optional, tag = "5")]
    pub device_integrity: ::core::option::Option<Integrity>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    #[prost(enumeration = "Source", tag = "1")]
    pub source: i32,
    #[prost(int64, tag = "2")]
    pub created: i64,
    #[prost(int64, tag = "3")]
    pub expires: i64,
    #[prost(int64, tag = "4")]
    pub customer_id: i64,
    #[prost(enumeration = "PassportAuthenticationLevel", tag = "11")]
    pub authentication_level: i32,
    #[prost(enumeration = "UserAction", repeated, tag = "12")]
    pub actions: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    #[prost(enumeration = "Source", tag = "1")]
    pub source: i32,
    #[prost(int64, tag = "2")]
    pub created: i64,
    #[prost(int64, tag = "3")]
    pub expires: i64,
    #[prost(string, tag = "4")]
    pub esn: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub device_type: i32,
    #[prost(enumeration = "DeviceAction", repeated, tag = "7")]
    pub actions: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration = "PassportAuthenticationLevel", tag = "8")]
    pub authentication_level: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Source {
    None = 0,
    Cookie = 1,
    CookieInsecure = 2,
    Msl = 3,
    PartnerToken = 4,
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
