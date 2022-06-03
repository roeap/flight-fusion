//! Utilities for writing unit tests
use crate::service::FlightFusionService;
use arrow_deps::datafusion::{
    arrow::{
        array::{Int32Array, StringArray, UInt32Array},
        compute::take,
        datatypes::{
            DataType, Field, Schema as ArrowSchema, SchemaRef as ArrowSchemaRef, TimeUnit,
        },
        record_batch::RecordBatch,
    },
    physical_plan::{
        common::SizedRecordBatchStream,
        metrics::{ExecutionPlanMetricsSet, MemTrackingMetrics},
        SendableRecordBatchStream,
    },
};
use std::{path::PathBuf, sync::Arc};

pub fn workspace_test_data_folder() -> PathBuf {
    let ws_root = crate::test_utils::workspace_root().unwrap();
    let ws_root = std::path::Path::new(&ws_root);
    ws_root.join("test")
}

pub fn get_fusion_handler(root: impl Into<PathBuf>) -> FlightFusionService {
    FlightFusionService::new_default(root).unwrap()
}

pub fn get_test_data_fusion_handler() -> FlightFusionService {
    let area_root = workspace_test_data_folder().join("db");
    FlightFusionService::new_default(area_root).unwrap()
}

pub fn get_test_data_schema() -> ArrowSchemaRef {
    Arc::new(ArrowSchema::new(vec![
        Field::new(
            "timestamp",
            DataType::Timestamp(TimeUnit::Microsecond, None),
            true,
        ),
        Field::new("date", DataType::Date32, true),
        Field::new("string", DataType::Utf8, true),
        Field::new("double", DataType::Float64, true),
        Field::new("real", DataType::Float64, true),
        Field::new("float", DataType::Float64, true),
    ]))
}

/// Run cargo to get the root of the workspace
pub fn workspace_root() -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("cargo")
        .arg("metadata")
        .arg("--no-deps")
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

pub fn get_input_stream(part: Option<String>, with_null: bool) -> SendableRecordBatchStream {
    let batch = Arc::new(get_record_batch(part, with_null));
    let schema = batch.schema();
    let metrics = ExecutionPlanMetricsSet::new();
    let tracking_metrics = MemTrackingMetrics::new(&metrics, 0);
    Box::pin(SizedRecordBatchStream::new(
        schema,
        vec![batch],
        tracking_metrics,
    ))
}

pub fn get_record_batch(part: Option<String>, with_null: bool) -> RecordBatch {
    let (base_int, base_str, base_mod) = if with_null {
        data_with_null()
    } else {
        data_without_null()
    };

    let indices = match &part {
        Some(key) if key == "modified=2021-02-01" => {
            UInt32Array::from(vec![3, 4, 5, 6, 7, 8, 9, 10])
        }
        Some(key) if key == "modified=2021-02-01/id=A" => UInt32Array::from(vec![4, 5, 6, 9, 10]),
        Some(key) if key == "modified=2021-02-01/id=B" => UInt32Array::from(vec![3, 7, 8]),
        Some(key) if key == "modified=2021-02-02" => UInt32Array::from(vec![0, 1, 2]),
        Some(key) if key == "modified=2021-02-02/id=A" => UInt32Array::from(vec![0, 2]),
        Some(key) if key == "modified=2021-02-02/id=B" => UInt32Array::from(vec![1]),
        _ => UInt32Array::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
    };

    let int_values = take(&base_int, &indices, None).unwrap();
    let str_values = take(&base_str, &indices, None).unwrap();
    let mod_values = take(&base_mod, &indices, None).unwrap();

    match &part {
        Some(key) if key.contains("/id=") => {
            let schema = Arc::new(ArrowSchema::new(vec![Field::new(
                "value",
                DataType::Int32,
                true,
            )]));
            RecordBatch::try_new(schema, vec![int_values]).unwrap()
        }
        Some(_) => {
            let schema = Arc::new(ArrowSchema::new(vec![
                Field::new("id", DataType::Utf8, true),
                Field::new("value", DataType::Int32, true),
            ]));
            RecordBatch::try_new(schema, vec![str_values, int_values]).unwrap()
        }
        _ => {
            let schema = Arc::new(ArrowSchema::new(vec![
                Field::new("id", DataType::Utf8, true),
                Field::new("value", DataType::Int32, true),
                Field::new("modified", DataType::Utf8, true),
            ]));
            RecordBatch::try_new(schema, vec![str_values, int_values, mod_values]).unwrap()
        }
    }
}

fn data_with_null() -> (Int32Array, StringArray, StringArray) {
    let base_int = Int32Array::from(vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        None,
        Some(7),
        Some(8),
        Some(9),
        Some(10),
        Some(11),
    ]);
    let base_str = StringArray::from(vec![
        Some("A"),
        Some("B"),
        None,
        Some("B"),
        Some("A"),
        Some("A"),
        None,
        None,
        Some("B"),
        Some("A"),
        Some("A"),
    ]);
    let base_mod = StringArray::from(vec![
        Some("2021-02-02"),
        Some("2021-02-02"),
        Some("2021-02-02"),
        Some("2021-02-01"),
        Some("2021-02-01"),
        Some("2021-02-01"),
        Some("2021-02-01"),
        Some("2021-02-01"),
        Some("2021-02-01"),
        Some("2021-02-01"),
        Some("2021-02-01"),
    ]);

    (base_int, base_str, base_mod)
}

fn data_without_null() -> (Int32Array, StringArray, StringArray) {
    let base_int = Int32Array::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    let base_str = StringArray::from(vec!["A", "B", "A", "B", "A", "A", "A", "B", "B", "A", "A"]);
    let base_mod = StringArray::from(vec![
        "2021-02-02",
        "2021-02-02",
        "2021-02-02",
        "2021-02-01",
        "2021-02-01",
        "2021-02-01",
        "2021-02-01",
        "2021-02-01",
        "2021-02-01",
        "2021-02-01",
        "2021-02-01",
    ]);

    (base_int, base_str, base_mod)
}
