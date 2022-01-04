use crate::error::*;
use crate::models::*;
use arrow_deps::arrow::datatypes::{DataType, Field, Schema, TimeUnit};

impl TryFrom<ResultTable> for Schema {
    type Error = KustoRsError;

    fn try_from(value: ResultTable) -> Result<Self> {
        let fields = value
            .columns
            .into_iter()
            .map(|f| Field::try_from(f).unwrap())
            .collect();
        Ok(Schema::new(fields))
    }
}

impl TryFrom<Column> for Field {
    type Error = KustoRsError;

    fn try_from(value: Column) -> Result<Self> {
        let data_type = DataType::try_from(value.column_type.clone())?;
        Ok(Field::new(value.column_name.as_str(), data_type, true))
    }
}

impl TryFrom<ColumnType> for DataType {
    type Error = KustoRsError;

    fn try_from(value: ColumnType) -> Result<Self> {
        // TODO not implemented right now: decimal
        match value {
            ColumnType::Bool => Ok(DataType::Boolean),
            ColumnType::Boolean => Ok(DataType::Boolean),
            ColumnType::Decimal => Err(KustoRsError::NotImplemented(
                "Conversion to Decimal type".to_string(),
            )),
            ColumnType::Dynamic => Ok(DataType::Utf8),
            ColumnType::Guid => Ok(DataType::Utf8),
            ColumnType::Int => Ok(DataType::Int32),
            ColumnType::Long => Ok(DataType::Int64),
            ColumnType::Real => Ok(DataType::Float64),
            ColumnType::String => Ok(DataType::Utf8),
            // We convert all timestamps to nanosecond accuracy, mainly since the arrow cast function behaves as such
            ColumnType::Date => Ok(DataType::Timestamp(TimeUnit::Nanosecond, None)),
            ColumnType::Datetime => Ok(DataType::Timestamp(TimeUnit::Nanosecond, None)),
            ColumnType::Time => Ok(DataType::Timestamp(TimeUnit::Nanosecond, None)),
            ColumnType::Timespan => Ok(DataType::Duration(TimeUnit::Nanosecond)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_column() {
        let data = r#" {
            "ColumnName": "int_col",
            "ColumnType": "int"
        } "#;

        let c: Column = serde_json::from_str(data).expect("deserialize error");
        let ref_col = Column {
            column_name: "int_col".to_string(),
            column_type: ColumnType::Int,
        };
        assert_eq!(c, ref_col)
    }

    #[test]
    fn field_from_column() {
        let field = Field::new("int_col", DataType::Int32, true);
        let ref_col = Column {
            column_name: "int_col".to_string(),
            column_type: ColumnType::Int,
        };
        let field_from = Field::try_from(ref_col).unwrap();
        assert_eq!(field, field_from)
    }

    #[test]
    fn deserialize_table() {
        let data = r#" {
            "FrameType": "DataTable",
            "TableId": 1,
            "TableName": "Deft",
            "TableKind": "PrimaryResult",
            "Columns": [
                {
                    "ColumnName": "int_col",
                    "ColumnType": "int"
                }
            ],
            "Rows": []
        } "#;

        let t: ResultTable = serde_json::from_str(data).expect("deserialize error");
        let ref_tbl = ResultTable {
            table_id: 1,
            table_name: "Deft".to_string(),
            table_kind: TableKind::PrimaryResult,
            columns: vec![Column {
                column_name: "int_col".to_string(),
                column_type: ColumnType::Int,
            }],
            rows: vec![],
        };
        assert_eq!(t, ref_tbl)
    }

    #[test]
    fn schema_from_table() {
        let tbl = ResultTable {
            table_id: 1,
            table_name: "Deft".to_string(),
            table_kind: TableKind::PrimaryResult,
            columns: vec![Column {
                column_name: "int_col".to_string(),
                column_type: ColumnType::Int,
            }],
            rows: vec![],
        };
        let field = Field::new("int_col", DataType::Int32, true);
        let ref_schema = Schema::new(vec![field]);
        let schema = Schema::try_from(tbl).unwrap();
        assert_eq!(schema, ref_schema);
    }
}
