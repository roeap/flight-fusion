//! Utilities for writing unit tests
// use super::*;
// use crate::{
//     action::Protocol, DeltaTable, DeltaTableConfig, DeltaTableMetaData, SchemaDataType, SchemaField,
// };
use crate::{handlers::FusionActionHandler, service::BoxedFlightStream};
use arrow_deps::datafusion::arrow::{
    array::{Array, Float32Array, Float64Array, Int32Array, Int64Array, StringArray, UInt32Array},
    compute::take,
    datatypes::{DataType, Field, Schema as ArrowSchema, SchemaRef as ArrowSchemaRef},
    ipc::writer::IpcWriteOptions,
    record_batch::RecordBatch,
};
use arrow_flight::{FlightData, SchemaAsIpc};
use rand::distributions::Standard;
use rand::prelude::*;
use std::sync::Arc;
use tonic::Status;

pub fn generate_random_batch(row_count: usize, schema: ArrowSchemaRef) -> RecordBatch {
    let mut arrays: Vec<Arc<dyn Array>> = vec![];
    for field in schema.fields() {
        match field.data_type() {
            DataType::Float64 => {
                arrays.push(Arc::new(Float64Array::from(generate_values(&mut vec![
                    1_f64;
                    row_count
                ]))))
            }
            DataType::Float32 => {
                arrays.push(Arc::new(Float32Array::from(generate_values(&mut vec![
                    1_f32;
                    row_count
                ]))))
            }
            DataType::Int64 => arrays.push(Arc::new(Int64Array::from(generate_values(&mut vec![
                    1_i64;
                    row_count
                ])))),
            DataType::Int32 => arrays.push(Arc::new(Int32Array::from(generate_values(&mut vec![
                    1_i32;
                    row_count
                ])))),
            _ => todo!(),
        };
    }

    RecordBatch::try_new(schema, arrays).unwrap()
}

fn generate_values<T: Clone>(raw: &mut Vec<T>) -> Vec<T>
where
    Standard: rand::distributions::Distribution<T>,
{
    let mut rng = rand::thread_rng();
    for x in raw.iter_mut() {
        *x = rng.gen::<T>();
    }
    raw.to_vec()
}

pub fn get_record_batch_stream() -> BoxedFlightStream<FlightData> {
    let batch = get_record_batch(None, false);
    let results = vec![batch.clone()];
    let options = IpcWriteOptions::default();
    let schema_flight_data: FlightData = SchemaAsIpc::new(&batch.schema().clone(), &options).into();

    let mut flights: Vec<Result<FlightData, Status>> = vec![Ok(schema_flight_data)];
    let mut batches: Vec<Result<FlightData, Status>> = results
        .iter()
        .flat_map(|batch| {
            let (flight_dictionaries, flight_batch) =
                arrow_flight::utils::flight_data_from_arrow_batch(batch, &options);
            flight_dictionaries
                .into_iter()
                .chain(std::iter::once(flight_batch))
                .map(Ok)
        })
        .collect();

    flights.append(&mut batches);

    Box::pin(futures::stream::iter(flights)) as BoxedFlightStream<FlightData>
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

pub fn get_fusion_handler() -> FusionActionHandler {
    let handler = FusionActionHandler::new();
    handler
}

// pub fn get_delta_schema() -> Schema {
//     Schema::new(vec![
//         SchemaField::new(
//             "id".to_string(),
//             SchemaDataType::primitive("string".to_string()),
//             true,
//             HashMap::new(),
//         ),
//         SchemaField::new(
//             "value".to_string(),
//             SchemaDataType::primitive("integer".to_string()),
//             true,
//             HashMap::new(),
//         ),
//         SchemaField::new(
//             "modified".to_string(),
//             SchemaDataType::primitive("string".to_string()),
//             true,
//             HashMap::new(),
//         ),
//     ])
// }

// pub fn get_delta_metadata(partition_cols: &[String]) -> DeltaTableMetaData {
//     let table_schema = get_delta_schema();
//     DeltaTableMetaData::new(
//         None,
//         None,
//         None,
//         table_schema,
//         partition_cols.to_vec(),
//         HashMap::new(),
//     )
// }

// pub fn create_bare_table() -> DeltaTable {
//     let table_dir = tempfile::tempdir().unwrap();
//     let table_path = table_dir.path();
//     let backend = Box::new(crate::storage::file::FileStorageBackend::new(
//         table_path.to_str().unwrap(),
//     ));
//     DeltaTable::new(
//         table_path.to_str().unwrap(),
//         backend,
//         DeltaTableConfig::default(),
//     )
//     .unwrap()
// }

// pub async fn create_initialized_table(partition_cols: &[String]) -> DeltaTable {
//     let mut table = create_bare_table();
//     let table_schema = get_delta_schema();
//
//     let mut commit_info = serde_json::Map::<String, serde_json::Value>::new();
//     commit_info.insert(
//         "operation".to_string(),
//         serde_json::Value::String("CREATE TABLE".to_string()),
//     );
//     commit_info.insert(
//         "userName".to_string(),
//         serde_json::Value::String("test user".to_string()),
//     );
//
//     let protocol = Protocol {
//         min_reader_version: 1,
//         min_writer_version: 2,
//     };
//
//     let metadata = DeltaTableMetaData::new(
//         None,
//         None,
//         None,
//         table_schema,
//         partition_cols.to_vec(),
//         HashMap::new(),
//     );
//
//     table
//         .create(metadata, protocol, Some(commit_info))
//         .await
//         .unwrap();
//
//     table
// }
