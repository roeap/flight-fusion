use flight_fusion_ipc::{
    signal_provider::Source as ProviderSource, table_reference::Table as TableRef,
    ExpressionReference, FileFormat, FileReference, Signal, SignalFrame, SignalProvider,
    TableReference,
};

/// Run cargo to get the root of the workspace
pub fn workspace_root() -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("cargo")
        .arg("metadata")
        .output()?;
    let output = String::from_utf8_lossy(&output.stdout);

    let key = "workspace_root\":\"";
    let index = output
        .find(key)
        .ok_or_else(|| "workspace_root key not found in metadata".to_string())?;
    let value = &output[index + key.len()..];
    let end = value
        .find('"')
        .ok_or_else(|| "workspace_root value was malformed".to_string())?;
    Ok(value[..end].into())
}

pub fn get_provider_1() -> SignalProvider {
    let mut path = workspace_root().unwrap();
    path.push_str("/test/data/P1.parquet");
    SignalProvider {
        uid: "provider-1".to_string(),
        name: "P1".to_string(),
        description: "first test provider".to_string(),
        signals: vec![
            Signal {
                uid: "id-S2".to_string(),
                name: "S2".to_string(),
                description: "S2".to_string(),
            },
            Signal {
                uid: "id-S3".to_string(),
                name: "S3".to_string(),
                description: "S3".to_string(),
            },
            Signal {
                uid: "id-S5".to_string(),
                name: "S5".to_string(),
                description: "S5".to_string(),
            },
        ],
        inputs: vec![],
        source: Some(ProviderSource::Table(TableReference {
            table: Some(TableRef::File(FileReference {
                path,
                format: FileFormat::Parquet as i32,
            })),
        })),
    }
}

pub fn get_provider_2() -> SignalProvider {
    let mut path = workspace_root().unwrap();
    path.push_str("/test/data/P2.parquet");
    SignalProvider {
        uid: "provider-2".to_string(),
        name: "P2".to_string(),
        description: "first test provider".to_string(),
        signals: vec![
            Signal {
                uid: "id-S6".to_string(),
                name: "S6".to_string(),
                description: "S6".to_string(),
            },
            Signal {
                uid: "id-S7".to_string(),
                name: "S7".to_string(),
                description: "S7".to_string(),
            },
        ],
        inputs: vec![],
        source: Some(ProviderSource::Table(TableReference {
            table: Some(TableRef::File(FileReference {
                path,
                format: FileFormat::Parquet as i32,
            })),
        })),
    }
}

pub fn get_signal_frame() -> SignalFrame {
    let source_p1 = get_provider_1();
    let source_p2 = get_provider_2();

    let expression_provider = SignalProvider {
        uid: "expression-provider-id".to_string(),
        name: "S4".to_string(),
        description: "description".to_string(),
        signals: vec![Signal {
            uid: "expr-id".to_string(),
            name: "S4".to_string(),
            description: "description".to_string(),
        }],
        inputs: vec![
            Signal {
                uid: "id-S5".to_string(),
                name: "S5".to_string(),
                description: "S5".to_string(),
            },
            Signal {
                uid: "id-S6".to_string(),
                name: "S6".to_string(),
                description: "S6".to_string(),
            },
        ],
        source: Some(ProviderSource::Expression(ExpressionReference {
            expression: "S5 * 2 + S6".to_string(),
            ..ExpressionReference::default()
        })),
    };

    SignalFrame {
        uid: "frame-id".to_string(),
        name: "frame".to_string(),
        description: "description".to_string(),
        providers: vec![source_p1, source_p2, expression_provider],
    }
}
