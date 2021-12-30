use arrow_deps::datafusion::{
    catalog::schema::SchemaProvider,
    datasource::{
        file_format::parquet::ParquetFormat,
        listing::{ListingOptions, ListingTable},
        object_store::local::LocalFileSystem,
        TableProvider,
    },
    error::Result as DataFusionResult,
    physical_plan::{ExecutionPlan, PhysicalExpr},
};
use async_trait::async_trait;
use flight_fusion_ipc::{
    errors::FlightFusionError, errors::Result as FusionResult,
    signal_provider::Source as ProviderSource, table_reference::Table as TableRef, SignalFrame,
    SignalProvider,
};
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub mod error;

pub enum ProviderNode {
    Table(Arc<dyn TableProvider>),
    Expression(Arc<dyn PhysicalExpr>),
    Model(Arc<dyn ExecutionPlan>),
}

#[async_trait]
pub trait ToProviderNode {
    async fn try_into_provider_node(&self) -> FusionResult<ProviderNode>;
}

#[async_trait]
impl ToProviderNode for SignalProvider {
    async fn try_into_provider_node(&self) -> FusionResult<ProviderNode> {
        match &self.source {
            Some(ProviderSource::Table(tbl_ref)) => match &tbl_ref.table {
                Some(TableRef::File(file)) => {
                    let opt = ListingOptions {
                        file_extension: "parquet".to_owned(),
                        format: Arc::new(ParquetFormat::default()),
                        table_partition_cols: vec![],
                        target_partitions: 1,
                        collect_stat: true,
                    };
                    // here we resolve the schema locally
                    let schema = opt
                        .infer_schema(Arc::new(LocalFileSystem {}), &file.path)
                        .await
                        .expect("Infer schema");
                    let table = ListingTable::new(
                        Arc::new(LocalFileSystem {}),
                        file.path.clone(),
                        schema,
                        opt,
                    );
                    Ok(ProviderNode::Table(Arc::new(table)))
                }
                Some(TableRef::Delta(_delta)) => todo!(),
                _ => todo!(),
            },
            _ => Err(FlightFusionError::InputError(
                "Only signal provider with table source can be converted to TableProvider"
                    .to_string(),
            )),
        }
    }
}

pub struct SignalFrameContext {
    frame: SignalFrame,
    tables: RwLock<HashMap<String, Arc<dyn TableProvider>>>,
}

impl SignalFrameContext {
    pub async fn try_new(frame: SignalFrame) -> FusionResult<Self> {
        let mut tables = HashMap::new();

        for provider in frame.clone().providers {
            match provider.try_into_provider_node().await? {
                ProviderNode::Table(tbl) => {
                    tables.insert(provider.name.clone(), tbl.clone());
                }
                _ => (),
            }
        }

        Ok(Self {
            frame,
            tables: RwLock::new(tables),
        })
    }
}

impl SchemaProvider for SignalFrameContext {
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Retrieves the list of available table names in this schema.
    fn table_names(&self) -> Vec<String> {
        let tables = self.tables.read().unwrap();
        tables.keys().cloned().collect()
    }

    /// Retrieves a specific table from the schema by name, provided it exists.
    fn table(&self, name: &str) -> Option<Arc<dyn TableProvider>> {
        let tables = self.tables.read().unwrap();
        tables.get(name).cloned()
    }

    fn deregister_table(&self, name: &str) -> DataFusionResult<Option<Arc<dyn TableProvider>>> {
        let mut tables = self.tables.write().unwrap();
        Ok(tables.remove(name))
    }

    fn table_exist(&self, name: &str) -> bool {
        let tables = self.tables.read().unwrap();
        tables.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow_deps::datafusion::sql::parser::DFParser;
    use flight_fusion_ipc::{
        signal_provider::Source as ProviderSource, table_reference::Table as TableRef, FileFormat,
        FileReference, Signal, SignalProvider, TableReference,
    };

    use crate::ToProviderNode;

    #[tokio::test]
    async fn provider_node_conversion() {
        let provider = get_provider();
        let node = provider.try_into_provider_node().await.unwrap();
        assert!(matches!(node, ProviderNode::Table(_)))
    }

    #[tokio::test]
    async fn create_catalog() {
        let provider = get_provider();
        let frame = SignalFrame {
            uid: "frame-id".to_string(),
            name: "frame".to_string(),
            description: "description".to_string(),
            providers: vec![provider]
        };
        let context = SignalFrameContext::try_new(frame).await.unwrap();
        assert!(context.table_exist("provider"))
    }

    #[test]
    fn test_sql_parser() {
        let sql = "SELECT c = a + b FROM tblref";
        let statements = DFParser::parse_sql(sql).unwrap();
        println!("{:#?}", statements[0])
    }

    fn get_provider() -> SignalProvider {
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
}
