use crate::error::{FusionPlannerError, Result};
use arrow_deps::datafusion::{
    datasource::{
        file_format::parquet::ParquetFormat,
        listing::{ListingOptions, ListingTable},
        object_store::local::LocalFileSystem,
        TableProvider,
    },
    logical_plan::{DFSchema, DFSchemaRef, Expr, JoinType, LogicalPlan, LogicalPlanBuilder},
    physical_plan::ExecutionPlan,
    prelude::ExecutionContext,
    sql::{
        parser::{DFParser, Statement},
        planner::SqlToRel,
    },
};
use flight_fusion_ipc::{
    signal_provider::Source as ProviderSource, table_reference::Table as TableRef, SignalProvider,
};
use sqlparser::ast::{Query, Select, SelectItem, SetExpr, Statement as SQLStatement};
use std::sync::{Arc, Mutex};

pub enum ProviderNode {
    Table(Arc<dyn TableProvider>),
    Expression(Expr),
    Model(Arc<dyn ExecutionPlan>),
}

#[derive(Debug)]
pub enum QueryTreeNode {
    Expression(String),
    SourceTable(SourceTableInfo),
    Model(String),
    Signal(String),
}

#[derive(Debug)]
pub struct SourceTableInfo {
    pub catalog: String,
    pub schema: String,
    pub table: String,
    pub alias: String,
}

impl SourceTableInfo {
    pub fn table_ref(&self) -> String {
        format!("{}.{}.{}", self.catalog, self.schema, self.table)
    }
}

pub struct FrameQueryPlanner {
    ctx: ExecutionContext,
    plan: Option<LogicalPlan>,
}

impl Default for FrameQueryPlanner {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameQueryPlanner {
    pub fn new() -> Self {
        Self {
            ctx: ExecutionContext::new(),
            plan: None,
        }
    }

    pub async fn register_signal_provider(&mut self, provider: &SignalProvider) -> Result<()> {
        match self.parse_provider(provider).await? {
            ProviderNode::Table(tbl) => {
                self.ctx
                    .register_table(provider.name.as_str(), tbl.clone())?;
                // TODO add filters / projection based on defined signals
                let scan_plan = LogicalPlanBuilder::scan(provider.name.as_str(), tbl, None)?;
                if let Some(plan) = &self.plan {
                    let builder = LogicalPlanBuilder::from(plan.clone()).join(
                        &scan_plan.build()?,
                        JoinType::Inner,
                        (vec!["timestamp"], vec!["timestamp"]),
                    )?;
                    self.plan = Some(builder.build()?);
                    Ok(())
                } else {
                    self.plan = Some(scan_plan.build()?);
                    Ok(())
                }
            }
            ProviderNode::Expression(expr) => {
                if let Some(plan) = &self.plan {
                    let builder = LogicalPlanBuilder::from(plan.clone()).project(vec![expr])?;
                    self.plan = Some(builder.build()?);
                    return Ok(());
                }
                Err(FusionPlannerError::PlanningError(
                    "Relevant sources must be registered before registering expressions"
                        .to_string(),
                ))
            }
            _ => todo!(),
        }
    }

    async fn parse_provider(&self, provider: &SignalProvider) -> Result<ProviderNode> {
        match &provider.source {
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
                _ => todo!(),
            },
            Some(ProviderSource::Expression(expr)) => {
                assert!(
                    provider.signals.len() == 1,
                    "Expressions need exactly one output signal"
                );
                let eqn = format!("SELECT {} as {}", expr.expression, provider.signals[0].name);
                let schema = self
                    .schema()
                    .expect("Sources must be registered before expressions");
                let expr = self.convert_expression(eqn.as_str(), &schema)?;
                Ok(ProviderNode::Expression(expr))
            }
            _ => Err(FusionPlannerError::PlanningError(
                "Only signal provider with table source can be converted to TableProvider"
                    .to_string(),
            )),
        }
    }

    pub fn generate_logical_plan(&self) {
        todo!()
    }

    pub fn schema(&self) -> Option<DFSchemaRef> {
        self.plan.clone().map(|p| p.schema().clone())
    }

    pub fn convert_expression(&self, expr: &str, schema: &DFSchema) -> Result<Expr> {
        let statement = DFParser::new(expr).unwrap().parse_statement().unwrap();
        let select_items = match statement {
            Statement::Statement(stmt) => match *stmt {
                SQLStatement::Query(query) => {
                    let Query { body, .. } = *query;
                    match body {
                        SetExpr::Select(select) => {
                            let Select { projection, .. } = *select;
                            projection
                        }
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            },
            _ => todo!(),
        };

        if select_items.len() != 1 {
            return Err(FusionPlannerError::PlanningError(
                "Expected exactly one select item.".to_string(),
            ));
        };

        let expression = match &select_items[0] {
            SelectItem::UnnamedExpr(expr) => Ok(expr.clone()),
            SelectItem::ExprWithAlias { expr, .. } => Ok(expr.clone()),
            _ => Err(FusionPlannerError::PlanningError(
                "unexpected parsing result.".to_string(),
            )),
        }?;
        let state = self.ctx.state.lock().unwrap().clone();
        let query_planner = SqlToRel::new(&state);
        let log_expr = query_planner.sql_to_rex(&expression, schema)?;

        Ok(log_expr)
    }

    pub async fn create_physical_plan(&self) -> Result<Arc<dyn ExecutionPlan>> {
        let plan = self.ctx.optimize(&self.plan.clone().unwrap()).unwrap();
        let state = self.ctx.state.lock().unwrap().clone();
        let ctx = ExecutionContext::from(Arc::new(Mutex::new(state)));
        let phys_plan = ctx.create_physical_plan(&plan).await?;
        Ok(phys_plan)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow_deps::arrow::util::pretty;
    use arrow_deps::datafusion::physical_plan::collect;
    use flight_fusion_ipc::{ExpressionReference, Signal};

    #[tokio::test]
    async fn test_register_provider() {
        let source_provider = crate::test_utils::get_provider_1();
        let mut planner = FrameQueryPlanner::new();
        planner
            .register_signal_provider(&source_provider)
            .await
            .unwrap();

        let expression_provider = SignalProvider {
            uid: "expression-provider-id".to_string(),
            name: "expression".to_string(),
            description: "description".to_string(),
            signals: vec![Signal {
                uid: "expr-id".to_string(),
                name: "S4".to_string(),
                description: "description".to_string(),
            }],
            inputs: vec![],
            source: Some(ProviderSource::Expression(ExpressionReference {
                expression: "S2 * 2 + S5".to_string(),
                ..ExpressionReference::default()
            })),
        };

        planner
            .register_signal_provider(&expression_provider)
            .await
            .unwrap();

        let plan = planner.create_physical_plan().await.unwrap();

        let results = collect(plan.clone()).await.unwrap();
        println!("{:?}", results[0])
    }

    #[tokio::test]
    async fn test_register_provider_multiple() {
        let source_provider = crate::test_utils::get_provider_1();
        let source_provider2 = crate::test_utils::get_provider_2();

        let mut planner = FrameQueryPlanner::new();
        planner
            .register_signal_provider(&source_provider)
            .await
            .unwrap();
        planner
            .register_signal_provider(&source_provider2)
            .await
            .unwrap();

        let expression_provider = SignalProvider {
            uid: "expression-provider-id".to_string(),
            name: "expression".to_string(),
            description: "description".to_string(),
            signals: vec![Signal {
                uid: "expr-id".to_string(),
                name: "S4".to_string(),
                description: "description".to_string(),
            }],
            inputs: vec![],
            source: Some(ProviderSource::Expression(ExpressionReference {
                expression: "S2 * 2 + S5 + S6".to_string(),
                ..ExpressionReference::default()
            })),
        };

        planner
            .register_signal_provider(&expression_provider)
            .await
            .unwrap();

        let plan = planner.create_physical_plan().await.unwrap();

        let results = collect(plan.clone()).await.unwrap();
        pretty::print_batches(&results).unwrap();
    }

    #[tokio::test]
    async fn test() {
        let planner = FrameQueryPlanner::new();
        let expr = "SELECT P1.S2 + P2.S6";
        let schema = planner.schema().unwrap();
        planner.convert_expression(expr, &schema).unwrap();
    }
}
