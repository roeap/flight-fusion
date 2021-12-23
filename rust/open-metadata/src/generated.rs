// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type Filters = Option<serde_json::Value>;
pub type Basic = Option<serde_json::Value>;
pub type Storage = Option<serde_json::Value>;

/// This schema defines the Pipeline entity. A pipeline enables the flow of data from source
/// to destination through a series of processing steps. ETL is a type of pipeline where the
/// series of steps Extract, Transform and Load the data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pipeline {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Concurrency of the Pipeline.
    #[serde(rename = "concurrency")]
    pub concurrency: Option<i64>,

    /// Description of this Pipeline.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this Pipeline. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Followers of this Pipeline.
    #[serde(rename = "followers")]
    pub followers: Option<Vec<ServiceElement>>,

    /// A unique name that identifies a pipeline in the format 'ServiceName.PipelineName'.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies a pipeline instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Name that identifies this pipeline instance uniquely.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this pipeline.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Pipeline Code Location.
    #[serde(rename = "pipelineLocation")]
    pub pipeline_location: Option<String>,

    /// Pipeline  URL to visit/manage. This URL points to respective pipeline service UI.
    #[serde(rename = "pipelineUrl")]
    pub pipeline_url: Option<String>,

    /// Link to service where this pipeline is hosted in.
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Service type where this pipeline is hosted in.
    #[serde(rename = "serviceType")]
    pub service_type: Option<PipelineServiceType>,

    /// Start date of the workflow.
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,

    /// Tags for this Pipeline.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// All the tasks that are part of pipeline.
    #[serde(rename = "tasks")]
    pub tasks: Option<Vec<Task>>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// Change that lead to this version of the entity.
///
/// Description of the change.
///
/// For `eventType` `entityUpdated` this field captures details about what fields were
/// added/updated/deleted. For `eventType` `entityCreated` or `entityDeleted` this field is
/// null.
///
/// Change that led to this version of the entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineChangeDescription {
    /// Names of fields added during the version changes.
    #[serde(rename = "fieldsAdded")]
    pub fields_added: Option<Vec<PurpleFieldChange>>,

    /// Fields deleted during the version changes with old value before deleted.
    #[serde(rename = "fieldsDeleted")]
    pub fields_deleted: Option<Vec<PurpleFieldChange>>,

    /// Fields modified during the version changes with old and new values.
    #[serde(rename = "fieldsUpdated")]
    pub fields_updated: Option<Vec<PurpleFieldChange>>,

    /// When a change did not result in change, this could be same as the current version.
    #[serde(rename = "previousVersion")]
    pub previous_version: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PurpleFieldChange {
    /// Name of the entity field that changed.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// New value of the field. Note that this is a JSON string and use the corresponding field
    /// type to deserialize it.
    #[serde(rename = "newValue")]
    pub new_value: Option<serde_json::Value>,

    /// Previous value of the field. Note that this is a JSON string and use the corresponding
    /// field type to deserialize it.
    #[serde(rename = "oldValue")]
    pub old_value: Option<serde_json::Value>,
}

/// Followers of this Pipeline.
///
/// This schema defines the EntityReference type used for referencing an entity.
/// EntityReference is used for capturing relationships from one entity to another. For
/// example, a table has an attribute called database of type EntityReference that captures
/// the relationship of a table `belongs to a` database.
///
/// Owner of this pipeline.
///
/// Link to service where this pipeline is hosted in.
///
/// Reference to Database that contains this table.
///
/// Reference to the Location that contains this table.
///
/// Owner of this table.
///
/// Link to Database service this table is hosted in.
///
/// User who ran this query.
///
/// Performance Dashboard URL to track metric evolution.
///
/// Owner of this ML Model.
///
/// Reference to the Location that contains this database.
///
/// Owner of this database.
///
/// Link to the database cluster/service where this database is hosted in.
///
/// Owner of this dashboard.
///
/// Link to service where this dashboard is hosted in.
///
/// Owner of this location.
///
/// Owner of this metrics.
///
/// Link to service where this metrics is hosted in.
///
/// Owner of this topic.
///
/// Link to the messaging cluster/service where this topic is hosted in.
///
/// Link to service where this report is hosted in.
///
/// Primary entity for which this lineage graph is created.
///
/// Entity for which usage is returned.
///
/// Owner of this Ingestion.
///
/// Link to the database service where this database is hosted in.
///
/// Owner of this database
///
/// Link to the database service where this database is hosted in
///
/// Performance Dashboard URL to track metric evolution
///
/// Owner of this Location
///
/// Link to the pipeline service where this location is used
///
/// From entity that is upstream of lineage edge.
///
/// To entity that is downstream of lineage edge.
///
/// Owner of this Policy.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceElement {
    /// Optional description of entity.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this entity.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Link to the entity resource.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies an entity instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Name of the entity instance. For entities such as tables, databases where the name is not
    /// unique, fullyQualifiedName is returned in this field.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Entity type/class name - Examples: `database`, `table`, `metrics`, `databaseService`,
    /// `dashboardService`...
    #[serde(rename = "type")]
    pub entity_reference_type: String,
}

/// This schema defines the type for labeling an entity with a Tag.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagElement {
    /// Unique name of the tag category.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Link to the tag resource.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Label type describes how a tag label was applied. 'Manual' indicates the tag label was
    /// applied by a person. 'Derived' indicates a tag label was derived using the associated tag
    /// relationship (see TagCategory.json for more details). 'Propagated` indicates a tag label
    /// was propagated from upstream based on lineage. 'Automated' is used when a tool was used
    /// to determine the tag label.
    #[serde(rename = "labelType")]
    pub label_type: LabelType,

    /// 'Suggested' state is used when a tag label is suggested by users or tools. Owner of the
    /// entity must confirm the suggested labels before it is marked as 'Confirmed'.
    #[serde(rename = "state")]
    pub state: State,

    #[serde(rename = "tagFQN")]
    pub tag_fqn: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    /// Description of this Task.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this Task. It could be title or label from the pipeline
    /// services.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// All the tasks that are downstream of this task.
    #[serde(rename = "downstreamTasks")]
    pub downstream_tasks: Option<Vec<String>>,

    /// A unique name that identifies a pipeline in the format
    /// 'ServiceName.PipelineName.TaskName'.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Name that identifies this task instance uniquely.
    #[serde(rename = "name")]
    pub name: String,

    /// Tags for this task.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// SQL used in the task. Can be used to determine the lineage.
    #[serde(rename = "taskSQL")]
    pub task_sql: Option<String>,

    /// Type of the Task. Usually refers to the class it implements.
    #[serde(rename = "taskType")]
    pub task_type: Option<String>,

    /// Task URL to visit/manage. This URL points to respective pipeline service UI.
    #[serde(rename = "taskUrl")]
    pub task_url: Option<String>,

    #[serde(rename = "id")]
    pub id: Option<serde_json::Value>,
}

/// This schema defines the Table entity. A Table organizes data in rows and columns and is
/// defined by a Schema. OpenMetadata does not have a separate abstraction for Schema. Both
/// Table and Schema are captured in this entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Columns in this table.
    #[serde(rename = "columns")]
    pub columns: Vec<TableColumn>,

    /// Reference to Database that contains this table.
    #[serde(rename = "database")]
    pub database: Option<ServiceElement>,

    /// This captures information about how the table is modeled. Currently only DBT model is
    /// supported.
    #[serde(rename = "dataModel")]
    pub data_model: Option<DataModel>,

    /// Description of a table.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this table. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Followers of this table.
    #[serde(rename = "followers")]
    pub followers: Option<Vec<ServiceElement>>,

    /// Fully qualified name of a table in the form `serviceName.databaseName.tableName`.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to this table resource.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier of this table instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Details of other tables this table is frequently joined with.
    #[serde(rename = "joins")]
    pub joins: Option<TableJoins>,

    /// Reference to the Location that contains this table.
    #[serde(rename = "location")]
    pub location: Option<ServiceElement>,

    /// Name of a table. Expected to be unique within a database.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this table.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Sample data for a table.
    #[serde(rename = "sampleData")]
    pub sample_data: Option<TableData>,

    /// Link to Database service this table is hosted in.
    #[serde(rename = "service")]
    pub service: Option<ServiceElement>,

    /// Service type this table is hosted in.
    #[serde(rename = "serviceType")]
    pub service_type: Option<DatabaseServiceType>,

    /// Table constraints.
    #[serde(rename = "tableConstraints")]
    pub table_constraints: Option<Vec<TableTableConstraint>>,

    /// Data profile for a table.
    #[serde(rename = "tableProfile")]
    pub table_profile: Option<Vec<TableProfile>>,

    /// List of queries that ran against a table.
    #[serde(rename = "tableQueries")]
    pub table_queries: Option<Vec<SqlQuery>>,

    #[serde(rename = "tableType")]
    pub table_type: Option<TableType>,

    /// Tags for this table.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Latest usage information for this table.
    #[serde(rename = "usageSummary")]
    pub usage_summary: Option<UsageSummaryElement>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,

    /// View Definition in SQL. Applies to TableType.View only.
    #[serde(rename = "viewDefinition")]
    pub view_definition: Option<String>,
}

/// This schema defines the type for a column in a table.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableColumn {
    /// Data type used array in dataType. For example, `array<int>` has dataType as `array` and
    /// arrayDataType as `int`.
    #[serde(rename = "arrayDataType")]
    pub array_data_type: Option<DataType>,

    /// Child columns if dataType or arrayDataType is `map`, `struct`, or `union` else `null`.
    #[serde(rename = "children")]
    pub children: Option<Vec<TableColumn>>,

    /// Column level constraint.
    #[serde(rename = "constraint")]
    pub constraint: Option<Constraint>,

    /// Length of `char`, `varchar`, `binary`, `varbinary` `dataTypes`, else null. For example,
    /// `varchar(20)` has dataType as `varchar` and dataLength as `20`.
    #[serde(rename = "dataLength")]
    pub data_length: Option<i64>,

    /// Data type of the column (int, date etc.).
    #[serde(rename = "dataType")]
    pub data_type: DataType,

    /// Display name used for dataType. This is useful for complex types, such as `array<int>,
    /// map<int,string>, struct<>, and union types.
    #[serde(rename = "dataTypeDisplay")]
    pub data_type_display: Option<String>,

    /// Description of the column.
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Json schema only if the dataType is JSON else null.
    #[serde(rename = "jsonSchema")]
    pub json_schema: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    /// Ordinal position of the column.
    #[serde(rename = "ordinalPosition")]
    pub ordinal_position: Option<i64>,

    /// Tags associated with the column.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,
}

/// This captures information about how the table is modeled. Currently only DBT model is
/// supported.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataModel {
    /// Columns from the schema defined during modeling. In case of DBT, the metadata here comes
    /// from `schema.yaml`.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<TableColumn>>,

    /// Description of the Table from the model.
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "generatedAt")]
    pub generated_at: Option<String>,

    #[serde(rename = "modelType")]
    pub model_type: ModelType,

    /// Path to sql definition file.
    #[serde(rename = "path")]
    pub path: Option<String>,

    /// This corresponds to rws SQL from `<model_name>.sql` in DBT. This might be null when SQL
    /// query need not be compiled as done in DBT.
    #[serde(rename = "rawSql")]
    pub raw_sql: Option<String>,

    /// This corresponds to compile SQL from `<model_name>.sql` in DBT. In cases where
    /// compilation is not necessary, this corresponds to SQL that created the table.
    #[serde(rename = "sql")]
    pub sql: String,

    /// Fully qualified name of Models/tables used for in `sql` for creating this table.
    #[serde(rename = "upstream")]
    pub upstream: Option<Vec<String>>,
}

/// Details of other tables this table is frequently joined with.
///
/// This schema defines the type to capture information about how columns in this table are
/// joined with columns in the other tables.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableJoins {
    #[serde(rename = "columnJoins")]
    pub column_joins: Option<Vec<ColumnJoins>>,

    #[serde(rename = "dayCount")]
    pub day_count: Option<i64>,

    /// Date can be only from today going back to last 29 days.
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
}

/// This schema defines the type to capture how frequently a column is joined with columns in
/// the other tables.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColumnJoins {
    #[serde(rename = "columnName")]
    pub column_name: Option<String>,

    /// Fully qualified names of the columns that this column is joined with.
    #[serde(rename = "joinedWith")]
    pub joined_with: Option<Vec<JoinedWith>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JoinedWith {
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    #[serde(rename = "joinCount")]
    pub join_count: Option<i64>,
}

/// Sample data for a table.
///
/// This schema defines the type to capture rows of sample data for a table.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableData {
    /// List of local column names (not fully qualified column names) of the table.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<String>>,

    /// Data for multiple rows of the table.
    #[serde(rename = "rows")]
    pub rows: Option<Vec<Vec<Option<serde_json::Value>>>>,
}

/// This enum defines the type for table constraint.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableTableConstraint {
    /// List of column names corresponding to the constraint.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<String>>,

    #[serde(rename = "constraintType")]
    pub constraint_type: Option<ConstraintType>,
}

/// This schema defines the type to capture the table's data profile.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableProfile {
    /// No.of columns in the table.
    #[serde(rename = "columnCount")]
    pub column_count: Option<f64>,

    /// List of local column profiles of the table.
    #[serde(rename = "columnProfile")]
    pub column_profile: Option<Vec<ColumnProfile>>,

    /// Data one which profile is taken.
    #[serde(rename = "profileDate")]
    pub profile_date: Option<String>,

    /// No.of rows in the table.
    #[serde(rename = "rowCount")]
    pub row_count: Option<f64>,
}

/// This schema defines the type to capture the table's column profile.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColumnProfile {
    /// Maximum value in a column.
    #[serde(rename = "max")]
    pub max: Option<String>,

    /// Avg value in a column.
    #[serde(rename = "mean")]
    pub mean: Option<String>,

    /// Median value in a column.
    #[serde(rename = "median")]
    pub median: Option<String>,

    /// Minimum value in a column.
    #[serde(rename = "min")]
    pub min: Option<String>,

    /// Column Name.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// No.of null values in a column.
    #[serde(rename = "nullCount")]
    pub null_count: Option<f64>,

    /// No.of null value proportion in columns.
    #[serde(rename = "nullProportion")]
    pub null_proportion: Option<f64>,

    /// Standard deviation of a column.
    #[serde(rename = "stddev")]
    pub stddev: Option<f64>,

    /// No. of unique values in the column.
    #[serde(rename = "uniqueCount")]
    pub unique_count: Option<f64>,

    /// Proportion of number of unique values in a column.
    #[serde(rename = "uniqueProportion")]
    pub unique_proportion: Option<f64>,
}

/// This schema defines the type to capture the table's sql queries.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SqlQuery {
    /// Checksum to avoid registering duplicate queries.
    #[serde(rename = "checksum")]
    pub checksum: Option<String>,

    /// How long did the query took to run in seconds.
    #[serde(rename = "duration")]
    pub duration: Option<f64>,

    /// SQL Query text that matches the table name.
    #[serde(rename = "query")]
    pub query: Option<String>,

    /// Date on which the query ran.
    #[serde(rename = "queryDate")]
    pub query_date: Option<String>,

    /// User who ran this query.
    #[serde(rename = "user")]
    pub user: Option<ServiceElement>,

    /// Users can vote up to rank the popular queries.
    #[serde(rename = "vote")]
    pub vote: Option<f64>,
}

/// Latest usage information for this table.
///
/// This schema defines the type for usage details. Daily, weekly, and monthly aggregation of
/// usage is computed along with the percentile rank based on the usage for a given day.
///
/// Latest usage information for this ML Model.
///
/// Latest usage information for this database.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsageSummaryElement {
    /// Daily usage stats of a data asset on the start date.
    #[serde(rename = "dailyStats")]
    pub daily_stats: UsageStats,

    /// Date in UTC.
    #[serde(rename = "date")]
    pub date: String,

    /// Monthly (last 30 days) rolling usage stats of a data asset on the start date.
    #[serde(rename = "monthlyStats")]
    pub monthly_stats: Option<UsageStats>,

    /// Weekly (last 7 days) rolling usage stats of a data asset on the start date.
    #[serde(rename = "weeklyStats")]
    pub weekly_stats: Option<UsageStats>,
}

/// Daily usage stats of a data asset on the start date.
///
/// Type used to return usage statistics.
///
/// Monthly (last 30 days) rolling usage stats of a data asset on the start date.
///
/// Weekly (last 7 days) rolling usage stats of a data asset on the start date.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsageStats {
    /// Usage count of a data asset on the start date.
    #[serde(rename = "count")]
    pub count: i64,

    /// Optional daily percentile rank data asset use when relevant.
    #[serde(rename = "percentileRank")]
    pub percentile_rank: Option<f64>,
}

/// This schema defines the Model entity. Models are algorithms trained on data to find
/// patterns or make predictions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MlModel {
    /// Algorithm used to train the ML Model.
    #[serde(rename = "algorithm")]
    pub algorithm: String,

    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Performance Dashboard URL to track metric evolution.
    #[serde(rename = "dashboard")]
    pub dashboard: Option<ServiceElement>,

    /// Description of the ML Model, what it is, and how to use it.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this ML Model.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Followers of this ML Model.
    #[serde(rename = "followers")]
    pub followers: Option<Vec<ServiceElement>>,

    /// A unique name that identifies an ML Model.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier of an ML Model instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Features used to train the ML Model.
    #[serde(rename = "mlFeatures")]
    pub ml_features: Option<Vec<MlFeature>>,

    /// Hyper Parameters used to train the ML Model.
    #[serde(rename = "mlHyperParameters")]
    pub ml_hyper_parameters: Option<Vec<MlHyperParameter>>,

    /// Location containing the ML Model. It can be a storage layer and/or a container repository.
    #[serde(rename = "mlStore")]
    pub ml_store: Option<MlStore>,

    /// Name that identifies this ML Model.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this ML Model.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Endpoint that makes the ML Model available, e.g,. a REST API serving the data or
    /// computing predictions.
    #[serde(rename = "server")]
    pub server: Option<String>,

    /// Tags for this ML Model.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Latest usage information for this ML Model.
    #[serde(rename = "usageSummary")]
    pub usage_summary: Option<UsageSummaryElement>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines the type for an ML Feature used in an ML Model.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MlFeature {
    /// Data type of the column (numerical vs. categorical).
    #[serde(rename = "dataType")]
    pub data_type: Option<FeatureType>,

    /// Description of the ML Feature.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Description of the algorithm used to compute the feature, e.g., PCA, bucketing...
    #[serde(rename = "featureAlgorithm")]
    pub feature_algorithm: Option<String>,

    /// Columns used to create the ML Feature.
    #[serde(rename = "featureSources")]
    pub feature_sources: Option<Vec<FeatureSource>>,

    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Tags associated with the feature.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,
}

/// This schema defines the sources of a ML Feature.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeatureSource {
    /// Data type of the source (int, date etc.).
    #[serde(rename = "dataType")]
    pub data_type: Option<FeatureSourceDataType>,

    /// Description of the feature source.
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Tags associated with the feature source.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,
}

/// This schema defines the type for an ML HyperParameter used in an ML Model.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MlHyperParameter {
    /// Description of the Hyper Parameter.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Hyper parameter name.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Hyper parameter value.
    #[serde(rename = "value")]
    pub value: Option<String>,
}

/// Location containing the ML Model. It can be a storage layer and/or a container repository.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MlStore {
    /// Container Repository with the ML Model image.
    #[serde(rename = "imageRepository")]
    pub image_repository: Option<String>,

    /// Storage Layer containing the ML Model data.
    #[serde(rename = "storage")]
    pub storage: Option<String>,
}

/// This schema defines the Database entity. A database also referred to as Database Catalog
/// is a collection of tables.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Database {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Description of the database instance.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this database.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Name that uniquely identifies a database in the format 'ServiceName.DatabaseName'.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies this database instance.
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Reference to the Location that contains this database.
    #[serde(rename = "location")]
    pub location: Option<ServiceElement>,

    /// Name that identifies the database.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this database.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Link to the database cluster/service where this database is hosted in.
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Service type where this database is hosted in.
    #[serde(rename = "serviceType")]
    pub service_type: Option<DatabaseServiceType>,

    /// References to tables in the database.
    #[serde(rename = "tables")]
    pub tables: Option<Vec<ServiceElement>>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Latest usage information for this database.
    #[serde(rename = "usageSummary")]
    pub usage_summary: Option<UsageSummaryElement>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines the Dashboard entity. Dashboards are computed from data and visually
/// present data, metrics, and KPIs. They are updated in real-time and allow interactive data
/// exploration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dashboard {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// All the charts included in this Dashboard.
    #[serde(rename = "charts")]
    pub charts: Option<Vec<ServiceElement>>,

    /// Dashboard URL.
    #[serde(rename = "dashboardUrl")]
    pub dashboard_url: Option<String>,

    /// Description of the dashboard, what it is, and how to use it.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this Dashboard. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Followers of this dashboard.
    #[serde(rename = "followers")]
    pub followers: Option<Vec<ServiceElement>>,

    /// A unique name that identifies a dashboard in the format 'ServiceName.DashboardName'.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies a dashboard instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Name that identifies this dashboard.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this dashboard.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Link to service where this dashboard is hosted in.
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Service type where this dashboard is hosted in.
    #[serde(rename = "serviceType")]
    pub service_type: Option<DashboardServiceType>,

    /// Tags for this dashboard.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Latest usage information for this database.
    #[serde(rename = "usageSummary")]
    pub usage_summary: Option<UsageSummaryElement>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines the Chart entity. Charts are built using tables or sql queries by
/// analyzing the data. Charts can be part of Dashboard.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Chart {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    #[serde(rename = "chartType")]
    pub chart_type: Option<ChartType>,

    /// Chart URL, pointing to its own Service URL.
    #[serde(rename = "chartUrl")]
    pub chart_url: Option<String>,

    /// Description of the dashboard, what it is, and how to use it.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this Chart. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Followers of this chart.
    #[serde(rename = "followers")]
    pub followers: Option<Vec<ServiceElement>>,

    /// A unique name that identifies a dashboard in the format 'ServiceName.ChartName'.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies a chart instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Name that identifies this Chart.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this dashboard.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Link to service where this dashboard is hosted in.
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Service type where this chart is hosted in.
    #[serde(rename = "serviceType")]
    pub service_type: Option<DashboardServiceType>,

    /// Link to table used in this chart.
    #[serde(rename = "tables")]
    pub tables: Option<Vec<ServiceElement>>,

    /// Tags for this chart.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Latest usage information for this database.
    #[serde(rename = "usageSummary")]
    pub usage_summary: Option<UsageSummaryElement>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines the Metrics entity. Metrics are measurements computed from data such
/// as `Monthly Active Users`. Some of the metrics that measures used to determine
/// performance against an objective are called KPIs or Key Performance Indicators, such as
/// `User Retention`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metrics {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Description of metrics instance, what it is, and how to use it.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this metric.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// A unique name that identifies a metric in the format 'ServiceName.MetricName'.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies this metrics instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Name that identifies this metrics instance uniquely.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this metrics.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Link to service where this metrics is hosted in.
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Tags for this chart.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Latest usage information for this database.
    #[serde(rename = "usageSummary")]
    pub usage_summary: Option<UsageSummaryElement>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines the Topic entity. A topic is a feed into which message are published
/// to by publishers and read from by consumers in a messaging service.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Topic {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Topic clean up policies. For Kafka - `cleanup.policy` configuration.
    #[serde(rename = "cleanupPolicies")]
    pub cleanup_policies: Option<Vec<CleanupPolicy>>,

    /// Description of the topic instance.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this topic. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Followers of this table.
    #[serde(rename = "followers")]
    pub followers: Option<Vec<ServiceElement>>,

    /// Name that uniquely identifies a topic in the format 'messagingServiceName.topicName'.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies this topic instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Maximum message size in bytes. For Kafka - `max.message.bytes` configuration.
    #[serde(rename = "maximumMessageSize")]
    pub maximum_message_size: Option<i64>,

    /// Minimum number replicas in sync to control durability. For Kafka - `min.insync.replicas`
    /// configuration.
    #[serde(rename = "minimumInSyncReplicas")]
    pub minimum_in_sync_replicas: Option<i64>,

    /// Name that identifies the topic.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this topic.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Number of partitions into which the topic is divided.
    #[serde(rename = "partitions")]
    pub partitions: i64,

    /// Replication Factor in integer (more than 1).
    #[serde(rename = "replicationFactor")]
    pub replication_factor: Option<i64>,

    /// Maximum size of a partition in bytes before old data is discarded. For Kafka -
    /// `retention.bytes` configuration.
    #[serde(rename = "retentionSize")]
    pub retention_size: Option<f64>,

    /// Retention time in milliseconds. For Kafka - `retention.ms` configuration.
    #[serde(rename = "retentionTime")]
    pub retention_time: Option<f64>,

    /// Schema used for message serialization. Optional as some topics may not have associated
    /// schemas.
    #[serde(rename = "schemaText")]
    pub schema_text: Option<String>,

    /// Schema used for message serialization.
    #[serde(rename = "schemaType")]
    pub schema_type: Option<SchemaType>,

    /// Link to the messaging cluster/service where this topic is hosted in.
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Service type where this topic is hosted in.
    #[serde(rename = "serviceType")]
    pub service_type: Option<MessagingServiceType>,

    /// Tags for this table.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines the Report entity. Reports are static information computed from data
/// periodically that includes data in text, table, and visual form.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Description of this report instance.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this report. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// A unique name that identifies a report in the format 'ServiceName.ReportName'.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to the resource corresponding to this report.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies this report.
    #[serde(rename = "id")]
    pub id: String,

    /// Name that identifies this report instance uniquely.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this pipeline.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Link to service where this report is hosted in.
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Latest usage information for this database.
    #[serde(rename = "usageSummary")]
    pub usage_summary: Option<UsageSummaryElement>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines the Messaging Service entity, such as Kafka and Pulsar.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessagingService {
    /// Multiple bootstrap addresses for Kafka. Single proxy address for Pulsar.
    #[serde(rename = "brokers")]
    pub brokers: Vec<String>,

    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Description of a messaging service instance.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this messaging service. It could be title or label from the
    /// source services.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Link to the resource corresponding to this messaging service.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier of this messaging service instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Schedule for running metadata ingestion jobs.
    #[serde(rename = "ingestionSchedule")]
    pub ingestion_schedule: Option<Schedule>,

    /// Name that identifies this messaging service.
    #[serde(rename = "name")]
    pub name: String,

    /// Schema registry URL.
    #[serde(rename = "schemaRegistry")]
    pub schema_registry: Option<String>,

    /// Type of messaging service such as Kafka or Pulsar...
    #[serde(rename = "serviceType")]
    pub service_type: MessagingServiceType,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// Schedule for running metadata ingestion jobs.
///
/// This schema defines the type used for the schedule. The schedule has a start time and
/// repeat frequency.
///
/// Schedule for running metadata ingestion jobs
///
/// Schedule for running pipeline ingestion jobs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngestionScheduleClass {
    /// Repeat frequency in ISO 8601 duration format. Example - 'P23DT23H'.
    #[serde(rename = "repeatFrequency")]
    pub repeat_frequency: Option<String>,

    /// Start date and time of the schedule.
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
}

/// This schema defines the Database Service entity, such as MySQL, BigQuery, Redshift,
/// Postgres, or Snowflake. Alternative terms such as Database Cluster, Database Server
/// instance are also used for database service.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseService {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Description of a database service instance.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this database service.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Link to the resource corresponding to this database service.
    #[serde(rename = "href")]
    pub href: String,

    /// Unique identifier of this database service instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Schedule for running metadata ingestion jobs.
    #[serde(rename = "ingestionSchedule")]
    pub ingestion_schedule: Option<Schedule>,

    /// JDBC connection information.
    #[serde(rename = "jdbc")]
    pub jdbc: JdbcInfo,

    /// Name that identifies this database service.
    #[serde(rename = "name")]
    pub name: String,

    /// Type of database service such as MySQL, BigQuery, Snowflake, Redshift, Postgres...
    #[serde(rename = "serviceType")]
    pub service_type: DatabaseServiceType,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// JDBC connection information.
///
/// Type for capturing JDBC connector information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JdbcInfo {
    #[serde(rename = "connectionUrl")]
    pub connection_url: String,

    #[serde(rename = "driverClass")]
    pub driver_class: String,
}

/// This schema defines the Dashboard Service entity, such as Looker and Superset.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardService {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Dashboard Service URL. This will be used to make REST API calls to Dashboard Service.
    #[serde(rename = "dashboardUrl")]
    pub dashboard_url: String,

    /// Description of a dashboard service instance.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this dashboard service.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Link to the resource corresponding to this dashboard service.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier of this dashboard service instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Schedule for running metadata ingestion jobs.
    #[serde(rename = "ingestionSchedule")]
    pub ingestion_schedule: Option<Schedule>,

    /// Name that identifies this dashboard service.
    #[serde(rename = "name")]
    pub name: String,

    /// Password to log-into Dashboard Service.
    #[serde(rename = "password")]
    pub password: Option<String>,

    /// Type of dashboard service such as Looker or Superset...
    #[serde(rename = "serviceType")]
    pub service_type: DashboardServiceType,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Username to log-into Dashboard Service.
    #[serde(rename = "username")]
    pub username: Option<String>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines the Pipeline Service entity, such as Airflow and Prefect.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PipelineService {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Description of a pipeline service instance.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this pipeline service. It could be title or label from the
    /// source services.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Link to the resource corresponding to this pipeline service.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier of this pipeline service instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Schedule for running metadata ingestion jobs.
    #[serde(rename = "ingestionSchedule")]
    pub ingestion_schedule: Option<Schedule>,

    /// Name that identifies this pipeline service.
    #[serde(rename = "name")]
    pub name: String,

    /// Pipeline Service Management/UI URL.
    #[serde(rename = "pipelineUrl")]
    pub pipeline_url: String,

    /// Type of pipeline service such as Airflow or Prefect...
    #[serde(rename = "serviceType")]
    pub service_type: Option<PipelineServiceType>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines the User entity. A user can be part of 0 or more teams. A special
/// type of user called Bot is used for automation. A user can be an owner of zero or more
/// data assets. A user can also follow zero or more data assets.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// When true indicates the user has been deactivated. Users are deactivated instead of
    /// deleted.
    #[serde(rename = "deactivated")]
    pub deactivated: Option<bool>,

    /// Used for user biography.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Name used for display purposes. Example 'FirstName LastName'.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Email address of the user.
    #[serde(rename = "email")]
    pub email: String,

    /// List of entities followed by the user.
    #[serde(rename = "follows")]
    pub follows: Option<Vec<ServiceElement>>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: String,

    /// Unique identifier that identifies a user entity instance.
    #[serde(rename = "id")]
    pub id: String,

    /// When true indicates user is an administrator for the system with superuser privileges.
    #[serde(rename = "isAdmin")]
    pub is_admin: Option<bool>,

    /// When true indicates a special type of user called Bot.
    #[serde(rename = "isBot")]
    pub is_bot: Option<bool>,

    #[serde(rename = "name")]
    pub name: String,

    /// List of entities owned by the user.
    #[serde(rename = "owns")]
    pub owns: Option<Vec<ServiceElement>>,

    /// Profile of the user.
    #[serde(rename = "profile")]
    pub profile: Option<ProfileClass>,

    /// Teams that the user belongs to.
    #[serde(rename = "teams")]
    pub teams: Option<Vec<ServiceElement>>,

    /// Timezone of the user.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// Profile of the user.
///
/// This schema defines the type for a profile of a user, team, or organization.
///
/// Team profile information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfileClass {
    #[serde(rename = "images")]
    pub images: Option<ImageList>,
}

/// Links to a list of images of varying resolutions/sizes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageList {
    #[serde(rename = "image")]
    pub image: Option<String>,

    #[serde(rename = "image192")]
    pub image192: Option<String>,

    #[serde(rename = "image24")]
    pub image24: Option<String>,

    #[serde(rename = "image32")]
    pub image32: Option<String>,

    #[serde(rename = "image48")]
    pub image48: Option<String>,

    #[serde(rename = "image512")]
    pub image512: Option<String>,

    #[serde(rename = "image72")]
    pub image72: Option<String>,
}

/// This schema defines the Team entity. A Team is a group of zero or more users. Teams can
/// own zero or more data assets.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Team {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// When true the team has been deleted.
    #[serde(rename = "deleted")]
    pub deleted: Option<bool>,

    /// Description of the team.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Name used for display purposes. Example 'Data Science team'.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    /// List of entities owned by the team.
    #[serde(rename = "owns")]
    pub owns: Option<Vec<ServiceElement>>,

    /// Team profile information.
    #[serde(rename = "profile")]
    pub profile: Option<ProfileClass>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Users that are part of the team.
    #[serde(rename = "users")]
    pub users: Option<Vec<ServiceElement>>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines Bot entity. A bot automates tasks, such as adding description,
/// identifying the importance of data. It runs as a special user in the system.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bot {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Description of the bot.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Name used for display purposes. Example 'FirstName LastName'.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Link to the resource corresponding to this bot.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier of a bot instance.
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Name of the bot.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines the Tag Category entity. A Tag Category contains tags called Primary
/// Tags. Primary Tags can further have children Tags called Secondary Tags. Only two levels
/// of tags are supported currently.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagCategory {
    #[serde(rename = "categoryType")]
    pub category_type: TagCategoryType,

    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Tags under this category.
    #[serde(rename = "children")]
    pub children: Option<Vec<Option<Tag>>>,

    /// Description of the tag category.
    #[serde(rename = "description")]
    pub description: String,

    /// Display Name that identifies this tag category.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Link to the resource corresponding to the tag category.
    #[serde(rename = "href")]
    pub href: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Count of how many times the tags from this tag category are used.
    #[serde(rename = "usageCount")]
    pub usage_count: Option<i64>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagClass {
    /// Fully qualified names of tags associated with this tag. Associated tags captures
    /// relationship of one tag to another automatically. As an example a tag 'User.PhoneNumber'
    /// might have an associated tag 'PII.Sensitive'. When 'User.Address' is used to label a
    /// column in a table, 'PII.Sensitive' label is also applied automatically due to Associated
    /// tag relationship.
    #[serde(rename = "associatedTags")]
    pub associated_tags: Option<Vec<String>>,

    /// Tags under this tag group or empty for tags at the leaf level.
    #[serde(rename = "children")]
    pub children: Option<Vec<Option<Tag>>>,

    /// If the tag is deprecated.
    #[serde(rename = "deprecated")]
    pub deprecated: Option<bool>,

    /// Unique name of the tag category.
    #[serde(rename = "description")]
    pub description: String,

    /// Unique name of the tag of format Category.PrimaryTag.SecondaryTag.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to the resource corresponding to the tag.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Name of the tag.
    #[serde(rename = "name")]
    pub name: String,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Count of how many times this tag and children tags are used.
    #[serde(rename = "usageCount")]
    pub usage_count: Option<i64>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// A Thread is a collection of posts made by the users. The first post that starts a thread
/// is **about** a data asset **from** a user. Other users can respond to this post by
/// creating new posts in the thread.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Thread {
    /// Data asset about which this thread is created for with format
    /// <#E/{entities}/{entityName}/{field}/{fieldValue}.
    #[serde(rename = "about")]
    pub about: String,

    /// User or team this thread is addressed to in format
    /// <#E/{entities}/{entityName}/{field}/{fieldValue}.
    #[serde(rename = "addressedTo")]
    pub addressed_to: Option<String>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies an entity instance.
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "posts")]
    pub posts: Vec<Post>,

    /// Timestamp of the when the first post created the thread.
    #[serde(rename = "threadTs")]
    pub thread_ts: Option<String>,
}

/// Post within a feed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    /// ID of User (regular user or a bot) posting the message.
    #[serde(rename = "from")]
    pub from: String,

    /// Message in markdown format. See markdown support for more details.
    #[serde(rename = "message")]
    pub message: String,

    /// Timestamp of the post.
    #[serde(rename = "postTs")]
    pub post_ts: Option<String>,
}

/// Describes an entity Lifecycle Rule used within a Policy.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleRule {
    /// A set of actions to take on the entities.
    #[serde(rename = "actions")]
    pub actions: Vec<ActionElement>,

    /// Is the rule enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,

    #[serde(rename = "filters")]
    pub filters: Vec<Filter>,

    /// Name that identifies this Rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

/// An action to delete or expire the entity.
///
/// An action to move the entity to a different location. For eg: Move from Standard storage
/// tier to Archive storage tier.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionElement {
    /// Number of days after creation of the entity that the deletion should be triggered.
    ///
    /// Number of days after creation of the entity that the move should be triggered.
    #[serde(rename = "daysAfterCreation")]
    pub days_after_creation: Option<i64>,

    /// Number of days after last modification of the entity that the deletion should be
    /// triggered.
    ///
    /// Number of days after last modification of the entity that the move should be triggered.
    #[serde(rename = "daysAfterModification")]
    pub days_after_modification: Option<i64>,

    /// Location where this entity needs to be moved to.
    #[serde(rename = "destination")]
    pub destination: Option<ActionDestination>,
}

/// Location where this entity needs to be moved to.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionDestination {
    /// The location where to move this entity to.
    #[serde(rename = "location")]
    pub location: Option<Location>,

    /// The storage class to move this entity to.
    #[serde(rename = "storageClassType")]
    pub storage_class_type: Option<StorageClassType>,

    /// The storage service to move this entity to.
    #[serde(rename = "storageServiceType")]
    pub storage_service_type: Option<StorageService>,
}

/// This schema defines the Location entity. A Location can contain the data of a table or
/// group other sublocation together.
///
/// The location where to move this entity to.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Description of a location.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this table. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Followers of this location.
    #[serde(rename = "followers")]
    pub followers: Option<Vec<ServiceElement>>,

    /// Fully qualified name of a location in the form `serviceName.locationName`.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to this location resource.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier of this location instance.
    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "locationType")]
    pub location_type: Option<LocationType>,

    /// Name of a location without the service. For example s3://bucket/path1/path2.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this location.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Link to the database cluster/service where this database is hosted in.
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Service type where this storage location is hosted in.
    #[serde(rename = "serviceType")]
    pub service_type: Option<StorageServiceType>,

    /// Tags for this location.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// This schema defines the Storage Service entity, such as S3, GCS, HDFS.
///
/// The storage service to move this entity to.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageService {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Description of a storage service instance.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this storage service.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Link to the resource corresponding to this storage service.
    #[serde(rename = "href")]
    pub href: String,

    /// Unique identifier of this storage service instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Name that identifies this storage service.
    #[serde(rename = "name")]
    pub name: String,

    /// Type of storage service such as S3, GCS, HDFS...
    #[serde(rename = "serviceType")]
    pub service_type: StorageServiceType,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// Entity tags to match on.
///
/// This schema defines the type for labeling an entity with a Tag.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagLabel {
    /// Unique name of the tag category.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Link to the tag resource.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Label type describes how a tag label was applied. 'Manual' indicates the tag label was
    /// applied by a person. 'Derived' indicates a tag label was derived using the associated tag
    /// relationship (see TagCategory.json for more details). 'Propagated` indicates a tag label
    /// was propagated from upstream based on lineage. 'Automated' is used when a tool was used
    /// to determine the tag label.
    #[serde(rename = "labelType")]
    pub label_type: LabelType,

    /// 'Suggested' state is used when a tag label is suggested by users or tools. Owner of the
    /// entity must confirm the suggested labels before it is marked as 'Confirmed'.
    #[serde(rename = "state")]
    pub state: State,

    #[serde(rename = "tagFQN")]
    pub tag_fqn: String,
}

/// An action to move the entity to a different location. For eg: Move from Standard storage
/// tier to Archive storage tier.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleMoveAction {
    /// Number of days after creation of the entity that the move should be triggered.
    #[serde(rename = "daysAfterCreation")]
    pub days_after_creation: Option<i64>,

    /// Number of days after last modification of the entity that the move should be triggered.
    #[serde(rename = "daysAfterModification")]
    pub days_after_modification: Option<i64>,

    /// Location where this entity needs to be moved to.
    #[serde(rename = "destination")]
    pub destination: Option<LifecycleMoveActionDestination>,
}

/// Location where this entity needs to be moved to.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleMoveActionDestination {
    /// The location where to move this entity to.
    #[serde(rename = "location")]
    pub location: Option<Location>,

    /// The storage class to move this entity to.
    #[serde(rename = "storageClassType")]
    pub storage_class_type: Option<StorageClassType>,

    /// The storage service to move this entity to.
    #[serde(rename = "storageServiceType")]
    pub storage_service_type: Option<StorageService>,
}

/// An action to delete or expire the entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifecycleDeleteAction {
    /// Number of days after creation of the entity that the deletion should be triggered.
    #[serde(rename = "daysAfterCreation")]
    pub days_after_creation: Option<i64>,

    /// Number of days after last modification of the entity that the deletion should be
    /// triggered.
    #[serde(rename = "daysAfterModification")]
    pub days_after_modification: Option<i64>,
}

/// Describes an entity Access Control Rule used within a Policy.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccessControlRule {
    /// A set of access control enforcements to take on the entities.
    #[serde(rename = "actions")]
    pub actions: Vec<AccessControlRuleAction>,

    /// Is the rule enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,

    #[serde(rename = "filters")]
    pub filters: Vec<Filter>,

    /// Name that identifies this Rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

/// Describes an Access Control Rule to selectively grant access to Teams/Users to tagged
/// entities.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccessControlRuleAction {
    /// Teams and Users who are able to access the tagged entities.
    #[serde(rename = "allow")]
    pub allow: Vec<AllowElement>,

    /// Tags that are associated with the entities.
    #[serde(rename = "tags")]
    pub tags: Vec<TagLabel>,
}

/// This schema defines the Team entity. A Team is a group of zero or more users. Teams can
/// own zero or more data assets.
///
/// This schema defines the User entity. A user can be part of 0 or more teams. A special
/// type of user called Bot is used for automation. A user can be an owner of zero or more
/// data assets. A user can also follow zero or more data assets.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllowElement {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// When true the team has been deleted.
    #[serde(rename = "deleted")]
    pub deleted: Option<bool>,

    /// Description of the team.
    ///
    /// Used for user biography.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Name used for display purposes. Example 'Data Science team'.
    ///
    /// Name used for display purposes. Example 'FirstName LastName'.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: String,

    /// Unique identifier that identifies a user entity instance.
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    /// List of entities owned by the team.
    ///
    /// List of entities owned by the user.
    #[serde(rename = "owns")]
    pub owns: Option<Vec<ServiceElement>>,

    /// Team profile information.
    ///
    /// Profile of the user.
    #[serde(rename = "profile")]
    pub profile: Option<ProfileClass>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Users that are part of the team.
    #[serde(rename = "users")]
    pub users: Option<Vec<ServiceElement>>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,

    /// When true indicates the user has been deactivated. Users are deactivated instead of
    /// deleted.
    #[serde(rename = "deactivated")]
    pub deactivated: Option<bool>,

    /// Email address of the user.
    #[serde(rename = "email")]
    pub email: Option<String>,

    /// List of entities followed by the user.
    #[serde(rename = "follows")]
    pub follows: Option<Vec<ServiceElement>>,

    /// When true indicates user is an administrator for the system with superuser privileges.
    #[serde(rename = "isAdmin")]
    pub is_admin: Option<bool>,

    /// When true indicates a special type of user called Bot.
    #[serde(rename = "isBot")]
    pub is_bot: Option<bool>,

    /// Teams that the user belongs to.
    #[serde(rename = "teams")]
    pub teams: Option<Vec<ServiceElement>>,

    /// Timezone of the user.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}

/// Describes an Access Control Rule to selectively grant access to Teams/Users to tagged
/// entities.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagBased {
    /// Teams and Users who are able to access the tagged entities.
    #[serde(rename = "allow")]
    pub allow: Vec<AllowElement>,

    /// Tags that are associated with the entities.
    #[serde(rename = "tags")]
    pub tags: Vec<TagLabel>,
}

/// This schema defines the Policy entity. A Policy defines lifecycle or access control that
/// needs to be applied across different Data Entities.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    /// Change that led to this version of the Policy.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PolicyChangeDescription>,

    /// A short description of the Policy, comprehensible to regular users.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Title for this Policy.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Is the policy enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,

    /// Name that uniquely identifies a Policy.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to the resource corresponding to this entity.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies this Policy.
    #[serde(rename = "id")]
    pub id: String,

    /// Name that identifies this Policy.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this Policy.
    #[serde(rename = "owner")]
    pub owner: Option<EntityReference>,

    #[serde(rename = "policyType")]
    pub policy_type: PolicyType,

    /// Link to a well documented definition of this Policy.
    #[serde(rename = "policyUrl")]
    pub policy_url: Option<String>,

    #[serde(rename = "rules")]
    pub rules: Option<Vec<PolicyRule>>,

    /// Last update time corresponding to the new version of the Policy.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Metadata version of the Policy.
    #[serde(rename = "version")]
    pub version: Option<f64>,
}

/// Change that led to this version of the Policy.
///
/// Description of the change.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyChangeDescription {
    /// Names of fields added during the version changes.
    #[serde(rename = "fieldsAdded")]
    pub fields_added: Option<Vec<FluffyFieldChange>>,

    /// Fields deleted during the version changes with old value before deleted.
    #[serde(rename = "fieldsDeleted")]
    pub fields_deleted: Option<Vec<FluffyFieldChange>>,

    /// Fields modified during the version changes with old and new values.
    #[serde(rename = "fieldsUpdated")]
    pub fields_updated: Option<Vec<FluffyFieldChange>>,

    /// When a change did not result in change, this could be same as the current version.
    #[serde(rename = "previousVersion")]
    pub previous_version: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FluffyFieldChange {
    /// Name of the entity field that changed.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// New value of the field. Note that this is a JSON string and use the corresponding field
    /// type to deserialize it.
    #[serde(rename = "newValue")]
    pub new_value: Option<serde_json::Value>,

    /// Previous value of the field. Note that this is a JSON string and use the corresponding
    /// field type to deserialize it.
    #[serde(rename = "oldValue")]
    pub old_value: Option<serde_json::Value>,
}

/// Owner of this Policy.
///
/// This schema defines the EntityReference type used for referencing an entity.
/// EntityReference is used for capturing relationships from one entity to another. For
/// example, a table has an attribute called database of type EntityReference that captures
/// the relationship of a table `belongs to a` database.
///
/// Owner of this database
///
/// Link to the database service where this database is hosted in
///
/// Owner of this entity
///
/// Owner of this topic
///
/// Link to the messaging service where this topic is hosted in
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityReference {
    /// Optional description of entity.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this entity.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Link to the entity resource.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies an entity instance.
    #[serde(rename = "id")]
    pub id: String,

    /// Name of the entity instance. For entities such as tables, databases where the name is not
    /// unique, fullyQualifiedName is returned in this field.
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Entity type/class name - Examples: `database`, `table`, `metrics`, `databaseService`,
    /// `dashboardService`...
    #[serde(rename = "type")]
    pub entity_reference_type: String,
}

/// A set of rules associated with the Policy.
///
/// Describes an entity Access Control Rule used within a Policy.
///
/// Describes an entity Lifecycle Rule used within a Policy.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PolicyRule {
    /// A set of access control enforcements to take on the entities.
    ///
    /// A set of actions to take on the entities.
    #[serde(rename = "actions")]
    pub actions: Vec<RuleAction>,

    /// Is the rule enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,

    #[serde(rename = "filters")]
    pub filters: Vec<Filter>,

    /// Name that identifies this Rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

/// Describes an Access Control Rule to selectively grant access to Teams/Users to tagged
/// entities.
///
/// An action to delete or expire the entity.
///
/// An action to move the entity to a different location. For eg: Move from Standard storage
/// tier to Archive storage tier.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuleAction {
    /// Teams and Users who are able to access the tagged entities.
    #[serde(rename = "allow")]
    pub allow: Option<Vec<AllowElement>>,

    /// Tags that are associated with the entities.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagLabel>>,

    /// Number of days after creation of the entity that the deletion should be triggered.
    ///
    /// Number of days after creation of the entity that the move should be triggered.
    #[serde(rename = "daysAfterCreation")]
    pub days_after_creation: Option<i64>,

    /// Number of days after last modification of the entity that the deletion should be
    /// triggered.
    ///
    /// Number of days after last modification of the entity that the move should be triggered.
    #[serde(rename = "daysAfterModification")]
    pub days_after_modification: Option<i64>,

    /// Location where this entity needs to be moved to.
    #[serde(rename = "destination")]
    pub destination: Option<ActionDestination>,
}

/// This schema defines the type used for lineage of an entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityLineage {
    #[serde(rename = "downstreamEdges")]
    pub downstream_edges: Option<Vec<Edge>>,

    /// Primary entity for which this lineage graph is created.
    #[serde(rename = "entity")]
    pub entity: ServiceElement,

    #[serde(rename = "nodes")]
    pub nodes: Option<Vec<ServiceElement>>,

    #[serde(rename = "upstreamEdges")]
    pub upstream_edges: Option<Vec<Edge>>,
}

/// Edge in the lineage graph from one entity to another by entity IDs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// From entity that is upstream of lineage edge.
    #[serde(rename = "fromEntity")]
    pub from_entity: Option<String>,

    /// To entity that is downstream of lineage edge.
    #[serde(rename = "toEntity")]
    pub to_entity: Option<String>,
}

/// This schema defines the type for reporting the daily count of some measurement. For
/// example, you might use this schema for the number of times a table is queried each day.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DailyCountOfSomeMeasurement {
    /// Daily count of a measurement on the given date.
    #[serde(rename = "count")]
    pub count: i64,

    #[serde(rename = "date")]
    pub date: String,
}

/// This schema defines the type used for capturing usage details of an entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsageDetailsOfAnEntity {
    /// Entity for which usage is returned.
    #[serde(rename = "entity")]
    pub entity: ServiceElement,

    /// List usage details per day.
    #[serde(rename = "usage")]
    pub usage: Vec<UsageSummaryElement>,
}

/// Type used for capturing the details of a collection.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchemaForCollectionDescriptor {
    #[serde(rename = "collection")]
    pub collection: Option<CollectionInfo>,
}

/// Collection Info.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CollectionInfo {
    /// Description of collection.
    #[serde(rename = "documentation")]
    pub documentation: Option<String>,

    /// URL of the API endpoint where given collections are available.
    #[serde(rename = "href")]
    pub href: Option<String>,

    #[serde(rename = "images")]
    pub images: Option<ImageList>,

    /// Unique name that identifies a collection.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

/// This schema defines the type for usage details. Daily, weekly, and monthly aggregation of
/// usage is computed along with the percentile rank based on the usage for a given day.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypeUsedToReturnUsageDetailsOfAnEntity {
    /// Daily usage stats of a data asset on the start date.
    #[serde(rename = "dailyStats")]
    pub daily_stats: UsageStats,

    /// Date in UTC.
    #[serde(rename = "date")]
    pub date: String,

    /// Monthly (last 30 days) rolling usage stats of a data asset on the start date.
    #[serde(rename = "monthlyStats")]
    pub monthly_stats: Option<UsageStats>,

    /// Weekly (last 7 days) rolling usage stats of a data asset on the start date.
    #[serde(rename = "weeklyStats")]
    pub weekly_stats: Option<UsageStats>,
}

/// This schema defines the type used for JDBC connection information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JdbcConnection {
    /// JDBC connection URL.
    #[serde(rename = "connectionUrl")]
    pub connection_url: String,

    /// JDBC driver class.
    #[serde(rename = "driverClass")]
    pub driver_class: String,

    /// Login password.
    #[serde(rename = "password")]
    pub password: String,

    /// Login user name.
    #[serde(rename = "userName")]
    pub user_name: String,
}

/// This schema defines the type used for capturing version of history of entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityVersionHistory {
    /// Entity type, such as `database`, `table`, `dashboard`, for which this version history is
    /// produced.
    #[serde(rename = "entityType")]
    pub entity_type: String,

    #[serde(rename = "versions")]
    pub versions: Vec<Option<serde_json::Value>>,
}

/// This schema defines the change event type to capture the changes to entities. Entities
/// change due to user activity, such as updating description of a dataset, changing
/// ownership, or adding new tags. Entity also changes due to activities at the metadata
/// sources, such as a new dataset was created, a datasets was deleted, or schema of a
/// dataset is modified. When state of entity changes, an event is produced. These events can
/// be used to build apps and bots that respond to the change from activities.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeEvent {
    /// For `eventType` `entityUpdated` this field captures details about what fields were
    /// added/updated/deleted. For `eventType` `entityCreated` or `entityDeleted` this field is
    /// null.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Current version of the entity after this change. Note that not all changes result in
    /// entity version change. When entity version is not changed, `previousVersion` is same as
    /// `currentVersion`.
    #[serde(rename = "currentVersion")]
    pub current_version: Option<f64>,

    /// Date and time when the change was made.
    #[serde(rename = "dateTime")]
    pub date_time: String,

    /// For `eventType` `entityCreated`, this field captures JSON coded string of the entity
    /// using the schema corresponding to `entityType`.
    #[serde(rename = "entity")]
    pub entity: Option<serde_json::Value>,

    /// Identifier of entity that was modified by the operation.
    #[serde(rename = "entityId")]
    pub entity_id: String,

    /// Entity type that changed. Use the schema of this entity to process the entity attribute.
    #[serde(rename = "entityType")]
    pub entity_type: String,

    #[serde(rename = "eventType")]
    pub event_type: EventType,

    /// Version of the entity before this change. Note that not all changes result in entity
    /// version change. When entity version is not changed, `previousVersion` is same as
    /// `currentVersion`.
    #[serde(rename = "previousVersion")]
    pub previous_version: Option<f64>,

    /// Name of the user whose activity resulted in the change.
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
}

/// Type used for cursor based pagination information in GET list responses.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Paging {
    /// After cursor used for getting the next page (see API pagination for details).
    #[serde(rename = "after")]
    pub after: Option<String>,

    /// Before cursor used for getting the previous page (see API pagination for details).
    #[serde(rename = "before")]
    pub before: Option<String>,

    /// Total number of entries available to page through.
    #[serde(rename = "total")]
    pub total: i64,
}

/// This schema defines the Audit Log type to capture the audit trail of POST, PUT, and PATCH
/// API operations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditLog {
    /// Date when the API call is made.
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>,

    /// Identifier of entity that was modified by the operation.
    #[serde(rename = "entityId")]
    pub entity_id: String,

    /// Type of Entity that is modified by the operation.
    #[serde(rename = "entityType")]
    pub entity_type: String,

    /// HTTP Method used in a call.
    #[serde(rename = "method")]
    pub method: Method,

    /// Requested API Path.
    #[serde(rename = "path")]
    pub path: String,

    /// HTTP response code for the api requested.
    #[serde(rename = "responseCode")]
    pub response_code: i64,

    /// Name of the user who made the API request.
    #[serde(rename = "userName")]
    pub user_name: String,
}

/// This schema defines the type used for the schedule. The schedule has a start time and
/// repeat frequency.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    /// Repeat frequency in ISO 8601 duration format. Example - 'P23DT23H'.
    #[serde(rename = "repeatFrequency")]
    pub repeat_frequency: Option<String>,

    /// Start date and time of the schedule.
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
}

/// Ingestion Config is used to setup a Airflow Ingestion pipeline.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ingestion {
    /// Change that led to this version of the entity.
    #[serde(rename = "changeDescription")]
    pub change_description: Option<PipelineChangeDescription>,

    /// Concurrency of the Pipeline.
    #[serde(rename = "concurrency")]
    pub concurrency: Option<i64>,

    #[serde(rename = "connectorConfig")]
    pub connector_config: ConnectorConfig,

    /// Description of the workflow.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this Ingestion.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// End Date of the workflow.
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,

    /// Deploy the workflow by overwriting existing workflow with the same name.
    #[serde(rename = "forceDeploy")]
    pub force_deploy: Option<bool>,

    /// Name that uniquely identifies a Ingestion.
    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Link to this ingestion resource.
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// Unique identifier that identifies this Ingestion.
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// List of executions and status for the Ingestion Pipeline.
    #[serde(rename = "ingestionStatuses")]
    pub ingestion_statuses: Option<Vec<IngestionStatus>>,

    #[serde(rename = "ingestionType")]
    pub ingestion_type: Option<IngestionType>,

    /// Name that identifies this ingestion instance uniquely.
    #[serde(rename = "name")]
    pub name: String,

    /// Next execution date from the underlying workflow platform once the ingestion scheduled.
    #[serde(rename = "nextExecutionDate")]
    pub next_execution_date: Option<String>,

    /// Owner of this Ingestion.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// pause the workflow from running once the deploy is finished successfully.
    #[serde(rename = "pauseWorkflow")]
    pub pause_workflow: Option<bool>,

    /// Retry workflow in case of failure.
    #[serde(rename = "retries")]
    pub retries: Option<i64>,

    /// Delay between retries in seconds.
    #[serde(rename = "retryDelay")]
    pub retry_delay: Option<i64>,

    /// Scheduler Interval for the Workflow in cron format.
    #[serde(rename = "scheduleInterval")]
    pub schedule_interval: Option<String>,

    /// Link to the database service where this database is hosted in.
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Start date of the workflow.
    #[serde(rename = "startDate")]
    pub start_date: String,

    /// Tags associated with the Ingestion.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,

    /// User who made the update.
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<String>,

    /// Metadata version of the entity.
    #[serde(rename = "version")]
    pub version: Option<f64>,

    /// Run past executions if the start date is in the past.
    #[serde(rename = "workflowCatchup")]
    pub workflow_catchup: Option<bool>,

    /// Timeout for the workflow in seconds.
    #[serde(rename = "workflowTimeout")]
    pub workflow_timeout: Option<i64>,

    /// Timezone in which workflow going to be scheduled.
    #[serde(rename = "workflowTimezone")]
    pub workflow_timezone: Option<String>,
}

/// This defines the configuration for connector.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectorConfig {
    /// Database of the data source.
    #[serde(rename = "database")]
    pub database: Option<String>,

    /// Run data profiler as part of ingestion to get table profile data.
    #[serde(rename = "enableDataProfiler")]
    pub enable_data_profiler: Option<bool>,

    /// Regex exclude tables or databases that matches the pattern.
    #[serde(rename = "excludeFilterPattern")]
    pub exclude_filter_pattern: Option<Vec<String>>,

    /// Host and port of the data source.
    #[serde(rename = "host")]
    pub host: Option<String>,

    /// Regex to only fetch tables or databases that matches the pattern.
    #[serde(rename = "includeFilterPattern")]
    pub include_filter_pattern: Option<Vec<String>>,

    /// optional configuration to turn off fetching metadata for views.
    #[serde(rename = "includeViews")]
    pub include_views: Option<bool>,

    /// password to connect  to the data source.
    #[serde(rename = "password")]
    pub password: Option<String>,

    /// username to connect  to the data source.
    #[serde(rename = "username")]
    pub username: Option<String>,
}

/// This defines the runtime status of Ingestion.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngestionStatus {
    /// endDate of the Ingestion pipeline run for this particular execution.
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,

    /// startDate of the Ingestion Pipeline run for this particular execution.
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,

    /// Workflow status denotes if its failed or succeeded.
    #[serde(rename = "state")]
    pub state: Option<String>,
}

/// Create Chart entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateChartRequest {
    #[serde(rename = "chartType")]
    pub chart_type: Option<ChartType>,

    /// Chart URL, pointing to its own Service URL
    #[serde(rename = "chartUrl")]
    pub chart_url: Option<String>,

    /// Description of the chart instance. What it has and how to use it.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this Chart. It could be title or label from the source
    /// services
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Name that identifies this Chart.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this database
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Link to the database service where this database is hosted in
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Link to tables used in this chart.
    #[serde(rename = "tables")]
    pub tables: Option<Vec<ServiceElement>>,

    /// Tags for this chart
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,
}

/// Create Dashboard entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDashboardRequest {
    /// All the charts included in this Dashboard.
    #[serde(rename = "charts")]
    pub charts: Option<Vec<ServiceElement>>,

    /// Dashboard URL
    #[serde(rename = "dashboardUrl")]
    pub dashboard_url: Option<String>,

    /// Description of the database instance. What it has and how to use it.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this Dashboard. It could be title or label from the source
    /// services
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Name that identifies this dashboard.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this database
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Link to the database service where this database is hosted in
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Tags for this chart
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,
}

/// Create Database entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDatabaseRequest {
    /// Description of the database instance. What it has and how to use it.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Name that identifies this database instance uniquely.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this database
    #[serde(rename = "owner")]
    pub owner: Option<EntityReference>,

    /// Link to the database service where this database is hosted in
    #[serde(rename = "service")]
    pub service: EntityReference,
}

/// Create Ml Model entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateMlModelRequest {
    /// Algorithm used to train the ML Model
    #[serde(rename = "algorithm")]
    pub algorithm: String,

    /// Performance Dashboard URL to track metric evolution
    #[serde(rename = "dashboard")]
    pub dashboard: Option<ServiceElement>,

    /// Description of the ML model instance. How it was trained and for what it is used.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this ML model. It could be title or label from the source
    /// services
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Features used to train the ML Model.
    #[serde(rename = "mlFeatures")]
    pub ml_features: Option<Vec<MlFeature>>,

    /// Hyper Parameters used to train the ML Model.
    #[serde(rename = "mlHyperParameters")]
    pub ml_hyper_parameters: Option<Vec<MlHyperParameter>>,

    /// Location containing the ML Model. It can be a storage layer and/or a container repository.
    #[serde(rename = "mlStore")]
    pub ml_store: Option<MlStore>,

    /// Name that identifies this ML model.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this database
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Endpoint that makes the ML Model available, e.g,. a REST API serving the data or
    /// computing predictions.
    #[serde(rename = "server")]
    pub server: Option<String>,

    /// Tags for this ML Model
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,
}

/// Schema corresponding to a table that belongs to a database
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTableRequest {
    /// Name of the tables in the database
    #[serde(rename = "columns")]
    pub columns: Vec<CreateTableRequestColumn>,

    /// Database corresponding to this table
    #[serde(rename = "database")]
    pub database: String,

    /// Description of entity instance.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Name that identifies the this entity instance uniquely. Same as id if when name is not
    /// unique
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this entity
    #[serde(rename = "owner")]
    pub owner: Option<EntityReference>,

    #[serde(rename = "tableConstraints")]
    pub table_constraints: Option<Vec<CreateTableRequestTableConstraint>>,

    #[serde(rename = "tableType")]
    pub table_type: Option<TableType>,

    /// Tags for this table
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagLabel>>,

    /// View Definition in SQL. Applies to TableType.View only
    #[serde(rename = "viewDefinition")]
    pub view_definition: Option<String>,
}

/// This schema defines the type for a column in a table.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTableRequestColumn {
    /// Data type used array in dataType. For example, `array<int>` has dataType as `array` and
    /// arrayDataType as `int`.
    #[serde(rename = "arrayDataType")]
    pub array_data_type: Option<DataType>,

    /// Child columns if dataType or arrayDataType is `map`, `struct`, or `union` else `null`.
    #[serde(rename = "children")]
    pub children: Option<Vec<CreateTableRequestColumn>>,

    /// Column level constraint.
    #[serde(rename = "constraint")]
    pub constraint: Option<Constraint>,

    /// Length of `char`, `varchar`, `binary`, `varbinary` `dataTypes`, else null. For example,
    /// `varchar(20)` has dataType as `varchar` and dataLength as `20`.
    #[serde(rename = "dataLength")]
    pub data_length: Option<i64>,

    /// Data type of the column (int, date etc.).
    #[serde(rename = "dataType")]
    pub data_type: DataType,

    /// Display name used for dataType. This is useful for complex types, such as `array<int>,
    /// map<int,string>, struct<>, and union types.
    #[serde(rename = "dataTypeDisplay")]
    pub data_type_display: Option<String>,

    /// Description of the column.
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "fullyQualifiedName")]
    pub fully_qualified_name: Option<String>,

    /// Json schema only if the dataType is JSON else null.
    #[serde(rename = "jsonSchema")]
    pub json_schema: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    /// Ordinal position of the column.
    #[serde(rename = "ordinalPosition")]
    pub ordinal_position: Option<i64>,

    /// Tags associated with the column.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagLabel>>,
}

/// This enum defines the type for table constraint.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTableRequestTableConstraint {
    /// List of column names corresponding to the constraint.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<String>>,

    #[serde(rename = "constraintType")]
    pub constraint_type: Option<ConstraintType>,
}

/// Create Pipeline entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePipelineRequest {
    /// Concurrency of the Pipeline
    #[serde(rename = "concurrency")]
    pub concurrency: Option<i64>,

    /// Description of the database instance. What it has and how to use it.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this Pipeline. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Name that identifies this pipeline instance uniquely.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this database
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Pipeline Code Location
    #[serde(rename = "pipelineLocation")]
    pub pipeline_location: Option<String>,

    /// Pipeline  URL to visit/manage. This URL points to respective pipeline service UI
    #[serde(rename = "pipelineUrl")]
    pub pipeline_url: Option<String>,

    /// Link to the database service where this database is hosted in
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Start date of the workflow
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,

    /// Tags for this Pipeline.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// All the tasks that are part of pipeline.
    #[serde(rename = "tasks")]
    pub tasks: Option<Vec<Task>>,
}

/// Create a topic entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTopicRequest {
    /// Topic clean up policy. For Kafka - `cleanup.policy` configuration.
    #[serde(rename = "cleanupPolicies")]
    pub cleanup_policies: Option<Vec<CleanupPolicy>>,

    /// Description of the topic instance. What it has and how to use it.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Maximum message size in bytes. For Kafka - `max.message.bytes` configuration.
    #[serde(rename = "maximumMessageSize")]
    pub maximum_message_size: Option<i64>,

    /// Minimum number replicas in sync to control durability. For Kafka - `min.insync.replicas`
    /// configuration.
    #[serde(rename = "minimumInSyncReplicas")]
    pub minimum_in_sync_replicas: Option<i64>,

    /// Name that identifies this topic instance uniquely.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this topic
    #[serde(rename = "owner")]
    pub owner: Option<EntityReference>,

    /// Number of partitions into which the topic is divided.
    #[serde(rename = "partitions")]
    pub partitions: i64,

    /// Replication Factor in integer (more than 1).
    #[serde(rename = "replicationFactor")]
    pub replication_factor: Option<i64>,

    /// Maximum size of a partition in bytes before old data is discarded. For Kafka -
    /// `retention.bytes` configuration.
    #[serde(rename = "retentionSize")]
    pub retention_size: Option<f64>,

    /// Retention time in milliseconds. For Kafka - `retention.ms` configuration.
    #[serde(rename = "retentionTime")]
    pub retention_time: Option<f64>,

    /// Schema used for message serialization. Optional as some topics may not have associated
    /// schemas.
    #[serde(rename = "schemaText")]
    pub schema_text: Option<String>,

    /// Schema used for message serialization.
    #[serde(rename = "schemaType")]
    pub schema_type: Option<SchemaType>,

    /// Link to the messaging service where this topic is hosted in
    #[serde(rename = "service")]
    pub service: EntityReference,

    /// Tags for this topic
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagLabel>>,
}

/// Create Location entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    /// Description of the location instance.
    #[serde(rename = "description")]
    pub description: Option<String>,

    #[serde(rename = "locationType")]
    pub location_type: Option<LocationType>,

    /// Name that identifies this Location.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this Location
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// Link to the pipeline service where this location is used
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Tags for this location
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,
}

/// Create Messaging service entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateMessagingServiceRequest {
    /// Multiple bootstrap addresses for Kafka. Single proxy address for Pulsar.
    #[serde(rename = "brokers")]
    pub brokers: Vec<String>,

    /// Description of messaging service entity.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Schedule for running metadata ingestion jobs
    #[serde(rename = "ingestionSchedule")]
    pub ingestion_schedule: Option<Schedule>,

    /// Name that identifies the this entity instance uniquely
    #[serde(rename = "name")]
    pub name: String,

    /// Schema registry URL
    #[serde(rename = "schemaRegistry")]
    pub schema_registry: Option<String>,

    #[serde(rename = "serviceType")]
    pub service_type: MessagingServiceType,
}

/// Create Storage service entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateStorageServiceRequest {
    /// Description of Storage entity.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Name that identifies the this entity instance uniquely
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "serviceType")]
    pub service_type: Option<StorageServiceType>,
}

/// Create Dashboard service entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDashboardServiceRequest {
    /// Dashboard Service URL
    #[serde(rename = "dashboardUrl")]
    pub dashboard_url: String,

    /// Description of dashboard service entity.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Schedule for running metadata ingestion jobs
    #[serde(rename = "ingestionSchedule")]
    pub ingestion_schedule: Option<Schedule>,

    /// Name that identifies the this entity instance uniquely
    #[serde(rename = "name")]
    pub name: String,

    /// Password to log-into Dashboard Service
    #[serde(rename = "password")]
    pub password: Option<String>,

    #[serde(rename = "serviceType")]
    pub service_type: DashboardServiceType,

    /// Username to log-into Dashboard Service
    #[serde(rename = "username")]
    pub username: Option<String>,
}

/// Create Pipeline service entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePipelineServiceRequest {
    /// Description of pipeline service entity.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Schedule for running pipeline ingestion jobs
    #[serde(rename = "ingestionSchedule")]
    pub ingestion_schedule: Option<Schedule>,

    /// Name that identifies the this entity instance uniquely
    #[serde(rename = "name")]
    pub name: String,

    /// Pipeline UI URL
    #[serde(rename = "pipelineUrl")]
    pub pipeline_url: String,

    #[serde(rename = "serviceType")]
    pub service_type: PipelineServiceType,
}

/// Create Database service entity request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateDatabaseServiceRequest {
    /// Description of Database entity.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Schedule for running metadata ingestion jobs
    #[serde(rename = "ingestionSchedule")]
    pub ingestion_schedule: Option<Schedule>,

    #[serde(rename = "jdbc")]
    pub jdbc: JdbcInfo,

    /// Name that identifies the this entity instance uniquely
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "serviceType")]
    pub service_type: DatabaseServiceType,
}

/// Team entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTeamRequest {
    /// Optional description of the team
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Optional name used for display purposes. Example 'Marketing Team'
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    /// Optional team profile information
    #[serde(rename = "profile")]
    pub profile: Option<Profile>,

    /// Optional IDs of users that are part of the team
    #[serde(rename = "users")]
    pub users: Option<Vec<String>>,
}

/// This schema defines the type for a profile of a user, team, or organization.
///
/// Optional team profile information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(rename = "images")]
    pub images: Option<ImageList>,
}

/// Request to create User entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// Used for user biography.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Name used for display purposes. Example 'FirstName LastName'
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    #[serde(rename = "email")]
    pub email: String,

    /// When true indicates user is an administrator for the system with superuser privileges
    #[serde(rename = "isAdmin")]
    pub is_admin: Option<bool>,

    /// When true indicates user is a bot with appropriate privileges
    #[serde(rename = "isBot")]
    pub is_bot: Option<bool>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "profile")]
    pub profile: Option<Profile>,

    /// Teams that the user belongs to
    #[serde(rename = "teams")]
    pub teams: Option<Vec<String>>,

    /// Timezone of the user
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}

/// Create tag API request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTagRequest {
    /// Fully qualified names of tags associated with this tag
    #[serde(rename = "associatedTags")]
    pub associated_tags: Option<Vec<String>>,

    /// Unique name of the tag category
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "name")]
    pub name: String,
}

/// Create tag category request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTagCategoryRequest {
    #[serde(rename = "categoryType")]
    pub category_type: TagCategoryType,

    /// Description of the tag category
    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "name")]
    pub name: String,
}

/// Ingestion Config is used to setup a Airflow Ingestion pipeline.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateIngestionRequest {
    /// Concurrency of the Pipeline.
    #[serde(rename = "concurrency")]
    pub concurrency: Option<i64>,

    #[serde(rename = "connectorConfig")]
    pub connector_config: ConnectorConfig,

    /// Description of the workflow.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Display Name that identifies this Ingestion.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// End Date of the workflow.
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,

    /// Deploy the workflow by overwriting existing workflow with the same name.
    #[serde(rename = "forceDeploy")]
    pub force_deploy: Option<bool>,

    #[serde(rename = "ingestionType")]
    pub ingestion_type: Option<IngestionType>,

    /// Name that identifies this ingestion instance uniquely.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this Ingestion.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    /// pause the workflow from running once the deploy is finished successfully.
    #[serde(rename = "pauseWorkflow")]
    pub pause_workflow: Option<bool>,

    /// Retry workflow in case of failure
    #[serde(rename = "retries")]
    pub retries: Option<i64>,

    /// Delay between retries in seconds.
    #[serde(rename = "retryDelay")]
    pub retry_delay: Option<i64>,

    /// Scheduler Interval for the Workflow in cron format.
    #[serde(rename = "scheduleInterval")]
    pub schedule_interval: Option<String>,

    /// Link to the database service where this database is hosted in.
    #[serde(rename = "service")]
    pub service: ServiceElement,

    /// Start date of the workflow.
    #[serde(rename = "startDate")]
    pub start_date: String,

    /// Tags associated with the Ingestion.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<TagElement>>,

    /// Workflow catchup for past executions.
    #[serde(rename = "workflowCatchup")]
    pub workflow_catchup: Option<bool>,

    /// Timeout for the workflow in seconds.
    #[serde(rename = "workflowTimeout")]
    pub workflow_timeout: Option<i64>,

    /// Timezone in which workflow going to be scheduled.
    #[serde(rename = "workflowTimezone")]
    pub workflow_timezone: Option<String>,
}

/// Catalog application software version
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CatalogVersion {
    /// Software revision of the catalog
    #[serde(rename = "revision")]
    pub revision: Option<String>,

    /// Build timestamp
    #[serde(rename = "timestamp")]
    pub timestamp: Option<String>,

    /// Software version of the catalog
    #[serde(rename = "version")]
    pub version: Option<String>,
}

/// Set ownership for a given entity
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SetOwnershipRequest {
    /// Id of the owner of the entity
    #[serde(rename = "id")]
    pub id: Option<String>,

    /// Entity type of the owner typically either 'user' or 'team'
    #[serde(rename = "type")]
    pub set_ownership_request_type: Option<String>,
}

/// Create thread request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateThreadRequest {
    /// Data asset about which this thread is created for with format
    /// <#E/{entities}/{entityName}/{field}/{fieldValue}
    #[serde(rename = "about")]
    pub about: String,

    /// ID of User (regular user or bot) posting the message
    #[serde(rename = "from")]
    pub from: String,

    /// Message
    #[serde(rename = "message")]
    pub message: String,
}

/// Add lineage details between two entities
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddLineageRequest {
    /// User provided description of the lineage details.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Lineage edge details.
    #[serde(rename = "edge")]
    pub edge: EntitiesEdge,
}

/// Lineage edge details.
///
/// Edge in the lineage graph from one entity to another using entity references.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntitiesEdge {
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// From entity that is upstream of lineage edge.
    #[serde(rename = "fromEntity")]
    pub from_entity: Option<ServiceElement>,

    /// To entity that is downstream of lineage edge.
    #[serde(rename = "toEntity")]
    pub to_entity: Option<ServiceElement>,
}

/// Create Policy Entity Request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePolicyRequest {
    /// A short description of the Policy, comprehensible to regular users.
    #[serde(rename = "description")]
    pub description: Option<String>,

    /// Title for this Policy.
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// Name that identifies this Policy.
    #[serde(rename = "name")]
    pub name: String,

    /// Owner of this Policy.
    #[serde(rename = "owner")]
    pub owner: Option<ServiceElement>,

    #[serde(rename = "policyType")]
    pub policy_type: PolicyType,

    /// Link to a well documented definition of this Policy.
    #[serde(rename = "policyUrl")]
    pub policy_url: Option<String>,

    #[serde(rename = "rules")]
    pub rules: Option<Vec<CreatePolicyRequestRule>>,
}

/// A set of rules associated with the Policy.
///
/// Describes an entity Access Control Rule used within a Policy.
///
/// Describes an entity Lifecycle Rule used within a Policy.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatePolicyRequestRule {
    /// A set of access control enforcements to take on the entities.
    ///
    /// A set of actions to take on the entities.
    #[serde(rename = "actions")]
    pub actions: Vec<RuleAction>,

    /// Is the rule enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,

    #[serde(rename = "filters")]
    pub filters: Vec<Filter>,

    /// Name that identifies this Rule.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tag {
    AnythingArray(Vec<Option<serde_json::Value>>),

    Bool(bool),

    Double(f64),

    Integer(i64),

    String(String),

    TagClass(TagClass),
}

/// The set of filters that are used to match on entities. A logical AND operation is applied
/// across all filters.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Filter {
    String(String),

    TagLabel(TagLabel),
}

/// Service type where this pipeline is hosted in.
///
/// Type of pipeline service - Airflow or Prefect.
///
/// Type of pipeline service such as Airflow or Prefect...
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PipelineServiceType {
    #[serde(rename = "Airflow")]
    Airflow,

    #[serde(rename = "Glue")]
    Glue,

    #[serde(rename = "Prefect")]
    Prefect,
}

/// Label type describes how a tag label was applied. 'Manual' indicates the tag label was
/// applied by a person. 'Derived' indicates a tag label was derived using the associated tag
/// relationship (see TagCategory.json for more details). 'Propagated` indicates a tag label
/// was propagated from upstream based on lineage. 'Automated' is used when a tool was used
/// to determine the tag label.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LabelType {
    #[serde(rename = "Automated")]
    Automated,

    #[serde(rename = "Derived")]
    Derived,

    #[serde(rename = "Manual")]
    Manual,

    #[serde(rename = "Propagated")]
    Propagated,
}

/// 'Suggested' state is used when a tag label is suggested by users or tools. Owner of the
/// entity must confirm the suggested labels before it is marked as 'Confirmed'.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "Confirmed")]
    Confirmed,

    #[serde(rename = "Suggested")]
    Suggested,
}

/// Data type used array in dataType. For example, `array<int>` has dataType as `array` and
/// arrayDataType as `int`.
///
/// This enum defines the type of data stored in a column.
///
/// Data type of the column (int, date etc.).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DataType {
    #[serde(rename = "ARRAY")]
    Array,

    #[serde(rename = "BIGINT")]
    Bigint,

    #[serde(rename = "BINARY")]
    Binary,

    #[serde(rename = "BLOB")]
    Blob,

    #[serde(rename = "BOOLEAN")]
    Boolean,

    #[serde(rename = "BYTEINT")]
    Byteint,

    #[serde(rename = "BYTES")]
    Bytes,

    #[serde(rename = "CHAR")]
    Char,

    #[serde(rename = "DATE")]
    Date,

    #[serde(rename = "DATETIME")]
    Datetime,

    #[serde(rename = "DECIMAL")]
    Decimal,

    #[serde(rename = "DOUBLE")]
    Double,

    #[serde(rename = "ENUM")]
    Enum,

    #[serde(rename = "FLOAT")]
    Float,

    #[serde(rename = "GEOGRAPHY")]
    Geography,

    #[serde(rename = "INT")]
    Int,

    #[serde(rename = "INTERVAL")]
    Interval,

    #[serde(rename = "JSON")]
    Json,

    #[serde(rename = "LONGBLOB")]
    Longblob,

    #[serde(rename = "MAP")]
    Map,

    #[serde(rename = "MEDIUMBLOB")]
    Mediumblob,

    #[serde(rename = "MEDIUMTEXT")]
    Mediumtext,

    #[serde(rename = "NUMBER")]
    Number,

    #[serde(rename = "NUMERIC")]
    Numeric,

    #[serde(rename = "SET")]
    Set,

    #[serde(rename = "SMALLINT")]
    Smallint,

    #[serde(rename = "STRING")]
    String,

    #[serde(rename = "STRUCT")]
    Struct,

    #[serde(rename = "TEXT")]
    Text,

    #[serde(rename = "TIME")]
    Time,

    #[serde(rename = "TIMESTAMP")]
    Timestamp,

    #[serde(rename = "TINYINT")]
    Tinyint,

    #[serde(rename = "UNION")]
    Union,

    #[serde(rename = "VARBINARY")]
    Varbinary,

    #[serde(rename = "VARCHAR")]
    Varchar,
}

/// Column level constraint.
///
/// This enum defines the type for column constraint.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Constraint {
    #[serde(rename = "NOT_NULL")]
    NotNull,

    #[serde(rename = "NULL")]
    Null,

    #[serde(rename = "PRIMARY_KEY")]
    PrimaryKey,

    #[serde(rename = "UNIQUE")]
    Unique,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModelType {
    #[serde(rename = "DBT")]
    Dbt,
}

/// Service type this table is hosted in.
///
/// Type of database service such as MySQL, BigQuery, Snowflake, Redshift, Postgres...
///
/// Service type where this database is hosted in.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DatabaseServiceType {
    #[serde(rename = "Athena")]
    Athena,

    #[serde(rename = "BigQuery")]
    BigQuery,

    #[serde(rename = "Druid")]
    Druid,

    #[serde(rename = "Glue")]
    Glue,

    #[serde(rename = "Hive")]
    Hive,

    #[serde(rename = "MariaDB")]
    MariaDb,

    #[serde(rename = "MSSQL")]
    Mssql,

    #[serde(rename = "MySQL")]
    MySql,

    #[serde(rename = "Oracle")]
    Oracle,

    #[serde(rename = "Postgres")]
    Postgres,

    #[serde(rename = "Presto")]
    Presto,

    #[serde(rename = "Redshift")]
    Redshift,

    #[serde(rename = "Snowflake")]
    Snowflake,

    #[serde(rename = "Trino")]
    Trino,

    #[serde(rename = "Vertica")]
    Vertica,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConstraintType {
    #[serde(rename = "FOREIGN_KEY")]
    ForeignKey,

    #[serde(rename = "PRIMARY_KEY")]
    PrimaryKey,

    #[serde(rename = "UNIQUE")]
    Unique,
}

/// This schema defines the type used for describing different types of tables.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TableType {
    #[serde(rename = "External")]
    External,

    #[serde(rename = "MaterializedView")]
    MaterializedView,

    #[serde(rename = "Regular")]
    Regular,

    #[serde(rename = "SecureView")]
    SecureView,

    #[serde(rename = "View")]
    View,
}

/// Data type of the column (numerical vs. categorical).
///
/// This enum defines the type of data stored in a ML Feature.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FeatureType {
    #[serde(rename = "categorical")]
    Categorical,

    #[serde(rename = "numerical")]
    Numerical,
}

/// Data type of the source (int, date etc.).
///
/// This enum defines the type of data of a ML Feature source.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FeatureSourceDataType {
    #[serde(rename = "array")]
    Array,

    #[serde(rename = "boolean")]
    Boolean,

    #[serde(rename = "date")]
    Date,

    #[serde(rename = "integer")]
    Integer,

    #[serde(rename = "number")]
    Number,

    #[serde(rename = "object")]
    Object,

    #[serde(rename = "string")]
    String,

    #[serde(rename = "timestamp")]
    Timestamp,
}

/// Service type where this dashboard is hosted in.
///
/// Type of Dashboard service - Superset, Looker, Redash or Tableau.
///
/// Service type where this chart is hosted in.
///
/// Type of dashboard service such as Looker or Superset...
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DashboardServiceType {
    #[serde(rename = "Looker")]
    Looker,

    #[serde(rename = "Metabase")]
    Metabase,

    #[serde(rename = "Redash")]
    Redash,

    #[serde(rename = "Superset")]
    Superset,

    #[serde(rename = "Tableau")]
    Tableau,
}

/// This schema defines the type used for describing different types of charts.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ChartType {
    #[serde(rename = "Area")]
    Area,

    #[serde(rename = "Bar")]
    Bar,

    #[serde(rename = "BoxPlot")]
    BoxPlot,

    #[serde(rename = "Histogram")]
    Histogram,

    #[serde(rename = "Line")]
    Line,

    #[serde(rename = "Other")]
    Other,

    #[serde(rename = "Pie")]
    Pie,

    #[serde(rename = "Scatter")]
    Scatter,

    #[serde(rename = "Table")]
    Table,

    #[serde(rename = "Text")]
    Text,
}

/// Topic clean up policy. For Kafka - `cleanup.policy` configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CleanupPolicy {
    #[serde(rename = "compact")]
    Compact,

    #[serde(rename = "delete")]
    Delete,
}

/// Schema used for message serialization.
///
/// Schema type used for the message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SchemaType {
    #[serde(rename = "Avro")]
    Avro,

    #[serde(rename = "JSON")]
    Json,

    #[serde(rename = "Other")]
    Other,

    #[serde(rename = "Protobuf")]
    Protobuf,
}

/// Service type where this topic is hosted in.
///
/// Type of messaging service - Kafka or Pulsar.
///
/// Type of messaging service such as Kafka or Pulsar...
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessagingServiceType {
    #[serde(rename = "Kafka")]
    Kafka,

    #[serde(rename = "Pulsar")]
    Pulsar,
}

/// Type of tag category.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TagCategoryType {
    #[serde(rename = "Classification")]
    Classification,

    #[serde(rename = "Descriptive")]
    Descriptive,
}

/// This schema defines the type used for describing different types of Location.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LocationType {
    #[serde(rename = "Bucket")]
    Bucket,

    #[serde(rename = "Database")]
    Database,

    #[serde(rename = "Prefix")]
    Prefix,

    #[serde(rename = "Table")]
    Table,
}

/// Service type where this storage location is hosted in.
///
/// Type of storage service such as S3, GCS, HDFS...
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StorageServiceType {
    #[serde(rename = "ABFS")]
    Abfs,

    #[serde(rename = "GCS")]
    Gcs,

    #[serde(rename = "HDFS")]
    Hdfs,

    #[serde(rename = "S3")]
    S3,
}

/// The storage class to move this entity to.
///
/// Type of storage class for the storage service
///
/// Name of the entity field that changed.
///
/// Name of the field of an entity.
///
/// Link to the entity resource.
///
/// URI that points to a resource.
///
/// Link to the resource corresponding to this entity.
///
/// Link to the tag resource.
///
/// Link to this table resource.
///
/// Container Repository with the ML Model image.
///
/// Storage Layer containing the ML Model data.
///
/// Endpoint that makes the ML Model available, e.g,. a REST API serving the data or
/// computing predictions.
///
/// Link to this location resource.
///
/// Link to the resource corresponding to this report.
///
/// Link to the resource corresponding to this storage service.
///
/// Link to the resource corresponding to this messaging service.
///
/// Link to the resource corresponding to this database service.
///
/// Link to the resource corresponding to this dashboard service.
///
/// Link to the resource corresponding to this pipeline service.
///
/// Link to the resource corresponding to this bot.
///
/// Link to the resource corresponding to the tag.
///
/// Link to the resource corresponding to the tag category.
///
/// Link to this ingestion resource.
///
/// Unique identifier that identifies an entity instance.
///
/// Unique id used to identify an entity.
///
/// Unique identifier that identifies a pipeline instance.
///
/// Unique identifier of this table instance.
///
/// Unique identifier of an ML Model instance.
///
/// Unique identifier that identifies this database instance.
///
/// Unique identifier that identifies a dashboard instance.
///
/// Unique identifier that identifies a chart instance.
///
/// Unique identifier of this location instance.
///
/// Unique identifier that identifies this metrics instance.
///
/// Unique identifier that identifies this topic instance.
///
/// Unique identifier that identifies this report.
///
/// Unique identifier of this storage service instance.
///
/// Unique identifier of this messaging service instance.
///
/// Unique identifier of this database service instance.
///
/// Unique identifier of this dashboard service instance.
///
/// Unique identifier of this pipeline service instance.
///
/// Unique identifier that identifies a user entity instance.
///
/// Unique identifier of a bot instance.
///
/// ID of User (regular user or a bot) posting the message.
///
/// From entity that is upstream of lineage edge.
///
/// To entity that is downstream of lineage edge.
///
/// Identifier of entity that was modified by the operation.
///
/// Unique identifier that identifies this Ingestion.
///
/// Start date of the workflow.
///
/// Date and time in ISO 8601 format. Example - '2018-11-13T20:20:39+00:00'.
///
/// Last update time corresponding to the new version of the entity.
///
/// Start date and time of the schedule.
///
/// Timestamp of the when the first post created the thread.
///
/// Date and time when the change was made.
///
/// Date when the API call is made.
///
/// Start date of the workflow
///
/// SQL used in the task. Can be used to determine the lineage.
///
/// SQL query statement. Example - 'select * from orders'.
///
/// This corresponds to rws SQL from `<model_name>.sql` in DBT. This might be null when SQL
/// query need not be compiled as done in DBT.
///
/// This corresponds to compile SQL from `<model_name>.sql` in DBT. In cases where
/// compilation is not necessary, this corresponds to SQL that created the table.
///
/// View Definition in SQL. Applies to TableType.View only.
///
/// Date can be only from today going back to last 29 days.
///
/// Date in ISO 8601 format in UTC. Example - '2018-11-13'.
///
/// Data one which profile is taken.
///
/// Date on which the query ran.
///
/// Date in UTC.
///
/// End Date of the workflow.
///
/// Next execution date from the underlying workflow platform once the ingestion scheduled.
///
/// Multiple bootstrap addresses for Kafka. Single proxy address for Pulsar.
///
/// Repeat frequency in ISO 8601 duration format. Example - 'P23DT23H'.
///
/// Duration in ISO 8601 format in UTC. Example - 'P23DT23H'.
///
/// Type used for JDBC connection URL of format
/// `url_scheme://<username>:<password>@<host>:<port>/<db_name>`.
///
/// JDBC connection URL.
///
/// Type used for JDBC driver class.
///
/// JDBC driver class.
///
/// Prefix path of the entity.
///
/// Regex that matches the entity.
///
/// Unique identifier that identifies this Policy.
///
/// Database corresponding to this table
///
/// Id of the owner of the entity
///
/// ID of User (regular user or bot) posting the message
///
/// Last update time corresponding to the new version of the Policy.
///
/// View Definition in SQL. Applies to TableType.View only
///
/// Build timestamp
///
/// Timestamp in unixTimeMillis.
///
/// Type of storage class offered by S3
///
/// Type of storage class offered by GCS
///
/// Type of storage class offered by ABFS
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StorageClassType {
    #[serde(rename = "ARCHIVE")]
    Archive,

    #[serde(rename = "COLDLINE")]
    Coldline,

    #[serde(rename = "COOL")]
    Cool,

    #[serde(rename = "DEEP_ARCHIVE")]
    DeepArchive,

    #[serde(rename = "DURABLE_REDUCED_AVAILABILITY")]
    DurableReducedAvailability,

    #[serde(rename = "GLACIER")]
    Glacier,

    #[serde(rename = "HOT")]
    Hot,

    #[serde(rename = "INTELLIGENT_TIERING")]
    IntelligentTiering,

    #[serde(rename = "MULTI_REGIONAL")]
    MultiRegional,

    #[serde(rename = "NEARLINE")]
    Nearline,

    #[serde(rename = "ONEZONE_IA")]
    OnezoneIa,

    #[serde(rename = "OUTPOSTS")]
    Outposts,

    #[serde(rename = "REDUCED_REDUNDANCY")]
    ReducedRedundancy,

    #[serde(rename = "REGIONAL")]
    Regional,

    #[serde(rename = "STANDARD")]
    Standard,

    #[serde(rename = "STANDARD_IA")]
    StandardIa,
}

/// This schema defines the type used for describing different types of policies.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PolicyType {
    #[serde(rename = "AccessControl")]
    AccessControl,

    #[serde(rename = "Lifecycle")]
    Lifecycle,
}

/// Type of event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "entityCreated")]
    EntityCreated,

    #[serde(rename = "entityDeleted")]
    EntityDeleted,

    #[serde(rename = "entityUpdated")]
    EntityUpdated,
}

/// HTTP Method used in a call.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "DELETE")]
    Delete,

    #[serde(rename = "PATCH")]
    Patch,

    #[serde(rename = "POST")]
    Post,

    #[serde(rename = "PUT")]
    Put,
}

/// Type of Ingestion - Bigquery, Redshift, Snowflake etc...
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IngestionType {
    #[serde(rename = "bigquery")]
    Bigquery,

    #[serde(rename = "bigquery-usage")]
    BigqueryUsage,

    #[serde(rename = "hive")]
    Hive,

    #[serde(rename = "mssql")]
    Mssql,

    #[serde(rename = "mysql")]
    Mysql,

    #[serde(rename = "postgres")]
    Postgres,

    #[serde(rename = "presto")]
    Presto,

    #[serde(rename = "redshift")]
    Redshift,

    #[serde(rename = "redshift-usage")]
    RedshiftUsage,

    #[serde(rename = "snowflake")]
    Snowflake,

    #[serde(rename = "snowflake-usage")]
    SnowflakeUsage,

    #[serde(rename = "trino")]
    Trino,

    #[serde(rename = "vertica")]
    Vertica,
}
