use arrow_deps::{
    arrow::temporal_conversions,
    datafusion::parquet::{
        errors::Result as ParquetResult,
        file::reader::{ChunkReader, Length},
    },
};
use parquet_format::TimeUnit;
use std::io::Read;

#[derive(Debug)]
pub struct BytesReader(pub bytes::Bytes);

impl Length for BytesReader {
    fn len(&self) -> u64 {
        self.0.len() as u64
    }
}

impl Read for BytesReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(Read::read(&mut self.0.as_ref(), buf)?)
    }
}

impl ChunkReader for BytesReader {
    type T = Box<dyn Read + Send + Sync>;

    fn get_read(&self, start: u64, length: usize) -> ParquetResult<Self::T> {
        let start = start as usize;
        Ok(Box::new(BytesReader(self.0.slice(start..start + length))))
    }
}

/// Convert an ISO-8601/RFC3339 timestamp string to a numeric microsecond epoch representation.
/// Stats strings are written with millisecond precision as described by the delta protocol.
pub fn timestamp_micros_from_stats_string(s: &str) -> Result<i64, chrono::format::ParseError> {
    chrono::DateTime::parse_from_rfc3339(s).map(|dt| dt.timestamp_millis() * 1000)
}

/// Convert the timestamp to a ISO-8601 style format suitable for JSON statistics.
pub fn timestamp_to_delta_stats_string(n: i64, time_unit: &TimeUnit) -> String {
    let dt = match time_unit {
        TimeUnit::MILLIS(_) => temporal_conversions::timestamp_ms_to_datetime(n),
        TimeUnit::MICROS(_) => temporal_conversions::timestamp_us_to_datetime(n),
        TimeUnit::NANOS(_) => temporal_conversions::timestamp_ns_to_datetime(n),
    };

    format!("{}", dt.format("%Y-%m-%dT%H:%M:%S%.3fZ"))
}

#[cfg(test)]
mod tests {
    use crate::area_store::BytesReader;
    use arrow_deps::datafusion::parquet::{
        arrow::{ArrowReader, ParquetFileArrowReader},
        file::serialized_reader::SerializedFileReader,
    };
    use bytes::Bytes;
    use object_store::{path::ObjectStorePath, ObjectStoreApi};
    use std::sync::Arc;

    #[tokio::test]
    async fn read_bytes_reader() {
        let storage = crate::test_utils::get_test_object_store();

        let mut location = storage.new_path();
        location.push_dir("data");
        location.set_file_name("P1.parquet");

        let obj_reader = BytesReader(Bytes::from(
            storage.get(&location).await.unwrap().bytes().await.unwrap(),
        ));
        let file_reader = Arc::new(SerializedFileReader::new(obj_reader).unwrap());
        let mut arrow_reader = ParquetFileArrowReader::new(file_reader);
        let schema = Arc::new(arrow_reader.get_schema().unwrap());

        assert!(schema.fields().len() > 1)
    }
}
