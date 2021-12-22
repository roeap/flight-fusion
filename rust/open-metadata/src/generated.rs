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
#[derive(Serialize, Deserialize)]
pub struct Pipeline {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Concurrency of the Pipeline.
    concurrency: Option<i64>,
    /// Description of this Pipeline.
    description: Option<String>,
    /// Display Name that identifies this Pipeline. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Followers of this Pipeline.
    followers: Option<Vec<ServiceElement>>,
    /// A unique name that identifies a pipeline in the format 'ServiceName.PipelineName'.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to the resource corresponding to this entity.
    href: Option<String>,
    /// Unique identifier that identifies a pipeline instance.
    id: String,
    /// Name that identifies this pipeline instance uniquely.
    name: String,
    /// Owner of this pipeline.
    owner: Option<ServiceElement>,
    /// Pipeline Code Location.
    #[serde(rename = "pipelineLocation")]
    pipeline_location: Option<String>,
    /// Pipeline  URL to visit/manage. This URL points to respective pipeline service UI.
    #[serde(rename = "pipelineUrl")]
    pipeline_url: Option<String>,
    /// Link to service where this pipeline is hosted in.
    service: ServiceElement,
    /// Service type where this pipeline is hosted in.
    #[serde(rename = "serviceType")]
    service_type: Option<PipelineServiceType>,
    /// Start date of the workflow.
    #[serde(rename = "startDate")]
    start_date: Option<String>,
    /// Tags for this Pipeline.
    tags: Option<Vec<TagElement>>,
    /// All the tasks that are part of pipeline.
    tasks: Option<Vec<Task>>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// Change that lead to this version of the entity.
///
/// Description of the change.
///
/// Change that led to this version of the entity.
#[derive(Serialize, Deserialize)]
pub struct PipelineChangeDescription {
    /// Names of fields added during the version changes.
    #[serde(rename = "fieldsAdded")]
    fields_added: Option<Vec<PurpleFieldChange>>,
    /// Fields deleted during the version changes with old value before deleted.
    #[serde(rename = "fieldsDeleted")]
    fields_deleted: Option<Vec<PurpleFieldChange>>,
    /// Fields modified during the version changes with old and new values.
    #[serde(rename = "fieldsUpdated")]
    fields_updated: Option<Vec<PurpleFieldChange>>,
    /// When a change did not result in change, this could be same as the current version.
    #[serde(rename = "previousVersion")]
    previous_version: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleFieldChange {
    /// Name of the entity field that changed.
    name: Option<String>,
    /// New value of the field. Note that this is a JSON string and use the corresponding field
    /// type to deserialize it.
    #[serde(rename = "newValue")]
    new_value: Option<serde_json::Value>,
    /// Previous value of the field. Note that this is a JSON string and use the corresponding
    /// field type to deserialize it.
    #[serde(rename = "oldValue")]
    old_value: Option<serde_json::Value>,
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
#[derive(Serialize, Deserialize)]
pub struct ServiceElement {
    /// Optional description of entity.
    description: Option<String>,
    /// Display Name that identifies this entity.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Link to the entity resource.
    href: Option<String>,
    /// Unique identifier that identifies an entity instance.
    id: String,
    /// Name of the entity instance. For entities such as tables, databases where the name is not
    /// unique, fullyQualifiedName is returned in this field.
    name: Option<String>,
    /// Entity type/class name - Examples: `database`, `table`, `metrics`, `databaseService`,
    /// `dashboardService`...
    #[serde(rename = "type")]
    entity_reference_type: String,
}

/// This schema defines the type for labeling an entity with a Tag.
#[derive(Serialize, Deserialize)]
pub struct TagElement {
    /// Unique name of the tag category.
    description: Option<String>,
    /// Link to the tag resource.
    href: Option<String>,
    /// Label type describes how a tag label was applied. 'Manual' indicates the tag label was
    /// applied by a person. 'Derived' indicates a tag label was derived using the associated tag
    /// relationship (see TagCategory.json for more details). 'Propagated` indicates a tag label
    /// was propagated from upstream based on lineage. 'Automated' is used when a tool was used
    /// to determine the tag label.
    #[serde(rename = "labelType")]
    label_type: LabelType,
    /// 'Suggested' state is used when a tag label is suggested by users or tools. Owner of the
    /// entity must confirm the suggested labels before it is marked as 'Confirmed'.
    state: State,
    #[serde(rename = "tagFQN")]
    tag_fqn: String,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    /// Description of this Task.
    description: Option<String>,
    /// Display Name that identifies this Task. It could be title or label from the pipeline
    /// services.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// All the tasks that are downstream of this task.
    #[serde(rename = "downstreamTasks")]
    downstream_tasks: Option<Vec<String>>,
    /// A unique name that identifies a pipeline in the format
    /// 'ServiceName.PipelineName.TaskName'.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Name that identifies this task instance uniquely.
    name: String,
    /// Tags for this task.
    tags: Option<Vec<TagElement>>,
    /// SQL used in the task. Can be used to determine the lineage.
    #[serde(rename = "taskSQL")]
    task_sql: Option<String>,
    /// Type of the Task. Usually refers to the class it implements.
    #[serde(rename = "taskType")]
    task_type: Option<String>,
    /// Task URL to visit/manage. This URL points to respective pipeline service UI.
    #[serde(rename = "taskUrl")]
    task_url: Option<String>,
    id: Option<serde_json::Value>,
}

/// This schema defines the Table entity. A Table organizes data in rows and columns and is
/// defined by a Schema. OpenMetadata does not have a separate abstraction for Schema. Both
/// Table and Schema are captured in this entity.
#[derive(Serialize, Deserialize)]
pub struct Table {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Columns in this table.
    columns: Vec<TableColumn>,
    /// Reference to Database that contains this table.
    database: Option<ServiceElement>,
    /// This captures information about how the table is modeled. Currently only DBT model is
    /// supported.
    #[serde(rename = "dataModel")]
    data_model: Option<DataModel>,
    /// Description of a table.
    description: Option<String>,
    /// Display Name that identifies this table. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Followers of this table.
    followers: Option<Vec<ServiceElement>>,
    /// Fully qualified name of a table in the form `serviceName.databaseName.tableName`.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to this table resource.
    href: Option<String>,
    /// Unique identifier of this table instance.
    id: String,
    /// Details of other tables this table is frequently joined with.
    joins: Option<TableJoins>,
    /// Reference to the Location that contains this table.
    location: Option<ServiceElement>,
    /// Name of a table. Expected to be unique within a database.
    name: String,
    /// Owner of this table.
    owner: Option<ServiceElement>,
    /// Sample data for a table.
    #[serde(rename = "sampleData")]
    sample_data: Option<TableData>,
    /// Link to Database service this table is hosted in.
    service: Option<ServiceElement>,
    /// Service type this table is hosted in.
    #[serde(rename = "serviceType")]
    service_type: Option<DatabaseServiceType>,
    /// Table constraints.
    #[serde(rename = "tableConstraints")]
    table_constraints: Option<Vec<TableTableConstraint>>,
    /// Data profile for a table.
    #[serde(rename = "tableProfile")]
    table_profile: Option<Vec<TableProfile>>,
    /// List of queries that ran against a table.
    #[serde(rename = "tableQueries")]
    table_queries: Option<Vec<SqlQuery>>,
    #[serde(rename = "tableType")]
    table_type: Option<TableType>,
    /// Tags for this table.
    tags: Option<Vec<TagElement>>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Latest usage information for this table.
    #[serde(rename = "usageSummary")]
    usage_summary: Option<UsageSummaryElement>,
    /// Metadata version of the entity.
    version: Option<f64>,
    /// View Definition in SQL. Applies to TableType.View only.
    #[serde(rename = "viewDefinition")]
    view_definition: Option<String>,
}

/// This schema defines the type for a column in a table.
#[derive(Serialize, Deserialize)]
pub struct TableColumn {
    /// Data type used array in dataType. For example, `array<int>` has dataType as `array` and
    /// arrayDataType as `int`.
    #[serde(rename = "arrayDataType")]
    array_data_type: Option<DataType>,
    /// Child columns if dataType or arrayDataType is `map`, `struct`, or `union` else `null`.
    children: Option<Vec<TableColumn>>,
    /// Column level constraint.
    constraint: Option<Constraint>,
    /// Length of `char`, `varchar`, `binary`, `varbinary` `dataTypes`, else null. For example,
    /// `varchar(20)` has dataType as `varchar` and dataLength as `20`.
    #[serde(rename = "dataLength")]
    data_length: Option<i64>,
    /// Data type of the column (int, date etc.).
    #[serde(rename = "dataType")]
    data_type: DataType,
    /// Display name used for dataType. This is useful for complex types, such as `array<int>,
    /// map<int,string>, struct<>, and union types.
    #[serde(rename = "dataTypeDisplay")]
    data_type_display: Option<String>,
    /// Description of the column.
    description: Option<String>,
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Json schema only if the dataType is JSON else null.
    #[serde(rename = "jsonSchema")]
    json_schema: Option<String>,
    name: String,
    /// Ordinal position of the column.
    #[serde(rename = "ordinalPosition")]
    ordinal_position: Option<i64>,
    /// Tags associated with the column.
    tags: Option<Vec<TagElement>>,
}

/// This captures information about how the table is modeled. Currently only DBT model is
/// supported.
#[derive(Serialize, Deserialize)]
pub struct DataModel {
    /// Columns from the schema defined during modeling. In case of DBT, the metadata here comes
    /// from `schema.yaml`.
    columns: Option<Vec<TableColumn>>,
    /// Description of the Table from the model.
    description: Option<String>,
    #[serde(rename = "generatedAt")]
    generated_at: Option<String>,
    #[serde(rename = "modelType")]
    model_type: ModelType,
    /// Path to sql definition file.
    path: Option<String>,
    /// This corresponds to rws SQL from `<model_name>.sql` in DBT. This might be null when SQL
    /// query need not be compiled as done in DBT.
    #[serde(rename = "rawSql")]
    raw_sql: Option<String>,
    /// This corresponds to compile SQL from `<model_name>.sql` in DBT. In cases where
    /// compilation is not necessary, this corresponds to SQL that created the table.
    sql: String,
    /// Fully qualified name of Models/tables used for in `sql` for creating this table.
    upstream: Option<Vec<String>>,
}

/// Details of other tables this table is frequently joined with.
///
/// This schema defines the type to capture information about how columns in this table are
/// joined with columns in the other tables.
#[derive(Serialize, Deserialize)]
pub struct TableJoins {
    #[serde(rename = "columnJoins")]
    column_joins: Option<Vec<ColumnJoins>>,
    #[serde(rename = "dayCount")]
    day_count: Option<i64>,
    /// Date can be only from today going back to last 29 days.
    #[serde(rename = "startDate")]
    start_date: Option<String>,
}

/// This schema defines the type to capture how frequently a column is joined with columns in
/// the other tables.
#[derive(Serialize, Deserialize)]
pub struct ColumnJoins {
    #[serde(rename = "columnName")]
    column_name: Option<String>,
    /// Fully qualified names of the columns that this column is joined with.
    #[serde(rename = "joinedWith")]
    joined_with: Option<Vec<JoinedWith>>,
}

#[derive(Serialize, Deserialize)]
pub struct JoinedWith {
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    #[serde(rename = "joinCount")]
    join_count: Option<i64>,
}

/// Sample data for a table.
///
/// This schema defines the type to capture rows of sample data for a table.
#[derive(Serialize, Deserialize)]
pub struct TableData {
    /// List of local column names (not fully qualified column names) of the table.
    columns: Option<Vec<String>>,
    /// Data for multiple rows of the table.
    rows: Option<Vec<Vec<Option<serde_json::Value>>>>,
}

/// This enum defines the type for table constraint.
#[derive(Serialize, Deserialize)]
pub struct TableTableConstraint {
    /// List of column names corresponding to the constraint.
    columns: Option<Vec<String>>,
    #[serde(rename = "constraintType")]
    constraint_type: Option<ConstraintType>,
}

/// This schema defines the type to capture the table's data profile.
#[derive(Serialize, Deserialize)]
pub struct TableProfile {
    /// No.of columns in the table.
    #[serde(rename = "columnCount")]
    column_count: Option<f64>,
    /// List of local column profiles of the table.
    #[serde(rename = "columnProfile")]
    column_profile: Option<Vec<ColumnProfile>>,
    /// Data one which profile is taken.
    #[serde(rename = "profileDate")]
    profile_date: Option<String>,
    /// No.of rows in the table.
    #[serde(rename = "rowCount")]
    row_count: Option<f64>,
}

/// This schema defines the type to capture the table's column profile.
#[derive(Serialize, Deserialize)]
pub struct ColumnProfile {
    /// Maximum value in a column.
    max: Option<String>,
    /// Avg value in a column.
    mean: Option<String>,
    /// Median value in a column.
    median: Option<String>,
    /// Minimum value in a column.
    min: Option<String>,
    /// Column Name.
    name: Option<String>,
    /// No.of null values in a column.
    #[serde(rename = "nullCount")]
    null_count: Option<f64>,
    /// No.of null value proportion in columns.
    #[serde(rename = "nullProportion")]
    null_proportion: Option<f64>,
    /// Standard deviation of a column.
    stddev: Option<f64>,
    /// No. of unique values in the column.
    #[serde(rename = "uniqueCount")]
    unique_count: Option<f64>,
    /// Proportion of number of unique values in a column.
    #[serde(rename = "uniqueProportion")]
    unique_proportion: Option<f64>,
}

/// This schema defines the type to capture the table's sql queries.
#[derive(Serialize, Deserialize)]
pub struct SqlQuery {
    /// Checksum to avoid registering duplicate queries.
    checksum: Option<String>,
    /// How long did the query took to run in seconds.
    duration: Option<f64>,
    /// SQL Query text that matches the table name.
    query: Option<String>,
    /// Date on which the query ran.
    #[serde(rename = "queryDate")]
    query_date: Option<String>,
    /// User who ran this query.
    user: Option<ServiceElement>,
    /// Users can vote up to rank the popular queries.
    vote: Option<f64>,
}

/// Latest usage information for this table.
///
/// This schema defines the type for usage details. Daily, weekly, and monthly aggregation of
/// usage is computed along with the percentile rank based on the usage for a given day.
///
/// Latest usage information for this ML Model.
///
/// Latest usage information for this database.
#[derive(Serialize, Deserialize)]
pub struct UsageSummaryElement {
    /// Daily usage stats of a data asset on the start date.
    #[serde(rename = "dailyStats")]
    daily_stats: UsageStats,
    /// Date in UTC.
    date: String,
    /// Monthly (last 30 days) rolling usage stats of a data asset on the start date.
    #[serde(rename = "monthlyStats")]
    monthly_stats: Option<UsageStats>,
    /// Weekly (last 7 days) rolling usage stats of a data asset on the start date.
    #[serde(rename = "weeklyStats")]
    weekly_stats: Option<UsageStats>,
}

/// Daily usage stats of a data asset on the start date.
///
/// Type used to return usage statistics.
///
/// Monthly (last 30 days) rolling usage stats of a data asset on the start date.
///
/// Weekly (last 7 days) rolling usage stats of a data asset on the start date.
#[derive(Serialize, Deserialize)]
pub struct UsageStats {
    /// Usage count of a data asset on the start date.
    count: i64,
    /// Optional daily percentile rank data asset use when relevant.
    #[serde(rename = "percentileRank")]
    percentile_rank: Option<f64>,
}

/// This schema defines the Model entity. Models are algorithms trained on data to find
/// patterns or make predictions.
#[derive(Serialize, Deserialize)]
pub struct MlModel {
    /// Algorithm used to train the ML Model.
    algorithm: String,
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Performance Dashboard URL to track metric evolution.
    dashboard: Option<ServiceElement>,
    /// Description of the ML Model, what it is, and how to use it.
    description: Option<String>,
    /// Display Name that identifies this ML Model.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Followers of this ML Model.
    followers: Option<Vec<ServiceElement>>,
    /// A unique name that identifies an ML Model.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to the resource corresponding to this entity.
    href: Option<String>,
    /// Unique identifier of an ML Model instance.
    id: String,
    /// Features used to train the ML Model.
    #[serde(rename = "mlFeatures")]
    ml_features: Option<Vec<MlFeature>>,
    /// Hyper Parameters used to train the ML Model.
    #[serde(rename = "mlHyperParameters")]
    ml_hyper_parameters: Option<Vec<MlHyperParameter>>,
    /// Location containing the ML Model. It can be a storage layer and/or a container repository.
    #[serde(rename = "mlStore")]
    ml_store: Option<MlStore>,
    /// Name that identifies this ML Model.
    name: String,
    /// Owner of this ML Model.
    owner: Option<ServiceElement>,
    /// Endpoint that makes the ML Model available, e.g,. a REST API serving the data or
    /// computing predictions.
    server: Option<String>,
    /// Tags for this ML Model.
    tags: Option<Vec<TagElement>>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Latest usage information for this ML Model.
    #[serde(rename = "usageSummary")]
    usage_summary: Option<UsageSummaryElement>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the type for an ML Feature used in an ML Model.
#[derive(Serialize, Deserialize)]
pub struct MlFeature {
    /// Data type of the column (numerical vs. categorical).
    #[serde(rename = "dataType")]
    data_type: Option<FeatureType>,
    /// Description of the ML Feature.
    description: Option<String>,
    /// Description of the algorithm used to compute the feature, e.g., PCA, bucketing...
    #[serde(rename = "featureAlgorithm")]
    feature_algorithm: Option<String>,
    /// Columns used to create the ML Feature.
    #[serde(rename = "featureSources")]
    feature_sources: Option<Vec<FeatureSource>>,
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    name: Option<String>,
    /// Tags associated with the feature.
    tags: Option<Vec<TagElement>>,
}

/// This schema defines the sources of a ML Feature.
#[derive(Serialize, Deserialize)]
pub struct FeatureSource {
    /// Data type of the source (int, date etc.).
    #[serde(rename = "dataType")]
    data_type: Option<FeatureSourceDataType>,
    /// Description of the feature source.
    description: Option<String>,
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    name: Option<String>,
    /// Tags associated with the feature source.
    tags: Option<Vec<TagElement>>,
}

/// This schema defines the type for an ML HyperParameter used in an ML Model.
#[derive(Serialize, Deserialize)]
pub struct MlHyperParameter {
    /// Description of the Hyper Parameter.
    description: Option<String>,
    /// Hyper parameter name.
    name: Option<String>,
    /// Hyper parameter value.
    value: Option<String>,
}

/// Location containing the ML Model. It can be a storage layer and/or a container repository.
#[derive(Serialize, Deserialize)]
pub struct MlStore {
    /// Container Repository with the ML Model image.
    #[serde(rename = "imageRepository")]
    image_repository: Option<String>,
    /// Storage Layer containing the ML Model data.
    storage: Option<String>,
}

/// This schema defines the Database entity. A database also referred to as Database Catalog
/// is a collection of tables.
#[derive(Serialize, Deserialize)]
pub struct Database {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Description of the database instance.
    description: Option<String>,
    /// Display Name that identifies this database.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Name that uniquely identifies a database in the format 'ServiceName.DatabaseName'.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to the resource corresponding to this entity.
    href: Option<String>,
    /// Unique identifier that identifies this database instance.
    id: Option<String>,
    /// Reference to the Location that contains this database.
    location: Option<ServiceElement>,
    /// Name that identifies the database.
    name: String,
    /// Owner of this database.
    owner: Option<ServiceElement>,
    /// Link to the database cluster/service where this database is hosted in.
    service: ServiceElement,
    /// Service type where this database is hosted in.
    #[serde(rename = "serviceType")]
    service_type: Option<DatabaseServiceType>,
    /// References to tables in the database.
    tables: Option<Vec<ServiceElement>>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Latest usage information for this database.
    #[serde(rename = "usageSummary")]
    usage_summary: Option<UsageSummaryElement>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the Dashboard entity. Dashboards are computed from data and visually
/// present data, metrics, and KPIs. They are updated in real-time and allow interactive data
/// exploration.
#[derive(Serialize, Deserialize)]
pub struct Dashboard {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// All the charts included in this Dashboard.
    charts: Option<Vec<ServiceElement>>,
    /// Dashboard URL.
    #[serde(rename = "dashboardUrl")]
    dashboard_url: Option<String>,
    /// Description of the dashboard, what it is, and how to use it.
    description: Option<String>,
    /// Display Name that identifies this Dashboard. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Followers of this dashboard.
    followers: Option<Vec<ServiceElement>>,
    /// A unique name that identifies a dashboard in the format 'ServiceName.DashboardName'.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to the resource corresponding to this entity.
    href: Option<String>,
    /// Unique identifier that identifies a dashboard instance.
    id: String,
    /// Name that identifies this dashboard.
    name: String,
    /// Owner of this dashboard.
    owner: Option<ServiceElement>,
    /// Link to service where this dashboard is hosted in.
    service: ServiceElement,
    /// Service type where this dashboard is hosted in.
    #[serde(rename = "serviceType")]
    service_type: Option<DashboardServiceType>,
    /// Tags for this dashboard.
    tags: Option<Vec<TagElement>>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Latest usage information for this database.
    #[serde(rename = "usageSummary")]
    usage_summary: Option<UsageSummaryElement>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the Chart entity. Charts are built using tables or sql queries by
/// analyzing the data. Charts can be part of Dashboard.
#[derive(Serialize, Deserialize)]
pub struct Chart {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    #[serde(rename = "chartType")]
    chart_type: Option<ChartType>,
    /// Chart URL, pointing to its own Service URL.
    #[serde(rename = "chartUrl")]
    chart_url: Option<String>,
    /// Description of the dashboard, what it is, and how to use it.
    description: Option<String>,
    /// Display Name that identifies this Chart. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Followers of this chart.
    followers: Option<Vec<ServiceElement>>,
    /// A unique name that identifies a dashboard in the format 'ServiceName.ChartName'.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to the resource corresponding to this entity.
    href: Option<String>,
    /// Unique identifier that identifies a chart instance.
    id: String,
    /// Name that identifies this Chart.
    name: String,
    /// Owner of this dashboard.
    owner: Option<ServiceElement>,
    /// Link to service where this dashboard is hosted in.
    service: ServiceElement,
    /// Service type where this chart is hosted in.
    #[serde(rename = "serviceType")]
    service_type: Option<DashboardServiceType>,
    /// Link to table used in this chart.
    tables: Option<Vec<ServiceElement>>,
    /// Tags for this chart.
    tags: Option<Vec<TagElement>>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Latest usage information for this database.
    #[serde(rename = "usageSummary")]
    usage_summary: Option<UsageSummaryElement>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the Metrics entity. Metrics are measurements computed from data such
/// as `Monthly Active Users`. Some of the metrics that measures used to determine
/// performance against an objective are called KPIs or Key Performance Indicators, such as
/// `User Retention`.
#[derive(Serialize, Deserialize)]
pub struct Metrics {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Description of metrics instance, what it is, and how to use it.
    description: Option<String>,
    /// Display Name that identifies this metric.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// A unique name that identifies a metric in the format 'ServiceName.MetricName'.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to the resource corresponding to this entity.
    href: Option<String>,
    /// Unique identifier that identifies this metrics instance.
    id: String,
    /// Name that identifies this metrics instance uniquely.
    name: String,
    /// Owner of this metrics.
    owner: Option<ServiceElement>,
    /// Link to service where this metrics is hosted in.
    service: ServiceElement,
    /// Tags for this chart.
    tags: Option<Vec<TagElement>>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Latest usage information for this database.
    #[serde(rename = "usageSummary")]
    usage_summary: Option<UsageSummaryElement>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the Topic entity. A topic is a feed into which message are published
/// to by publishers and read from by consumers in a messaging service.
#[derive(Serialize, Deserialize)]
pub struct Topic {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Topic clean up policies. For Kafka - `cleanup.policy` configuration.
    #[serde(rename = "cleanupPolicies")]
    cleanup_policies: Option<Vec<CleanupPolicy>>,
    /// Description of the topic instance.
    description: Option<String>,
    /// Display Name that identifies this topic. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Followers of this table.
    followers: Option<Vec<ServiceElement>>,
    /// Name that uniquely identifies a topic in the format 'messagingServiceName.topicName'.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to the resource corresponding to this entity.
    href: Option<String>,
    /// Unique identifier that identifies this topic instance.
    id: String,
    /// Maximum message size in bytes. For Kafka - `max.message.bytes` configuration.
    #[serde(rename = "maximumMessageSize")]
    maximum_message_size: Option<i64>,
    /// Minimum number replicas in sync to control durability. For Kafka - `min.insync.replicas`
    /// configuration.
    #[serde(rename = "minimumInSyncReplicas")]
    minimum_in_sync_replicas: Option<i64>,
    /// Name that identifies the topic.
    name: String,
    /// Owner of this topic.
    owner: Option<ServiceElement>,
    /// Number of partitions into which the topic is divided.
    partitions: i64,
    /// Replication Factor in integer (more than 1).
    #[serde(rename = "replicationFactor")]
    replication_factor: Option<i64>,
    /// Maximum size of a partition in bytes before old data is discarded. For Kafka -
    /// `retention.bytes` configuration.
    #[serde(rename = "retentionSize")]
    retention_size: Option<f64>,
    /// Retention time in milliseconds. For Kafka - `retention.ms` configuration.
    #[serde(rename = "retentionTime")]
    retention_time: Option<f64>,
    /// Schema used for message serialization. Optional as some topics may not have associated
    /// schemas.
    #[serde(rename = "schemaText")]
    schema_text: Option<String>,
    /// Schema used for message serialization.
    #[serde(rename = "schemaType")]
    schema_type: Option<SchemaType>,
    /// Link to the messaging cluster/service where this topic is hosted in.
    service: ServiceElement,
    /// Service type where this topic is hosted in.
    #[serde(rename = "serviceType")]
    service_type: Option<MessagingServiceType>,
    /// Tags for this table.
    tags: Option<Vec<TagElement>>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the Report entity. Reports are static information computed from data
/// periodically that includes data in text, table, and visual form.
#[derive(Serialize, Deserialize)]
pub struct Report {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Description of this report instance.
    description: Option<String>,
    /// Display Name that identifies this report. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// A unique name that identifies a report in the format 'ServiceName.ReportName'.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to the resource corresponding to this report.
    href: Option<String>,
    /// Unique identifier that identifies this report.
    id: String,
    /// Name that identifies this report instance uniquely.
    name: String,
    /// Owner of this pipeline.
    owner: Option<ServiceElement>,
    /// Link to service where this report is hosted in.
    service: ServiceElement,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Latest usage information for this database.
    #[serde(rename = "usageSummary")]
    usage_summary: Option<UsageSummaryElement>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the Messaging Service entity, such as Kafka and Pulsar.
#[derive(Serialize, Deserialize)]
pub struct MessagingService {
    /// Multiple bootstrap addresses for Kafka. Single proxy address for Pulsar.
    brokers: Vec<String>,
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Description of a messaging service instance.
    description: Option<String>,
    /// Display Name that identifies this messaging service. It could be title or label from the
    /// source services.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Link to the resource corresponding to this messaging service.
    href: Option<String>,
    /// Unique identifier of this messaging service instance.
    id: String,
    /// Schedule for running metadata ingestion jobs.
    #[serde(rename = "ingestionSchedule")]
    ingestion_schedule: Option<IngestionScheduleClass>,
    /// Name that identifies this messaging service.
    name: String,
    /// Schema registry URL.
    #[serde(rename = "schemaRegistry")]
    schema_registry: Option<String>,
    /// Type of messaging service such as Kafka or Pulsar...
    #[serde(rename = "serviceType")]
    service_type: MessagingServiceType,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// Schedule for running metadata ingestion jobs.
///
/// This schema defines the type used for the schedule. The schedule has a start time and
/// repeat frequency.
///
/// Schedule for running metadata ingestion jobs
///
/// Schedule for running pipeline ingestion jobs
#[derive(Serialize, Deserialize)]
pub struct IngestionScheduleClass {
    /// Repeat frequency in ISO 8601 duration format. Example - 'P23DT23H'.
    #[serde(rename = "repeatFrequency")]
    repeat_frequency: Option<String>,
    /// Start date and time of the schedule.
    #[serde(rename = "startDate")]
    start_date: Option<String>,
}

/// This schema defines the Database Service entity, such as MySQL, BigQuery, Redshift,
/// Postgres, or Snowflake. Alternative terms such as Database Cluster, Database Server
/// instance are also used for database service.
#[derive(Serialize, Deserialize)]
pub struct DatabaseService {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Description of a database service instance.
    description: Option<String>,
    /// Display Name that identifies this database service.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Link to the resource corresponding to this database service.
    href: String,
    /// Unique identifier of this database service instance.
    id: String,
    /// Schedule for running metadata ingestion jobs.
    #[serde(rename = "ingestionSchedule")]
    ingestion_schedule: Option<IngestionScheduleClass>,
    /// JDBC connection information.
    jdbc: JdbcInfo,
    /// Name that identifies this database service.
    name: String,
    /// Type of database service such as MySQL, BigQuery, Snowflake, Redshift, Postgres...
    #[serde(rename = "serviceType")]
    service_type: DatabaseServiceType,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// JDBC connection information.
///
/// Type for capturing JDBC connector information.
#[derive(Serialize, Deserialize)]
pub struct JdbcInfo {
    #[serde(rename = "connectionUrl")]
    connection_url: String,
    #[serde(rename = "driverClass")]
    driver_class: String,
}

/// This schema defines the Dashboard Service entity, such as Looker and Superset.
#[derive(Serialize, Deserialize)]
pub struct DashboardService {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Dashboard Service URL. This will be used to make REST API calls to Dashboard Service.
    #[serde(rename = "dashboardUrl")]
    dashboard_url: String,
    /// Description of a dashboard service instance.
    description: Option<String>,
    /// Display Name that identifies this dashboard service.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Link to the resource corresponding to this dashboard service.
    href: Option<String>,
    /// Unique identifier of this dashboard service instance.
    id: String,
    /// Schedule for running metadata ingestion jobs.
    #[serde(rename = "ingestionSchedule")]
    ingestion_schedule: Option<IngestionScheduleClass>,
    /// Name that identifies this dashboard service.
    name: String,
    /// Password to log-into Dashboard Service.
    password: Option<String>,
    /// Type of dashboard service such as Looker or Superset...
    #[serde(rename = "serviceType")]
    service_type: DashboardServiceType,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Username to log-into Dashboard Service.
    username: Option<String>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the Pipeline Service entity, such as Airflow and Prefect.
#[derive(Serialize, Deserialize)]
pub struct PipelineService {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Description of a pipeline service instance.
    description: Option<String>,
    /// Display Name that identifies this pipeline service. It could be title or label from the
    /// source services.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Link to the resource corresponding to this pipeline service.
    href: Option<String>,
    /// Unique identifier of this pipeline service instance.
    id: String,
    /// Schedule for running metadata ingestion jobs.
    #[serde(rename = "ingestionSchedule")]
    ingestion_schedule: Option<IngestionScheduleClass>,
    /// Name that identifies this pipeline service.
    name: String,
    /// Pipeline Service Management/UI URL.
    #[serde(rename = "pipelineUrl")]
    pipeline_url: String,
    /// Type of pipeline service such as Airflow or Prefect...
    #[serde(rename = "serviceType")]
    service_type: Option<PipelineServiceType>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the User entity. A user can be part of 0 or more teams. A special
/// type of user called Bot is used for automation. A user can be an owner of zero or more
/// data assets. A user can also follow zero or more data assets.
#[derive(Serialize, Deserialize)]
pub struct User {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// When true indicates the user has been deactivated. Users are deactivated instead of
    /// deleted.
    deactivated: Option<bool>,
    /// Used for user biography.
    description: Option<String>,
    /// Name used for display purposes. Example 'FirstName LastName'.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Email address of the user.
    email: String,
    /// List of entities followed by the user.
    follows: Option<Vec<ServiceElement>>,
    /// Link to the resource corresponding to this entity.
    href: String,
    /// Unique identifier that identifies a user entity instance.
    id: String,
    /// When true indicates user is an administrator for the system with superuser privileges.
    #[serde(rename = "isAdmin")]
    is_admin: Option<bool>,
    /// When true indicates a special type of user called Bot.
    #[serde(rename = "isBot")]
    is_bot: Option<bool>,
    name: String,
    /// List of entities owned by the user.
    owns: Option<Vec<ServiceElement>>,
    /// Profile of the user.
    profile: Option<ProfileClass>,
    /// Teams that the user belongs to.
    teams: Option<Vec<ServiceElement>>,
    /// Timezone of the user.
    timezone: Option<String>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// Profile of the user.
///
/// This schema defines the type for a profile of a user, team, or organization.
///
/// Team profile information.
#[derive(Serialize, Deserialize)]
pub struct ProfileClass {
    images: Option<ImageList>,
}

/// Links to a list of images of varying resolutions/sizes.
#[derive(Serialize, Deserialize)]
pub struct ImageList {
    image: Option<String>,
    image192: Option<String>,
    image24: Option<String>,
    image32: Option<String>,
    image48: Option<String>,
    image512: Option<String>,
    image72: Option<String>,
}

/// This schema defines the Team entity. A Team is a group of zero or more users. Teams can
/// own zero or more data assets.
#[derive(Serialize, Deserialize)]
pub struct Team {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// When true the team has been deleted.
    deleted: Option<bool>,
    /// Description of the team.
    description: Option<String>,
    /// Name used for display purposes. Example 'Data Science team'.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Link to the resource corresponding to this entity.
    href: String,
    id: String,
    name: String,
    /// List of entities owned by the team.
    owns: Option<Vec<ServiceElement>>,
    /// Team profile information.
    profile: Option<ProfileClass>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Users that are part of the team.
    users: Option<Vec<ServiceElement>>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines Bot entity. A bot automates tasks, such as adding description,
/// identifying the importance of data. It runs as a special user in the system.
#[derive(Serialize, Deserialize)]
pub struct Bot {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Description of the bot.
    description: Option<String>,
    /// Name used for display purposes. Example 'FirstName LastName'.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Link to the resource corresponding to this bot.
    href: Option<String>,
    /// Unique identifier of a bot instance.
    id: Option<String>,
    /// Name of the bot.
    name: Option<String>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the Tag Category entity. A Tag Category contains tags called Primary
/// Tags. Primary Tags can further have children Tags called Secondary Tags. Only two levels
/// of tags are supported currently.
#[derive(Serialize, Deserialize)]
pub struct TagCategory {
    #[serde(rename = "categoryType")]
    category_type: TagCategoryType,
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Tags under this category.
    children: Option<Vec<Option<Tag>>>,
    /// Description of the tag category.
    description: String,
    /// Display Name that identifies this tag category.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Link to the resource corresponding to the tag category.
    href: Option<String>,
    name: String,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Count of how many times the tags from this tag category are used.
    #[serde(rename = "usageCount")]
    usage_count: Option<i64>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct TagClass {
    /// Fully qualified names of tags associated with this tag. Associated tags captures
    /// relationship of one tag to another automatically. As an example a tag 'User.PhoneNumber'
    /// might have an associated tag 'PII.Sensitive'. When 'User.Address' is used to label a
    /// column in a table, 'PII.Sensitive' label is also applied automatically due to Associated
    /// tag relationship.
    #[serde(rename = "associatedTags")]
    associated_tags: Option<Vec<String>>,
    /// Tags under this tag group or empty for tags at the leaf level.
    children: Option<Vec<Option<Tag>>>,
    /// If the tag is deprecated.
    deprecated: Option<bool>,
    /// Unique name of the tag category.
    description: String,
    /// Unique name of the tag of format Category.PrimaryTag.SecondaryTag.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to the resource corresponding to the tag.
    href: Option<String>,
    /// Name of the tag.
    name: String,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Count of how many times this tag and children tags are used.
    #[serde(rename = "usageCount")]
    usage_count: Option<i64>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the Thread entity. A Thread is a collection of posts made by the
/// users. The first post that starts a thread is **about** a data asset **from** a user.
/// Other users can respond to this post by creating new posts in the thread. Note that bot
/// users can also interact with a thread. A post can contains links that mention Users or
/// other Data Assets.
#[derive(Serialize, Deserialize)]
pub struct Thread {
    /// Data asset about which this thread is created for with format
    /// <#E/{entities}/{entityName}/{field}/{fieldValue}.
    about: String,
    /// User or team this thread is addressed to in format
    /// <#E/{entities}/{entityName}/{field}/{fieldValue}.
    #[serde(rename = "addressedTo")]
    addressed_to: Option<String>,
    /// Link to the resource corresponding to this entity.
    href: Option<String>,
    /// Unique identifier that identifies an entity instance.
    id: String,
    posts: Vec<Post>,
    /// Timestamp of the when the first post created the thread.
    #[serde(rename = "threadTs")]
    thread_ts: Option<String>,
}

/// Post within a feed.
#[derive(Serialize, Deserialize)]
pub struct Post {
    /// ID of User (regular user or a bot) posting the message.
    from: String,
    /// Message in markdown format. See markdown support for more details.
    message: String,
    /// Timestamp of the post.
    #[serde(rename = "postTs")]
    post_ts: Option<String>,
}

/// Describes an entity Lifecycle Rule used within a Policy.
#[derive(Serialize, Deserialize)]
pub struct LifecycleRule {
    /// A set of actions to take on the entities.
    actions: Vec<ActionElement>,
    /// Is the rule enabled.
    enabled: Option<bool>,
    filters: Vec<Filter>,
    /// Name that identifies this Rule.
    name: Option<String>,
}

/// An action to delete or expire the entity.
///
/// An action to move the entity to a different location. For eg: Move from Standard storage
/// tier to Archive storage tier.
#[derive(Serialize, Deserialize)]
pub struct ActionElement {
    /// Number of days after creation of the entity that the deletion should be triggered.
    ///
    /// Number of days after creation of the entity that the move should be triggered.
    #[serde(rename = "daysAfterCreation")]
    days_after_creation: Option<i64>,
    /// Number of days after last modification of the entity that the deletion should be
    /// triggered.
    ///
    /// Number of days after last modification of the entity that the move should be triggered.
    #[serde(rename = "daysAfterModification")]
    days_after_modification: Option<i64>,
    /// Location where this entity needs to be moved to.
    destination: Option<ActionDestination>,
}

/// Location where this entity needs to be moved to.
#[derive(Serialize, Deserialize)]
pub struct ActionDestination {
    /// The location where to move this entity to.
    location: Option<Location>,
    /// The storage class to move this entity to.
    #[serde(rename = "storageClassType")]
    storage_class_type: Option<StorageClassType>,
    /// The storage service to move this entity to.
    #[serde(rename = "storageServiceType")]
    storage_service_type: Option<StorageService>,
}

/// This schema defines the Location entity. A Location can contain the data of a table or
/// group other sublocation together.
///
/// The location where to move this entity to.
#[derive(Serialize, Deserialize)]
pub struct Location {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Description of a location.
    description: Option<String>,
    /// Display Name that identifies this table. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Followers of this location.
    followers: Option<Vec<ServiceElement>>,
    /// Fully qualified name of a location in the form `serviceName.locationName`.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to this location resource.
    href: Option<String>,
    /// Unique identifier of this location instance.
    id: Option<String>,
    #[serde(rename = "locationType")]
    location_type: Option<LocationType>,
    /// Name of a location without the service. For example s3://bucket/path1/path2.
    name: String,
    /// Owner of this location.
    owner: Option<ServiceElement>,
    /// Link to the database cluster/service where this database is hosted in.
    service: ServiceElement,
    /// Service type where this storage location is hosted in.
    #[serde(rename = "serviceType")]
    service_type: Option<StorageServiceType>,
    /// Tags for this location.
    tags: Option<Vec<TagElement>>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// This schema defines the Storage Service entity, such as S3, GCS, HDFS.
///
/// The storage service to move this entity to.
#[derive(Serialize, Deserialize)]
pub struct StorageService {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Description of a storage service instance.
    description: Option<String>,
    /// Display Name that identifies this storage service.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Link to the resource corresponding to this storage service.
    href: String,
    /// Unique identifier of this storage service instance.
    id: String,
    /// Name that identifies this storage service.
    name: String,
    /// Type of storage service such as S3, GCS, HDFS...
    #[serde(rename = "serviceType")]
    service_type: StorageServiceType,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Metadata version of the entity.
    version: Option<f64>,
}

/// Entity tags to match on.
///
/// This schema defines the type for labeling an entity with a Tag.
#[derive(Serialize, Deserialize)]
pub struct TagLabel {
    /// Unique name of the tag category.
    description: Option<String>,
    /// Link to the tag resource.
    href: Option<String>,
    /// Label type describes how a tag label was applied. 'Manual' indicates the tag label was
    /// applied by a person. 'Derived' indicates a tag label was derived using the associated tag
    /// relationship (see TagCategory.json for more details). 'Propagated` indicates a tag label
    /// was propagated from upstream based on lineage. 'Automated' is used when a tool was used
    /// to determine the tag label.
    #[serde(rename = "labelType")]
    label_type: LabelType,
    /// 'Suggested' state is used when a tag label is suggested by users or tools. Owner of the
    /// entity must confirm the suggested labels before it is marked as 'Confirmed'.
    state: State,
    #[serde(rename = "tagFQN")]
    tag_fqn: String,
}

/// An action to move the entity to a different location. For eg: Move from Standard storage
/// tier to Archive storage tier.
#[derive(Serialize, Deserialize)]
pub struct LifecycleMoveAction {
    /// Number of days after creation of the entity that the move should be triggered.
    #[serde(rename = "daysAfterCreation")]
    days_after_creation: Option<i64>,
    /// Number of days after last modification of the entity that the move should be triggered.
    #[serde(rename = "daysAfterModification")]
    days_after_modification: Option<i64>,
    /// Location where this entity needs to be moved to.
    destination: Option<LifecycleMoveActionDestination>,
}

/// Location where this entity needs to be moved to.
#[derive(Serialize, Deserialize)]
pub struct LifecycleMoveActionDestination {
    /// The location where to move this entity to.
    location: Option<Location>,
    /// The storage class to move this entity to.
    #[serde(rename = "storageClassType")]
    storage_class_type: Option<StorageClassType>,
    /// The storage service to move this entity to.
    #[serde(rename = "storageServiceType")]
    storage_service_type: Option<StorageService>,
}

/// An action to delete or expire the entity.
#[derive(Serialize, Deserialize)]
pub struct LifecycleDeleteAction {
    /// Number of days after creation of the entity that the deletion should be triggered.
    #[serde(rename = "daysAfterCreation")]
    days_after_creation: Option<i64>,
    /// Number of days after last modification of the entity that the deletion should be
    /// triggered.
    #[serde(rename = "daysAfterModification")]
    days_after_modification: Option<i64>,
}

/// Describes an entity Access Control Rule used within a Policy.
#[derive(Serialize, Deserialize)]
pub struct AccessControlRule {
    /// A set of access control enforcements to take on the entities.
    actions: Vec<AccessControlRuleAction>,
    /// Is the rule enabled.
    enabled: Option<bool>,
    filters: Vec<Filter>,
    /// Name that identifies this Rule.
    name: Option<String>,
}

/// Describes an Access Control Rule to selectively grant access to Teams/Users to tagged
/// entities.
#[derive(Serialize, Deserialize)]
pub struct AccessControlRuleAction {
    /// Teams and Users who are able to access the tagged entities.
    allow: Vec<AllowElement>,
    /// Tags that are associated with the entities.
    tags: Vec<TagLabel>,
}

/// This schema defines the Team entity. A Team is a group of zero or more users. Teams can
/// own zero or more data assets.
///
/// This schema defines the User entity. A user can be part of 0 or more teams. A special
/// type of user called Bot is used for automation. A user can be an owner of zero or more
/// data assets. A user can also follow zero or more data assets.
#[derive(Serialize, Deserialize)]
pub struct AllowElement {
    /// Change that lead to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// When true the team has been deleted.
    deleted: Option<bool>,
    /// Description of the team.
    ///
    /// Used for user biography.
    description: Option<String>,
    /// Name used for display purposes. Example 'Data Science team'.
    ///
    /// Name used for display purposes. Example 'FirstName LastName'.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Link to the resource corresponding to this entity.
    href: String,
    /// Unique identifier that identifies a user entity instance.
    id: String,
    name: String,
    /// List of entities owned by the team.
    ///
    /// List of entities owned by the user.
    owns: Option<Vec<ServiceElement>>,
    /// Team profile information.
    ///
    /// Profile of the user.
    profile: Option<ProfileClass>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Users that are part of the team.
    users: Option<Vec<ServiceElement>>,
    /// Metadata version of the entity.
    version: Option<f64>,
    /// When true indicates the user has been deactivated. Users are deactivated instead of
    /// deleted.
    deactivated: Option<bool>,
    /// Email address of the user.
    email: Option<String>,
    /// List of entities followed by the user.
    follows: Option<Vec<ServiceElement>>,
    /// When true indicates user is an administrator for the system with superuser privileges.
    #[serde(rename = "isAdmin")]
    is_admin: Option<bool>,
    /// When true indicates a special type of user called Bot.
    #[serde(rename = "isBot")]
    is_bot: Option<bool>,
    /// Teams that the user belongs to.
    teams: Option<Vec<ServiceElement>>,
    /// Timezone of the user.
    timezone: Option<String>,
}

/// Describes an Access Control Rule to selectively grant access to Teams/Users to tagged
/// entities.
#[derive(Serialize, Deserialize)]
pub struct TagBased {
    /// Teams and Users who are able to access the tagged entities.
    allow: Vec<AllowElement>,
    /// Tags that are associated with the entities.
    tags: Vec<TagLabel>,
}

/// This schema defines the Policy entity. A Policy defines lifecycle or access control that
/// needs to be applied across different Data Entities.
#[derive(Serialize, Deserialize)]
pub struct Policy {
    /// Change that led to this version of the Policy.
    #[serde(rename = "changeDescription")]
    change_description: Option<PolicyChangeDescription>,
    /// A short description of the Policy, comprehensible to regular users.
    description: Option<String>,
    /// Title for this Policy.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Is the policy enabled.
    enabled: Option<bool>,
    /// Name that uniquely identifies a Policy.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to the resource corresponding to this entity.
    href: Option<String>,
    /// Unique identifier that identifies this Policy.
    id: String,
    /// Name that identifies this Policy.
    name: String,
    /// Owner of this Policy.
    owner: Option<EntityReference>,
    #[serde(rename = "policyType")]
    policy_type: PolicyType,
    /// Link to a well documented definition of this Policy.
    #[serde(rename = "policyUrl")]
    policy_url: Option<String>,
    rules: Option<Vec<PolicyRule>>,
    /// Last update time corresponding to the new version of the Policy.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Metadata version of the Policy.
    version: Option<f64>,
}

/// Change that led to this version of the Policy.
///
/// Description of the change.
#[derive(Serialize, Deserialize)]
pub struct PolicyChangeDescription {
    /// Names of fields added during the version changes.
    #[serde(rename = "fieldsAdded")]
    fields_added: Option<Vec<FluffyFieldChange>>,
    /// Fields deleted during the version changes with old value before deleted.
    #[serde(rename = "fieldsDeleted")]
    fields_deleted: Option<Vec<FluffyFieldChange>>,
    /// Fields modified during the version changes with old and new values.
    #[serde(rename = "fieldsUpdated")]
    fields_updated: Option<Vec<FluffyFieldChange>>,
    /// When a change did not result in change, this could be same as the current version.
    #[serde(rename = "previousVersion")]
    previous_version: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyFieldChange {
    /// Name of the entity field that changed.
    name: Option<String>,
    /// New value of the field. Note that this is a JSON string and use the corresponding field
    /// type to deserialize it.
    #[serde(rename = "newValue")]
    new_value: Option<serde_json::Value>,
    /// Previous value of the field. Note that this is a JSON string and use the corresponding
    /// field type to deserialize it.
    #[serde(rename = "oldValue")]
    old_value: Option<serde_json::Value>,
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
#[derive(Serialize, Deserialize)]
pub struct EntityReference {
    /// Optional description of entity.
    description: Option<String>,
    /// Display Name that identifies this entity.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Link to the entity resource.
    href: Option<String>,
    /// Unique identifier that identifies an entity instance.
    id: String,
    /// Name of the entity instance. For entities such as tables, databases where the name is not
    /// unique, fullyQualifiedName is returned in this field.
    name: Option<String>,
    /// Entity type/class name - Examples: `database`, `table`, `metrics`, `databaseService`,
    /// `dashboardService`...
    #[serde(rename = "type")]
    entity_reference_type: String,
}

/// A set of rules associated with the Policy.
///
/// Describes an entity Access Control Rule used within a Policy.
///
/// Describes an entity Lifecycle Rule used within a Policy.
#[derive(Serialize, Deserialize)]
pub struct PolicyRule {
    /// A set of access control enforcements to take on the entities.
    ///
    /// A set of actions to take on the entities.
    actions: Vec<RuleAction>,
    /// Is the rule enabled.
    enabled: Option<bool>,
    filters: Vec<Filter>,
    /// Name that identifies this Rule.
    name: Option<String>,
}

/// Describes an Access Control Rule to selectively grant access to Teams/Users to tagged
/// entities.
///
/// An action to delete or expire the entity.
///
/// An action to move the entity to a different location. For eg: Move from Standard storage
/// tier to Archive storage tier.
#[derive(Serialize, Deserialize)]
pub struct RuleAction {
    /// Teams and Users who are able to access the tagged entities.
    allow: Option<Vec<AllowElement>>,
    /// Tags that are associated with the entities.
    tags: Option<Vec<TagLabel>>,
    /// Number of days after creation of the entity that the deletion should be triggered.
    ///
    /// Number of days after creation of the entity that the move should be triggered.
    #[serde(rename = "daysAfterCreation")]
    days_after_creation: Option<i64>,
    /// Number of days after last modification of the entity that the deletion should be
    /// triggered.
    ///
    /// Number of days after last modification of the entity that the move should be triggered.
    #[serde(rename = "daysAfterModification")]
    days_after_modification: Option<i64>,
    /// Location where this entity needs to be moved to.
    destination: Option<ActionDestination>,
}

/// This schema defines the type used for lineage of an entity.
#[derive(Serialize, Deserialize)]
pub struct EntityLineage {
    #[serde(rename = "downstreamEdges")]
    downstream_edges: Option<Vec<Edge>>,
    /// Primary entity for which this lineage graph is created.
    entity: ServiceElement,
    nodes: Option<Vec<ServiceElement>>,
    #[serde(rename = "upstreamEdges")]
    upstream_edges: Option<Vec<Edge>>,
}

/// Edge in the lineage graph from one entity to another by entity IDs.
#[derive(Serialize, Deserialize)]
pub struct Edge {
    description: Option<String>,
    /// From entity that is upstream of lineage edge.
    #[serde(rename = "fromEntity")]
    from_entity: Option<String>,
    /// To entity that is downstream of lineage edge.
    #[serde(rename = "toEntity")]
    to_entity: Option<String>,
}

/// This schema defines the type for reporting the daily count of some measurement. For
/// example, you might use this schema for the number of times a table is queried each day.
#[derive(Serialize, Deserialize)]
pub struct DailyCountOfSomeMeasurement {
    /// Daily count of a measurement on the given date.
    count: i64,
    date: String,
}

/// This schema defines the type used for capturing usage details of an entity.
#[derive(Serialize, Deserialize)]
pub struct UsageDetailsOfAnEntity {
    /// Entity for which usage is returned.
    entity: ServiceElement,
    /// List usage details per day.
    usage: Vec<UsageSummaryElement>,
}

/// Type used for capturing the details of a collection.
#[derive(Serialize, Deserialize)]
pub struct SchemaForCollectionDescriptor {
    collection: Option<CollectionInfo>,
}

/// Collection Info.
#[derive(Serialize, Deserialize)]
pub struct CollectionInfo {
    /// Description of collection.
    documentation: Option<String>,
    /// URL of the API endpoint where given collections are available.
    href: Option<String>,
    images: Option<ImageList>,
    /// Unique name that identifies a collection.
    name: Option<String>,
}

/// This schema defines the type for usage details. Daily, weekly, and monthly aggregation of
/// usage is computed along with the percentile rank based on the usage for a given day.
#[derive(Serialize, Deserialize)]
pub struct TypeUsedToReturnUsageDetailsOfAnEntity {
    /// Daily usage stats of a data asset on the start date.
    #[serde(rename = "dailyStats")]
    daily_stats: UsageStats,
    /// Date in UTC.
    date: String,
    /// Monthly (last 30 days) rolling usage stats of a data asset on the start date.
    #[serde(rename = "monthlyStats")]
    monthly_stats: Option<UsageStats>,
    /// Weekly (last 7 days) rolling usage stats of a data asset on the start date.
    #[serde(rename = "weeklyStats")]
    weekly_stats: Option<UsageStats>,
}

/// This schema defines the type used for JDBC connection information.
#[derive(Serialize, Deserialize)]
pub struct JdbcConnection {
    /// JDBC connection URL.
    #[serde(rename = "connectionUrl")]
    connection_url: String,
    /// JDBC driver class.
    #[serde(rename = "driverClass")]
    driver_class: String,
    /// Login password.
    password: String,
    /// Login user name.
    #[serde(rename = "userName")]
    user_name: String,
}

/// This schema defines the type used for capturing version of history of entity.
#[derive(Serialize, Deserialize)]
pub struct EntityVersionHistory {
    /// Entity type, such as `database`, `table`, `dashboard`, for which this version history is
    /// produced.
    #[serde(rename = "entityType")]
    entity_type: String,
    versions: Vec<Option<serde_json::Value>>,
}

/// This schema defines the change event type to capture the changes to entities. Entities
/// change due to user activity, such as updating description of a dataset, changing
/// ownership, or adding new tags. Entity also changes due to activities at the metadata
/// sources, such as a new dataset was created, a datasets was deleted, or schema of a
/// dataset is modified. When state of entity changes, an event is produced. These events can
/// be used to build apps and bots that respond to the change from activities.
#[derive(Serialize, Deserialize)]
pub struct ChangeEvent {
    /// For `eventType` `entityUpdated` this field captures details about what fields were
    /// added/updated/deleted. For `eventType` `entityCreated` or `entityDeleted` this field is
    /// null.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Current version of the entity after this change. Note that not all changes result in
    /// entity version change. When entity version is not changed, `previousVersion` is same as
    /// `currentVersion`.
    #[serde(rename = "currentVersion")]
    current_version: Option<f64>,
    /// Date and time when the change was made.
    #[serde(rename = "dateTime")]
    date_time: String,
    /// For `eventType` `entityCreated`, this field captures JSON coded string of the entity
    /// using the schema corresponding to `entityType`.
    entity: Option<serde_json::Value>,
    /// Identifier of entity that was modified by the operation.
    #[serde(rename = "entityId")]
    entity_id: String,
    /// Entity type that changed. Use the schema of this entity to process the entity attribute.
    #[serde(rename = "entityType")]
    entity_type: String,
    #[serde(rename = "eventType")]
    event_type: EventType,
    /// Version of the entity before this change. Note that not all changes result in entity
    /// version change. When entity version is not changed, `previousVersion` is same as
    /// `currentVersion`.
    #[serde(rename = "previousVersion")]
    previous_version: Option<f64>,
    /// Name of the user whose activity resulted in the change.
    #[serde(rename = "userName")]
    user_name: Option<String>,
}

/// Type used for cursor based pagination information in GET list responses.
#[derive(Serialize, Deserialize)]
pub struct Paging {
    /// After cursor used for getting the next page (see API pagination for details).
    after: Option<String>,
    /// Before cursor used for getting the previous page (see API pagination for details).
    before: Option<String>,
    /// Total number of entries available to page through.
    total: i64,
}

/// This schema defines the Audit Log type to capture the audit trail of POST, PUT, and PATCH
/// API operations.
#[derive(Serialize, Deserialize)]
pub struct AuditLog {
    /// Date when the API call is made.
    #[serde(rename = "dateTime")]
    date_time: Option<String>,
    /// Identifier of entity that was modified by the operation.
    #[serde(rename = "entityId")]
    entity_id: String,
    /// Type of Entity that is modified by the operation.
    #[serde(rename = "entityType")]
    entity_type: String,
    /// HTTP Method used in a call.
    method: Method,
    /// Requested API Path.
    path: String,
    /// HTTP response code for the api requested.
    #[serde(rename = "responseCode")]
    response_code: i64,
    /// Name of the user who made the API request.
    #[serde(rename = "userName")]
    user_name: String,
}

/// This schema defines the type used for the schedule. The schedule has a start time and
/// repeat frequency.
#[derive(Serialize, Deserialize)]
pub struct Schedule {
    /// Repeat frequency in ISO 8601 duration format. Example - 'P23DT23H'.
    #[serde(rename = "repeatFrequency")]
    repeat_frequency: Option<String>,
    /// Start date and time of the schedule.
    #[serde(rename = "startDate")]
    start_date: Option<String>,
}

/// Ingestion Config is used to setup a Airflow Ingestion pipeline.
#[derive(Serialize, Deserialize)]
pub struct Ingestion {
    /// Change that led to this version of the entity.
    #[serde(rename = "changeDescription")]
    change_description: Option<PipelineChangeDescription>,
    /// Concurrency of the Pipeline.
    concurrency: Option<i64>,
    #[serde(rename = "connectorConfig")]
    connector_config: ConnectorConfig,
    /// Description of the workflow.
    description: Option<String>,
    /// Display Name that identifies this Ingestion.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// End Date of the workflow.
    #[serde(rename = "endDate")]
    end_date: Option<String>,
    /// Deploy the workflow by overwriting existing workflow with the same name.
    #[serde(rename = "forceDeploy")]
    force_deploy: Option<bool>,
    /// Name that uniquely identifies a Ingestion.
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Link to this ingestion resource.
    href: Option<String>,
    /// Unique identifier that identifies this Ingestion.
    id: Option<String>,
    /// List of executions and status for the Ingestion Pipeline.
    #[serde(rename = "ingestionStatuses")]
    ingestion_statuses: Option<Vec<IngestionStatus>>,
    #[serde(rename = "ingestionType")]
    ingestion_type: Option<IngestionType>,
    /// Name that identifies this ingestion instance uniquely.
    name: String,
    /// Next execution date from the underlying workflow platform once the ingestion scheduled.
    #[serde(rename = "nextExecutionDate")]
    next_execution_date: Option<String>,
    /// Owner of this Ingestion.
    owner: Option<ServiceElement>,
    /// pause the workflow from running once the deploy is finished successfully.
    #[serde(rename = "pauseWorkflow")]
    pause_workflow: Option<bool>,
    /// Retry workflow in case of failure.
    retries: Option<i64>,
    /// Delay between retries in seconds.
    #[serde(rename = "retryDelay")]
    retry_delay: Option<i64>,
    /// Scheduler Interval for the Workflow in cron format.
    #[serde(rename = "scheduleInterval")]
    schedule_interval: Option<String>,
    /// Link to the database service where this database is hosted in.
    service: ServiceElement,
    /// Start date of the workflow.
    #[serde(rename = "startDate")]
    start_date: String,
    /// Tags associated with the Ingestion.
    tags: Option<Vec<TagElement>>,
    /// Last update time corresponding to the new version of the entity.
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy")]
    updated_by: Option<String>,
    /// Metadata version of the entity.
    version: Option<f64>,
    /// Run past executions if the start date is in the past.
    #[serde(rename = "workflowCatchup")]
    workflow_catchup: Option<bool>,
    /// Timeout for the workflow in seconds.
    #[serde(rename = "workflowTimeout")]
    workflow_timeout: Option<i64>,
    /// Timezone in which workflow going to be scheduled.
    #[serde(rename = "workflowTimezone")]
    workflow_timezone: Option<String>,
}

/// This defines the configuration for connector.
#[derive(Serialize, Deserialize)]
pub struct ConnectorConfig {
    /// Database of the data source.
    database: Option<String>,
    /// Run data profiler as part of ingestion to get table profile data.
    #[serde(rename = "enableDataProfiler")]
    enable_data_profiler: Option<bool>,
    /// Regex exclude tables or databases that matches the pattern.
    #[serde(rename = "excludeFilterPattern")]
    exclude_filter_pattern: Option<Vec<String>>,
    /// Host and port of the data source.
    host: Option<String>,
    /// Regex to only fetch tables or databases that matches the pattern.
    #[serde(rename = "includeFilterPattern")]
    include_filter_pattern: Option<Vec<String>>,
    /// optional configuration to turn off fetching metadata for views.
    #[serde(rename = "includeViews")]
    include_views: Option<bool>,
    /// password to connect  to the data source.
    password: Option<String>,
    /// username to connect  to the data source.
    username: Option<String>,
}

/// This defines the runtime status of Ingestion.
#[derive(Serialize, Deserialize)]
pub struct IngestionStatus {
    /// endDate of the Ingestion pipeline run for this particular execution.
    #[serde(rename = "endDate")]
    end_date: Option<String>,
    /// startDate of the Ingestion Pipeline run for this particular execution.
    #[serde(rename = "startDate")]
    start_date: Option<String>,
    /// Workflow status denotes if its failed or succeeded.
    state: Option<String>,
}

/// Create Chart entity request
#[derive(Serialize, Deserialize)]
pub struct CreateChartEntityRequest {
    #[serde(rename = "chartType")]
    chart_type: Option<ChartType>,
    /// Chart URL, pointing to its own Service URL
    #[serde(rename = "chartUrl")]
    chart_url: Option<String>,
    /// Description of the chart instance. What it has and how to use it.
    description: Option<String>,
    /// Display Name that identifies this Chart. It could be title or label from the source
    /// services
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Name that identifies this Chart.
    name: String,
    /// Owner of this database
    owner: Option<ServiceElement>,
    /// Link to the database service where this database is hosted in
    service: ServiceElement,
    /// Link to tables used in this chart.
    tables: Option<Vec<ServiceElement>>,
    /// Tags for this chart
    tags: Option<Vec<TagElement>>,
}

/// Create Dashboard entity request
#[derive(Serialize, Deserialize)]
pub struct CreateDashboardEntityRequest {
    /// All the charts included in this Dashboard.
    charts: Option<Vec<ServiceElement>>,
    /// Dashboard URL
    #[serde(rename = "dashboardUrl")]
    dashboard_url: Option<String>,
    /// Description of the database instance. What it has and how to use it.
    description: Option<String>,
    /// Display Name that identifies this Dashboard. It could be title or label from the source
    /// services
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Name that identifies this dashboard.
    name: String,
    /// Owner of this database
    owner: Option<ServiceElement>,
    /// Link to the database service where this database is hosted in
    service: ServiceElement,
    /// Tags for this chart
    tags: Option<Vec<TagElement>>,
}

/// Create Database entity request
#[derive(Serialize, Deserialize)]
pub struct CreateDatabaseEntityRequest {
    /// Description of the database instance. What it has and how to use it.
    description: Option<String>,
    /// Name that identifies this database instance uniquely.
    name: String,
    /// Owner of this database
    owner: Option<EntityReference>,
    /// Link to the database service where this database is hosted in
    service: EntityReference,
}

/// Create Ml Model entity request
#[derive(Serialize, Deserialize)]
pub struct CreateMlModelEntityRequest {
    /// Algorithm used to train the ML Model
    algorithm: String,
    /// Performance Dashboard URL to track metric evolution
    dashboard: Option<ServiceElement>,
    /// Description of the ML model instance. How it was trained and for what it is used.
    description: Option<String>,
    /// Display Name that identifies this ML model. It could be title or label from the source
    /// services
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Features used to train the ML Model.
    #[serde(rename = "mlFeatures")]
    ml_features: Option<Vec<MlFeature>>,
    /// Hyper Parameters used to train the ML Model.
    #[serde(rename = "mlHyperParameters")]
    ml_hyper_parameters: Option<Vec<MlHyperParameter>>,
    /// Location containing the ML Model. It can be a storage layer and/or a container repository.
    #[serde(rename = "mlStore")]
    ml_store: Option<MlStore>,
    /// Name that identifies this ML model.
    name: String,
    /// Owner of this database
    owner: Option<ServiceElement>,
    /// Endpoint that makes the ML Model available, e.g,. a REST API serving the data or
    /// computing predictions.
    server: Option<String>,
    /// Tags for this ML Model
    tags: Option<Vec<TagElement>>,
}

/// Schema corresponding to a table that belongs to a database
#[derive(Serialize, Deserialize)]
pub struct CreateTableEntityRequest {
    /// Name of the tables in the database
    columns: Vec<CreateTableEntityRequestColumn>,
    /// Database corresponding to this table
    database: String,
    /// Description of entity instance.
    description: Option<String>,
    /// Name that identifies the this entity instance uniquely. Same as id if when name is not
    /// unique
    name: String,
    /// Owner of this entity
    owner: Option<EntityReference>,
    #[serde(rename = "tableConstraints")]
    table_constraints: Option<Vec<CreateTableEntityRequestTableConstraint>>,
    #[serde(rename = "tableType")]
    table_type: Option<TableType>,
    /// Tags for this table
    tags: Option<Vec<TagLabel>>,
    /// View Definition in SQL. Applies to TableType.View only
    #[serde(rename = "viewDefinition")]
    view_definition: Option<String>,
}

/// This schema defines the type for a column in a table.
#[derive(Serialize, Deserialize)]
pub struct CreateTableEntityRequestColumn {
    /// Data type used array in dataType. For example, `array<int>` has dataType as `array` and
    /// arrayDataType as `int`.
    #[serde(rename = "arrayDataType")]
    array_data_type: Option<DataType>,
    /// Child columns if dataType or arrayDataType is `map`, `struct`, or `union` else `null`.
    children: Option<Vec<CreateTableEntityRequestColumn>>,
    /// Column level constraint.
    constraint: Option<Constraint>,
    /// Length of `char`, `varchar`, `binary`, `varbinary` `dataTypes`, else null. For example,
    /// `varchar(20)` has dataType as `varchar` and dataLength as `20`.
    #[serde(rename = "dataLength")]
    data_length: Option<i64>,
    /// Data type of the column (int, date etc.).
    #[serde(rename = "dataType")]
    data_type: DataType,
    /// Display name used for dataType. This is useful for complex types, such as `array<int>,
    /// map<int,string>, struct<>, and union types.
    #[serde(rename = "dataTypeDisplay")]
    data_type_display: Option<String>,
    /// Description of the column.
    description: Option<String>,
    #[serde(rename = "fullyQualifiedName")]
    fully_qualified_name: Option<String>,
    /// Json schema only if the dataType is JSON else null.
    #[serde(rename = "jsonSchema")]
    json_schema: Option<String>,
    name: String,
    /// Ordinal position of the column.
    #[serde(rename = "ordinalPosition")]
    ordinal_position: Option<i64>,
    /// Tags associated with the column.
    tags: Option<Vec<TagLabel>>,
}

/// This enum defines the type for table constraint.
#[derive(Serialize, Deserialize)]
pub struct CreateTableEntityRequestTableConstraint {
    /// List of column names corresponding to the constraint.
    columns: Option<Vec<String>>,
    #[serde(rename = "constraintType")]
    constraint_type: Option<ConstraintType>,
}

/// Create Pipeline entity request
#[derive(Serialize, Deserialize)]
pub struct CreatePipelineEntityRequest {
    /// Concurrency of the Pipeline
    concurrency: Option<i64>,
    /// Description of the database instance. What it has and how to use it.
    description: Option<String>,
    /// Display Name that identifies this Pipeline. It could be title or label from the source
    /// services.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Name that identifies this pipeline instance uniquely.
    name: String,
    /// Owner of this database
    owner: Option<ServiceElement>,
    /// Pipeline Code Location
    #[serde(rename = "pipelineLocation")]
    pipeline_location: Option<String>,
    /// Pipeline  URL to visit/manage. This URL points to respective pipeline service UI
    #[serde(rename = "pipelineUrl")]
    pipeline_url: Option<String>,
    /// Link to the database service where this database is hosted in
    service: ServiceElement,
    /// Start date of the workflow
    #[serde(rename = "startDate")]
    start_date: Option<String>,
    /// Tags for this Pipeline.
    tags: Option<Vec<TagElement>>,
    /// All the tasks that are part of pipeline.
    tasks: Option<Vec<Task>>,
}

/// Create a topic entity request
#[derive(Serialize, Deserialize)]
pub struct CreateTopicEntityRequest {
    /// Topic clean up policy. For Kafka - `cleanup.policy` configuration.
    #[serde(rename = "cleanupPolicies")]
    cleanup_policies: Option<Vec<CleanupPolicy>>,
    /// Description of the topic instance. What it has and how to use it.
    description: Option<String>,
    /// Maximum message size in bytes. For Kafka - `max.message.bytes` configuration.
    #[serde(rename = "maximumMessageSize")]
    maximum_message_size: Option<i64>,
    /// Minimum number replicas in sync to control durability. For Kafka - `min.insync.replicas`
    /// configuration.
    #[serde(rename = "minimumInSyncReplicas")]
    minimum_in_sync_replicas: Option<i64>,
    /// Name that identifies this topic instance uniquely.
    name: String,
    /// Owner of this topic
    owner: Option<EntityReference>,
    /// Number of partitions into which the topic is divided.
    partitions: i64,
    /// Replication Factor in integer (more than 1).
    #[serde(rename = "replicationFactor")]
    replication_factor: Option<i64>,
    /// Maximum size of a partition in bytes before old data is discarded. For Kafka -
    /// `retention.bytes` configuration.
    #[serde(rename = "retentionSize")]
    retention_size: Option<f64>,
    /// Retention time in milliseconds. For Kafka - `retention.ms` configuration.
    #[serde(rename = "retentionTime")]
    retention_time: Option<f64>,
    /// Schema used for message serialization. Optional as some topics may not have associated
    /// schemas.
    #[serde(rename = "schemaText")]
    schema_text: Option<String>,
    /// Schema used for message serialization.
    #[serde(rename = "schemaType")]
    schema_type: Option<SchemaType>,
    /// Link to the messaging service where this topic is hosted in
    service: EntityReference,
    /// Tags for this topic
    tags: Option<Vec<TagLabel>>,
}

/// Create Location entity request
#[derive(Serialize, Deserialize)]
pub struct CreateLocationEntityRequest {
    /// Description of the location instance.
    description: Option<String>,
    #[serde(rename = "locationType")]
    location_type: Option<LocationType>,
    /// Name that identifies this Location.
    name: String,
    /// Owner of this Location
    owner: Option<ServiceElement>,
    /// Link to the pipeline service where this location is used
    service: ServiceElement,
    /// Tags for this location
    tags: Option<Vec<TagElement>>,
}

/// Create Messaging service entity request
#[derive(Serialize, Deserialize)]
pub struct CreateMessagingServiceEntityRequest {
    /// Multiple bootstrap addresses for Kafka. Single proxy address for Pulsar.
    brokers: Vec<String>,
    /// Description of messaging service entity.
    description: Option<String>,
    /// Schedule for running metadata ingestion jobs
    #[serde(rename = "ingestionSchedule")]
    ingestion_schedule: Option<IngestionScheduleClass>,
    /// Name that identifies the this entity instance uniquely
    name: String,
    /// Schema registry URL
    #[serde(rename = "schemaRegistry")]
    schema_registry: Option<String>,
    #[serde(rename = "serviceType")]
    service_type: MessagingServiceType,
}

/// Create Storage service entity request
#[derive(Serialize, Deserialize)]
pub struct CreateStorageServiceEntityRequest {
    /// Description of Storage entity.
    description: Option<String>,
    /// Name that identifies the this entity instance uniquely
    name: String,
    #[serde(rename = "serviceType")]
    service_type: Option<StorageServiceType>,
}

/// Create Dashboard service entity request
#[derive(Serialize, Deserialize)]
pub struct CreateDashboardServiceEntityRequest {
    /// Dashboard Service URL
    #[serde(rename = "dashboardUrl")]
    dashboard_url: String,
    /// Description of dashboard service entity.
    description: Option<String>,
    /// Schedule for running metadata ingestion jobs
    #[serde(rename = "ingestionSchedule")]
    ingestion_schedule: Option<IngestionScheduleClass>,
    /// Name that identifies the this entity instance uniquely
    name: String,
    /// Password to log-into Dashboard Service
    password: Option<String>,
    #[serde(rename = "serviceType")]
    service_type: DashboardServiceType,
    /// Username to log-into Dashboard Service
    username: Option<String>,
}

/// Create Pipeline service entity request
#[derive(Serialize, Deserialize)]
pub struct CreatePipelineServiceEntityRequest {
    /// Description of pipeline service entity.
    description: Option<String>,
    /// Schedule for running pipeline ingestion jobs
    #[serde(rename = "ingestionSchedule")]
    ingestion_schedule: Option<IngestionScheduleClass>,
    /// Name that identifies the this entity instance uniquely
    name: String,
    /// Pipeline UI URL
    #[serde(rename = "pipelineUrl")]
    pipeline_url: String,
    #[serde(rename = "serviceType")]
    service_type: PipelineServiceType,
}

/// Create Database service entity request
#[derive(Serialize, Deserialize)]
pub struct CreateDatabaseServiceEntityRequest {
    /// Description of Database entity.
    description: Option<String>,
    /// Schedule for running metadata ingestion jobs
    #[serde(rename = "ingestionSchedule")]
    ingestion_schedule: Option<IngestionScheduleClass>,
    jdbc: JdbcInfo,
    /// Name that identifies the this entity instance uniquely
    name: String,
    #[serde(rename = "serviceType")]
    service_type: DatabaseServiceType,
}

/// Team entity
#[derive(Serialize, Deserialize)]
pub struct CreateTeamEntityRequest {
    /// Optional description of the team
    description: Option<String>,
    /// Optional name used for display purposes. Example 'Marketing Team'
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    name: String,
    /// Optional team profile information
    profile: Option<Profile>,
    /// Optional IDs of users that are part of the team
    users: Option<Vec<String>>,
}

/// This schema defines the type for a profile of a user, team, or organization.
///
/// Optional team profile information
#[derive(Serialize, Deserialize)]
pub struct Profile {
    images: Option<ImageList>,
}

/// Request to create User entity
#[derive(Serialize, Deserialize)]
pub struct CreateUserEntityRequest {
    /// Used for user biography.
    description: Option<String>,
    /// Name used for display purposes. Example 'FirstName LastName'
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    email: String,
    /// When true indicates user is an administrator for the system with superuser privileges
    #[serde(rename = "isAdmin")]
    is_admin: Option<bool>,
    /// When true indicates user is a bot with appropriate privileges
    #[serde(rename = "isBot")]
    is_bot: Option<bool>,
    name: String,
    profile: Option<Profile>,
    /// Teams that the user belongs to
    teams: Option<Vec<String>>,
    /// Timezone of the user
    timezone: Option<String>,
}

/// Create tag API request
#[derive(Serialize, Deserialize)]
pub struct CreateTagEntityRequest {
    /// Fully qualified names of tags associated with this tag
    #[serde(rename = "associatedTags")]
    associated_tags: Option<Vec<String>>,
    /// Unique name of the tag category
    description: String,
    name: String,
}

/// Create tag category request
#[derive(Serialize, Deserialize)]
pub struct CreateTagCategoryEntityRequest {
    #[serde(rename = "categoryType")]
    category_type: TagCategoryType,
    /// Description of the tag category
    description: String,
    name: String,
}

/// Ingestion Config is used to setup a Airflow Ingestion pipeline.
#[derive(Serialize, Deserialize)]
pub struct CreateIngestionEntityRequest {
    /// Concurrency of the Pipeline.
    concurrency: Option<i64>,
    #[serde(rename = "connectorConfig")]
    connector_config: ConnectorConfig,
    /// Description of the workflow.
    description: Option<String>,
    /// Display Name that identifies this Ingestion.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// End Date of the workflow.
    #[serde(rename = "endDate")]
    end_date: Option<String>,
    /// Deploy the workflow by overwriting existing workflow with the same name.
    #[serde(rename = "forceDeploy")]
    force_deploy: Option<bool>,
    #[serde(rename = "ingestionType")]
    ingestion_type: Option<IngestionType>,
    /// Name that identifies this ingestion instance uniquely.
    name: String,
    /// Owner of this Ingestion.
    owner: Option<ServiceElement>,
    /// pause the workflow from running once the deploy is finished successfully.
    #[serde(rename = "pauseWorkflow")]
    pause_workflow: Option<bool>,
    /// Retry workflow in case of failure
    retries: Option<i64>,
    /// Delay between retries in seconds.
    #[serde(rename = "retryDelay")]
    retry_delay: Option<i64>,
    /// Scheduler Interval for the Workflow in cron format.
    #[serde(rename = "scheduleInterval")]
    schedule_interval: Option<String>,
    /// Link to the database service where this database is hosted in.
    service: ServiceElement,
    /// Start date of the workflow.
    #[serde(rename = "startDate")]
    start_date: String,
    /// Tags associated with the Ingestion.
    tags: Option<Vec<TagElement>>,
    /// Workflow catchup for past executions.
    #[serde(rename = "workflowCatchup")]
    workflow_catchup: Option<bool>,
    /// Timeout for the workflow in seconds.
    #[serde(rename = "workflowTimeout")]
    workflow_timeout: Option<i64>,
    /// Timezone in which workflow going to be scheduled.
    #[serde(rename = "workflowTimezone")]
    workflow_timezone: Option<String>,
}

/// Catalog application software version
#[derive(Serialize, Deserialize)]
pub struct CatalogApplicationSoftwareVersion {
    /// Software revision of the catalog
    revision: Option<String>,
    /// Build timestamp
    timestamp: Option<String>,
    /// Software version of the catalog
    version: Option<String>,
}

/// Set ownership for a given entity
#[derive(Serialize, Deserialize)]
pub struct SetOwnershipForAGivenEntity {
    /// Id of the owner of the entity
    id: Option<String>,
    /// Entity type of the owner typically either 'user' or 'team'
    #[serde(rename = "type")]
    set_ownership_for_a_given_entity_type: Option<String>,
}

/// Create thread request
#[derive(Serialize, Deserialize)]
pub struct CreateThreadEntityRequest {
    /// Data asset about which this thread is created for with format
    /// <#E/{entities}/{entityName}/{field}/{fieldValue}
    about: String,
    /// ID of User (regular user or bot) posting the message
    from: String,
    /// Message
    message: String,
}

/// Add lineage details between two entities
#[derive(Serialize, Deserialize)]
pub struct AddLineage {
    /// User provided description of the lineage details.
    description: Option<String>,
    /// Lineage edge details.
    edge: EntitiesEdge,
}

/// Lineage edge details.
///
/// Edge in the lineage graph from one entity to another using entity references.
#[derive(Serialize, Deserialize)]
pub struct EntitiesEdge {
    description: Option<String>,
    /// From entity that is upstream of lineage edge.
    #[serde(rename = "fromEntity")]
    from_entity: Option<ServiceElement>,
    /// To entity that is downstream of lineage edge.
    #[serde(rename = "toEntity")]
    to_entity: Option<ServiceElement>,
}

/// Create Policy Entity Request
#[derive(Serialize, Deserialize)]
pub struct CreatePolicyEntityRequest {
    /// A short description of the Policy, comprehensible to regular users.
    description: Option<String>,
    /// Title for this Policy.
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// Name that identifies this Policy.
    name: String,
    /// Owner of this Policy.
    owner: Option<ServiceElement>,
    #[serde(rename = "policyType")]
    policy_type: PolicyType,
    /// Link to a well documented definition of this Policy.
    #[serde(rename = "policyUrl")]
    policy_url: Option<String>,
    rules: Option<Vec<CreatePolicyEntityRequestRule>>,
}

/// A set of rules associated with the Policy.
///
/// Describes an entity Access Control Rule used within a Policy.
///
/// Describes an entity Lifecycle Rule used within a Policy.
#[derive(Serialize, Deserialize)]
pub struct CreatePolicyEntityRequestRule {
    /// A set of access control enforcements to take on the entities.
    ///
    /// A set of actions to take on the entities.
    actions: Vec<RuleAction>,
    /// Is the rule enabled.
    enabled: Option<bool>,
    filters: Vec<Filter>,
    /// Name that identifies this Rule.
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub enum PipelineServiceType {
    Airflow,
    Glue,
    Prefect,
}

/// Label type describes how a tag label was applied. 'Manual' indicates the tag label was
/// applied by a person. 'Derived' indicates a tag label was derived using the associated tag
/// relationship (see TagCategory.json for more details). 'Propagated` indicates a tag label
/// was propagated from upstream based on lineage. 'Automated' is used when a tool was used
/// to determine the tag label.
#[derive(Serialize, Deserialize)]
pub enum LabelType {
    Automated,
    Derived,
    Manual,
    Propagated,
}

/// 'Suggested' state is used when a tag label is suggested by users or tools. Owner of the
/// entity must confirm the suggested labels before it is marked as 'Confirmed'.
#[derive(Serialize, Deserialize)]
pub enum State {
    Confirmed,
    Suggested,
}

/// Data type used array in dataType. For example, `array<int>` has dataType as `array` and
/// arrayDataType as `int`.
///
/// This enum defines the type of data stored in a column.
///
/// Data type of the column (int, date etc.).
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum ModelType {
    #[serde(rename = "DBT")]
    Dbt,
}

/// Service type this table is hosted in.
///
/// Type of database service such as MySQL, BigQuery, Snowflake, Redshift, Postgres...
///
/// Service type where this database is hosted in.
#[derive(Serialize, Deserialize)]
pub enum DatabaseServiceType {
    Athena,
    BigQuery,
    Druid,
    Glue,
    Hive,
    #[serde(rename = "MariaDB")]
    MariaDb,
    #[serde(rename = "MSSQL")]
    Mssql,
    #[serde(rename = "MySQL")]
    MySql,
    Oracle,
    Postgres,
    Presto,
    Redshift,
    Snowflake,
    Trino,
    Vertica,
}

#[derive(Serialize, Deserialize)]
pub enum ConstraintType {
    #[serde(rename = "FOREIGN_KEY")]
    ForeignKey,
    #[serde(rename = "PRIMARY_KEY")]
    PrimaryKey,
    #[serde(rename = "UNIQUE")]
    Unique,
}

/// This schema defines the type used for describing different types of tables.
#[derive(Serialize, Deserialize)]
pub enum TableType {
    External,
    MaterializedView,
    Regular,
    SecureView,
    View,
}

/// Data type of the column (numerical vs. categorical).
///
/// This enum defines the type of data stored in a ML Feature.
#[derive(Serialize, Deserialize)]
pub enum FeatureType {
    #[serde(rename = "categorical")]
    Categorical,
    #[serde(rename = "numerical")]
    Numerical,
}

/// Data type of the source (int, date etc.).
///
/// This enum defines the type of data of a ML Feature source.
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub enum DashboardServiceType {
    Looker,
    Metabase,
    Redash,
    Superset,
    Tableau,
}

/// This schema defines the type used for describing different types of charts.
#[derive(Serialize, Deserialize)]
pub enum ChartType {
    Area,
    Bar,
    BoxPlot,
    Histogram,
    Line,
    Other,
    Pie,
    Scatter,
    Table,
    Text,
}

/// Topic clean up policy. For Kafka - `cleanup.policy` configuration.
#[derive(Serialize, Deserialize)]
pub enum CleanupPolicy {
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "delete")]
    Delete,
}

/// Schema used for message serialization.
///
/// Schema type used for the message.
#[derive(Serialize, Deserialize)]
pub enum SchemaType {
    Avro,
    #[serde(rename = "JSON")]
    Json,
    Other,
    Protobuf,
}

/// Service type where this topic is hosted in.
///
/// Type of messaging service - Kafka or Pulsar.
///
/// Type of messaging service such as Kafka or Pulsar...
#[derive(Serialize, Deserialize)]
pub enum MessagingServiceType {
    Kafka,
    Pulsar,
}

/// Type of tag category.
#[derive(Serialize, Deserialize)]
pub enum TagCategoryType {
    Classification,
    Descriptive,
}

/// This schema defines the type used for describing different types of Location.
#[derive(Serialize, Deserialize)]
pub enum LocationType {
    Bucket,
    Database,
    Prefix,
    Table,
}

/// Service type where this storage location is hosted in.
///
/// Type of storage service such as S3, GCS, HDFS...
#[derive(Serialize, Deserialize)]
pub enum StorageServiceType {
    #[serde(rename = "ABFS")]
    Abfs,
    #[serde(rename = "GCS")]
    Gcs,
    #[serde(rename = "HDFS")]
    Hdfs,
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub enum PolicyType {
    AccessControl,
    Lifecycle,
}

/// Type of event.
#[derive(Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "entityCreated")]
    EntityCreated,
    #[serde(rename = "entityDeleted")]
    EntityDeleted,
    #[serde(rename = "entityUpdated")]
    EntityUpdated,
}

/// HTTP Method used in a call.
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
