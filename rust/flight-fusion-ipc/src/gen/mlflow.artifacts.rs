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
    /// Filter artifacts matching this path (a relative path from the root artifact directory).
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ListArtifacts`.
pub mod list_artifacts {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// File location and metadata for artifacts.
        #[prost(message, repeated, tag="1")]
        pub files: ::prost::alloc::vec::Vec<super::FileInfo>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    /// Path relative to the root artifact directory run.
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether the path is a directory.
    #[prost(bool, optional, tag="2")]
    pub is_dir: ::core::option::Option<bool>,
    /// Size in bytes. Unset for directories.
    #[prost(int64, optional, tag="3")]
    pub file_size: ::core::option::Option<i64>,
}
/// Encoded file descriptor set for the `mlflow.artifacts` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb0, 0x14, 0x0a, 0x1d, 0x6d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x2f, 0x6d, 0x6c, 0x66, 0x6c,
    0x6f, 0x77, 0x5f, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x10, 0x6d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x2e, 0x61, 0x72, 0x74, 0x69, 0x66,
    0x61, 0x63, 0x74, 0x73, 0x22, 0x46, 0x0a, 0x10, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64,
    0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x1a, 0x1e, 0x0a, 0x08,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x22, 0x44, 0x0a, 0x0e,
    0x55, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x12, 0x12,
    0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61,
    0x74, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x1a, 0x0a, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x22, 0x6f, 0x0a, 0x0d, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61,
    0x63, 0x74, 0x73, 0x12, 0x17, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x48, 0x00, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x88, 0x01, 0x01, 0x1a, 0x3c, 0x0a, 0x08,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x30, 0x0a, 0x05, 0x66, 0x69, 0x6c, 0x65,
    0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6d, 0x6c, 0x66, 0x6c, 0x6f, 0x77,
    0x2e, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x2e, 0x46, 0x69, 0x6c, 0x65, 0x49,
    0x6e, 0x66, 0x6f, 0x52, 0x05, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x42, 0x07, 0x0a, 0x05, 0x5f, 0x70,
    0x61, 0x74, 0x68, 0x22, 0x83, 0x01, 0x0a, 0x08, 0x46, 0x69, 0x6c, 0x65, 0x49, 0x6e, 0x66, 0x6f,
    0x12, 0x17, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00,
    0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x88, 0x01, 0x01, 0x12, 0x1a, 0x0a, 0x06, 0x69, 0x73, 0x5f,
    0x64, 0x69, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x48, 0x01, 0x52, 0x05, 0x69, 0x73, 0x44,
    0x69, 0x72, 0x88, 0x01, 0x01, 0x12, 0x20, 0x0a, 0x09, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x73, 0x69,
    0x7a, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x48, 0x02, 0x52, 0x08, 0x66, 0x69, 0x6c, 0x65,
    0x53, 0x69, 0x7a, 0x65, 0x88, 0x01, 0x01, 0x42, 0x07, 0x0a, 0x05, 0x5f, 0x70, 0x61, 0x74, 0x68,
    0x42, 0x09, 0x0a, 0x07, 0x5f, 0x69, 0x73, 0x5f, 0x64, 0x69, 0x72, 0x42, 0x0c, 0x0a, 0x0a, 0x5f,
    0x66, 0x69, 0x6c, 0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x32, 0xc2, 0x02, 0x0a, 0x16, 0x4d, 0x6c,
    0x66, 0x6c, 0x6f, 0x77, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x53, 0x65, 0x72,
    0x76, 0x69, 0x63, 0x65, 0x12, 0x67, 0x0a, 0x10, 0x64, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64,
    0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x12, 0x22, 0x2e, 0x6d, 0x6c, 0x66, 0x6c, 0x6f,
    0x77, 0x2e, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x2e, 0x44, 0x6f, 0x77, 0x6e,
    0x6c, 0x6f, 0x61, 0x64, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x1a, 0x2b, 0x2e, 0x6d,
    0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x2e, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x2e,
    0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74,
    0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x30, 0x01, 0x12, 0x61, 0x0a,
    0x0e, 0x75, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x12,
    0x20, 0x2e, 0x6d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x2e, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63,
    0x74, 0x73, 0x2e, 0x55, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63,
    0x74, 0x1a, 0x29, 0x2e, 0x6d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x2e, 0x61, 0x72, 0x74, 0x69, 0x66,
    0x61, 0x63, 0x74, 0x73, 0x2e, 0x55, 0x70, 0x6c, 0x6f, 0x61, 0x64, 0x41, 0x72, 0x74, 0x69, 0x66,
    0x61, 0x63, 0x74, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x28, 0x01,
    0x12, 0x5c, 0x0a, 0x0d, 0x6c, 0x69, 0x73, 0x74, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74,
    0x73, 0x12, 0x1f, 0x2e, 0x6d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x2e, 0x61, 0x72, 0x74, 0x69, 0x66,
    0x61, 0x63, 0x74, 0x73, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63,
    0x74, 0x73, 0x1a, 0x28, 0x2e, 0x6d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x2e, 0x61, 0x72, 0x74, 0x69,
    0x66, 0x61, 0x63, 0x74, 0x73, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61,
    0x63, 0x74, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x00, 0x42, 0xc8,
    0x01, 0x0a, 0x14, 0x63, 0x6f, 0x6d, 0x2e, 0x6d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x2e, 0x61, 0x72,
    0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x42, 0x14, 0x4d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x41,
    0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a,
    0x36, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6d, 0x6c, 0x66, 0x75,
    0x73, 0x69, 0x6f, 0x6e, 0x2f, 0x66, 0x75, 0x73, 0x69, 0x6f, 0x6e, 0x2f, 0x70, 0x72, 0x69, 0x76,
    0x61, 0x74, 0x65, 0x2f, 0x67, 0x65, 0x6e, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f,
    0x2f, 0x6d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x90, 0x01, 0x01, 0xa2, 0x02, 0x03, 0x4d, 0x41, 0x58,
    0xaa, 0x02, 0x10, 0x4d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x2e, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61,
    0x63, 0x74, 0x73, 0xca, 0x02, 0x10, 0x4d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x5c, 0x41, 0x72, 0x74,
    0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0xe2, 0x02, 0x1c, 0x4d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x5c,
    0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74,
    0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x11, 0x4d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x3a, 0x3a,
    0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x4a, 0xdf, 0x0c, 0x0a, 0x06, 0x12, 0x04,
    0x06, 0x00, 0x38, 0x01, 0x0a, 0xd8, 0x02, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x06, 0x00, 0x12, 0x32,
    0xcd, 0x02, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x66, 0x69,
    0x6c, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d,
    0x4c, 0x66, 0x6c, 0x6f, 0x77, 0x20, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x20,
    0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x70, 0x72, 0x6f,
    0x76, 0x69, 0x64, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77,
    0x69, 0x6e, 0x67, 0x20, 0x52, 0x45, 0x53, 0x54, 0x20, 0x41, 0x50, 0x49, 0x73, 0x0a, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x70, 0x72, 0x6f, 0x78, 0x69, 0x65, 0x64, 0x20, 0x61, 0x72, 0x74, 0x69, 0x66,
    0x61, 0x63, 0x74, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x3a, 0x0a,
    0x20, 0x2d, 0x20, 0x2f, 0x6d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x2d, 0x61, 0x72, 0x74, 0x69, 0x66,
    0x61, 0x63, 0x74, 0x73, 0x2f, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x2f, 0x3c,
    0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x3e, 0x20, 0x47,
    0x45, 0x54, 0x3a, 0x20, 0x44, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x61, 0x6e, 0x20,
    0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x0a, 0x20, 0x2d, 0x20, 0x2f, 0x6d, 0x6c, 0x66,
    0x6c, 0x6f, 0x77, 0x2d, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x2f, 0x61, 0x72,
    0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x2f, 0x3c, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63,
    0x74, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x3e, 0x20, 0x50, 0x55, 0x54, 0x3a, 0x20, 0x55, 0x70, 0x6c,
    0x6f, 0x61, 0x64, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x0a,
    0x20, 0x2d, 0x20, 0x2f, 0x6d, 0x6c, 0x66, 0x6c, 0x6f, 0x77, 0x2d, 0x61, 0x72, 0x74, 0x69, 0x66,
    0x61, 0x63, 0x74, 0x73, 0x2f, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x3f, 0x70, 0x61,
    0x74, 0x68, 0x3d, 0x3c, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3e, 0x20, 0x47, 0x45, 0x54, 0x3a, 0x20,
    0x4c, 0x69, 0x73, 0x74, 0x20, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x0a, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x08, 0x00, 0x19, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x0a, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x12, 0x12, 0x03, 0x0a, 0x00, 0x22, 0x0a, 0x0a,
    0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0c, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00,
    0x01, 0x12, 0x03, 0x0c, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0e, 0x02, 0x57, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x06,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0e, 0x18, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0e, 0x33, 0x39, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x3a, 0x53, 0x0a, 0x0b, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x10, 0x02, 0x51, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x10, 0x06, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x10, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x10,
    0x1d, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x10, 0x36, 0x4d,
    0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03, 0x12, 0x02, 0x47, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x06, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x12, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x12, 0x2d, 0x43, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x15,
    0x00, 0x1b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x15, 0x08, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x16, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x16, 0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04,
    0x18, 0x02, 0x1a, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x18,
    0x0a, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x19, 0x04,
    0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x19, 0x04,
    0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x0a,
    0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x11,
    0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1d, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x1e, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x1e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e,
    0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x10, 0x11,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x20, 0x02, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x20, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x20, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x20, 0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x03, 0x00, 0x12,
    0x03, 0x22, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x03, 0x00, 0x01, 0x12, 0x03, 0x22,
    0x0a, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x25, 0x00, 0x2d, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x25, 0x08, 0x15, 0x0a, 0x66, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x27, 0x02, 0x1b, 0x1a, 0x59, 0x20, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x20, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x28, 0x61,
    0x20, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x66,
    0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x6f, 0x6f, 0x74, 0x20, 0x61, 0x72, 0x74,
    0x69, 0x66, 0x61, 0x63, 0x74, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x29,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x27, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x27, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02,
    0x03, 0x00, 0x12, 0x04, 0x29, 0x02, 0x2c, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x29, 0x0a, 0x12, 0x0a, 0x3a, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x2b, 0x04, 0x20, 0x1a, 0x2b, 0x20, 0x46, 0x69, 0x6c, 0x65, 0x20, 0x6c, 0x6f, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61,
    0x74, 0x61, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x73,
    0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2b,
    0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2b,
    0x0d, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b,
    0x16, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b,
    0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2f, 0x00, 0x38, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x10, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x31, 0x02, 0x1b, 0x1a, 0x33, 0x20, 0x50, 0x61, 0x74, 0x68, 0x20, 0x72,
    0x65, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x6f, 0x6f, 0x74, 0x20, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x20, 0x64, 0x69, 0x72,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x72, 0x75, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x31, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x31, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x31, 0x19, 0x1a, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x34, 0x02,
    0x1b, 0x1a, 0x22, 0x20, 0x57, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x70, 0x61, 0x74, 0x68, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x34, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x34, 0x10, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x34, 0x19, 0x1a, 0x0a, 0x34, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x37, 0x02, 0x1f, 0x1a, 0x27, 0x20, 0x53, 0x69, 0x7a,
    0x65, 0x20, 0x69, 0x6e, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x2e, 0x20, 0x55, 0x6e, 0x73, 0x65,
    0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x65,
    0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x37, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x37, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x37, 0x11, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x37, 0x1d, 0x1e, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
include!("mlflow.artifacts.tonic.rs");
// @@protoc_insertion_point(module)