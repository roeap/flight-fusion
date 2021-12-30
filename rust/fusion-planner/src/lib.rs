use arrow_deps::datafusion::{
    catalog::{
        catalog::MemoryCatalogProvider,
        schema::{MemorySchemaProvider, SchemaProvider},
    },
    datasource::{
        file_format::parquet::ParquetFormat,
        listing::{ListingOptions, ListingTable},
        object_store::local::LocalFileSystem,
        TableProvider,
    },
    physical_plan::{ExecutionPlan, PhysicalExpr},
    prelude::{ExecutionConfig, ExecutionContext},
};
use async_trait::async_trait;
use flight_fusion_ipc::{
    errors::FlightFusionError, errors::Result as FusionResult,
    signal_provider::Source as ProviderSource, table_reference::Table as TableRef, SignalFrame,
    SignalProvider,
};
use std::sync::Arc;

pub mod error;
pub mod frames;
pub mod test_utils;

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
}

impl SignalFrameContext {
    pub fn new(frame: SignalFrame) -> Self {
        Self { frame }
    }

    pub async fn into_query_context(&self) -> FusionResult<ExecutionContext> {
        let schema_provider = MemorySchemaProvider::new();

        for provider in self.frame.clone().providers {
            match provider.try_into_provider_node().await? {
                ProviderNode::Table(tbl) => {
                    // TODO remove panic
                    schema_provider
                        .register_table(provider.name.clone(), tbl.clone())
                        .unwrap();
                }
                _ => (),
            }
        }

        let catalog = Arc::new(MemoryCatalogProvider::new());
        catalog.register_schema(self.frame.name.clone(), Arc::new(schema_provider));

        let config = ExecutionConfig::new().with_information_schema(true);
        let ctx = ExecutionContext::with_config(config);
        ctx.register_catalog("catalog", catalog);

        Ok(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ToProviderNode, test_utils::get_provider};
    use arrow_deps::datafusion::sql::parser::DFParser;

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
            providers: vec![provider],
        };
        let context = SignalFrameContext::new(frame.clone());
        let mut ctx = context.into_query_context().await.unwrap();

        let sql = "select * from information_schema.columns";
        let df = ctx.sql(sql).await.unwrap();

        df.show().await.unwrap();
    }

    #[test]
    fn test_sql_parser() {
        let sql = "SELECT c = a + b FROM tblref";
        let statements = DFParser::parse_sql(sql).unwrap();
        println!("{:#?}", statements[0])
    }
}
