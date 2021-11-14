use std::sync::Arc;

use datafusion::arrow::array::{Int32Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::arrow::record_batch::RecordBatch;

use datafusion::catalog::catalog::MemoryCatalogProvider;
use datafusion::catalog::schema::{MemorySchemaProvider, SchemaProvider};
use datafusion::datasource::MemTable;
use datafusion::error::Result;
use datafusion::prelude::*;

/// This example demonstrates how to use the DataFrame API against in-memory data.
#[tokio::main]
async fn main() -> Result<()> {
    // define a schema.
    let schema = Arc::new(Schema::new(vec![
        Field::new("a", DataType::Utf8, false),
        Field::new("b", DataType::Int32, false),
    ]));

    // define data.
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(vec!["a", "b", "c", "d"])),
            Arc::new(Int32Array::from(vec![1, 10, 10, 100])),
        ],
    )?;

    // declare a table in memory. In spark API, this corresponds to createDataFrame(...).
    let table_provider = MemTable::try_new(schema, vec![vec![batch]])?;

    let schema_provider = MemorySchemaProvider::new();
    schema_provider.register_table("table".to_string(), Arc::new(table_provider))?;

    let catalog = Arc::new(MemoryCatalogProvider::new());
    catalog.register_schema("schema".to_string(), Arc::new(schema_provider));

    // declare a new context. In spark API, this corresponds to a new spark SQLsession
    let mut ctx = ExecutionContext::new();
    ctx.register_catalog("catalog", catalog);

    let sql = "SELECT * FROM catalog.schema.table";
    let df = ctx.sql(sql).await?;

    // print the results
    df.show().await?;

    Ok(())
}
