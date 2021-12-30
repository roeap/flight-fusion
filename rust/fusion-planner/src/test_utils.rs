use flight_fusion_ipc::{
    signal_provider::Source as ProviderSource, table_reference::Table as TableRef, FileFormat,
    FileReference, Signal, SignalProvider, TableReference,
};

pub fn get_provider() -> SignalProvider {
    SignalProvider {
        uid: "provider-id".to_string(),
        name: "provider".to_string(),
        description: "description".to_string(),
        signals: vec![Signal {
            uid: "signal-id".to_string(),
            name: "signal".to_string(),
            description: "description".to_string(),
        }],
        source: Some(ProviderSource::Table(TableReference {
            table: Some(TableRef::File(FileReference {
                path: "/home/robstar/github/flight-fusion/.tmp/file/table.parquet".to_string(),
                format: FileFormat::Parquet as i32,
            })),
        })),
    }
}
