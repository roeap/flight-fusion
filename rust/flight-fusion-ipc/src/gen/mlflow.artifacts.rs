// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadArtifact {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DownloadArtifact`.
pub mod download_artifact {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(bytes="vec", tag="1")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadArtifact {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `UploadArtifact`.
pub mod upload_artifact {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListArtifacts {
    ///  Filter artifacts matching this path (a relative path from the root artifact directory).
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ListArtifacts`.
pub mod list_artifacts {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        ///  File location and metadata for artifacts.
        #[prost(message, repeated, tag="1")]
        pub files: ::prost::alloc::vec::Vec<super::FileInfo>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    ///  Path relative to the root artifact directory run.
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    ///  Whether the path is a directory.
    #[prost(bool, optional, tag="2")]
    pub is_dir: ::core::option::Option<bool>,
    ///  Size in bytes. Unset for directories.
    #[prost(int64, optional, tag="3")]
    pub file_size: ::core::option::Option<i64>,
}
include!("mlflow.artifacts.tonic.rs");
// @@protoc_insertion_point(module)
