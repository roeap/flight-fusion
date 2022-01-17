use error::FlightFusionClientError;
use flight_fusion_client::{
    arrow::{datatypes::Schema, record_batch::RecordBatch},
    flight_fusion_ipc::{
        CommandDropSource, CommandExecuteQuery, CommandGetSchema, CommandReadDataset,
        CommandWriteIntoDataset,
    },
    FlightFusionClient,
};
use observability_deps::{
    opentelemetry::{global, sdk::propagation::TraceContextPropagator},
    opentelemetry_jaeger, tracing_opentelemetry, tracing_subscriber,
    tracing_subscriber::prelude::*,
};
use prost::Message;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use utils::wait_for_future;

mod error;
mod utils;

#[pyclass(module = "flight_fusion")]
struct FusionClient {
    host: String,
    port: i32,
}

#[pymethods]
impl FusionClient {
    #[new]
    fn new(host: String, port: i32) -> PyResult<Self> {
        Ok(Self { host, port })
    }

    fn write_into_table<'py>(
        &self,
        py: Python<'py>,
        command: Vec<u8>,
        batches: Vec<RecordBatch>,
    ) -> PyResult<&'py PyBytes> {
        let command = CommandWriteIntoDataset::decode(command.as_ref())
            .map_err(FlightFusionClientError::from)?;
        let op = async {
            let client = FlightFusionClient::try_new(&self.host, self.port).await?;
            client.write_into_table(command, batches).await
        };
        let response = wait_for_future(py, op).map_err(FlightFusionClientError::from)?;
        let obj = serialize_message(response).map_err(FlightFusionClientError::from)?;
        Ok(PyBytes::new(py, &obj))
    }

    fn read_table<'py>(&self, py: Python<'py>, command: Vec<u8>) -> PyResult<Vec<RecordBatch>> {
        let command =
            CommandReadDataset::decode(command.as_ref()).map_err(FlightFusionClientError::from)?;
        let op = async {
            let client = FlightFusionClient::try_new(&self.host, self.port).await?;
            client.read_table(command).await
        };
        let response = wait_for_future(py, op).map_err(FlightFusionClientError::from)?;
        Ok(response)
    }

    fn get_schema<'py>(&self, py: Python<'py>, command: Vec<u8>) -> PyResult<Schema> {
        let command =
            CommandGetSchema::decode(command.as_ref()).map_err(FlightFusionClientError::from)?;
        let op = async {
            let client = FlightFusionClient::try_new(&self.host, self.port).await?;
            client.get_schema(command).await
        };
        let response = wait_for_future(py, op).map_err(FlightFusionClientError::from)?;
        Ok(response)
    }

    fn execute_query<'py>(&self, py: Python<'py>, command: Vec<u8>) -> PyResult<Vec<RecordBatch>> {
        let command =
            CommandExecuteQuery::decode(command.as_ref()).map_err(FlightFusionClientError::from)?;
        let op = async {
            let client = FlightFusionClient::try_new(&self.host, self.port).await?;
            client.execute_query(command).await
        };
        let response = wait_for_future(py, op).map_err(FlightFusionClientError::from)?;
        Ok(response)
    }

    fn put_memory_table<'py>(
        &self,
        py: Python<'py>,
        table_ref: &str,
        batches: Vec<RecordBatch>,
    ) -> PyResult<&'py PyBytes> {
        let op = async {
            let client = FlightFusionClient::try_new(&self.host, self.port).await?;
            client.put_memory_table(table_ref, batches).await
        };
        let response = wait_for_future(py, op).map_err(FlightFusionClientError::from)?;
        let obj = serialize_message(response).map_err(FlightFusionClientError::from)?;
        Ok(PyBytes::new(py, &obj))
    }

    fn drop_table<'py>(&self, py: Python<'py>, command: Vec<u8>) -> PyResult<&'py PyBytes> {
        let command =
            CommandDropSource::decode(command.as_ref()).map_err(FlightFusionClientError::from)?;
        let op = async {
            let client = FlightFusionClient::try_new(&self.host, self.port).await?;
            client.drop_table(command).await
        };
        let response = wait_for_future(py, op).map_err(FlightFusionClientError::from)?;
        let obj = serialize_message(response).map_err(FlightFusionClientError::from)?;
        Ok(PyBytes::new(py, &obj))
    }
}

fn serialize_message<T: prost::Message>(
    msg: T,
) -> std::result::Result<Vec<u8>, prost::EncodeError> {
    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf)?;
    Ok(buf)
}

/// Low-level flight fusion internal package.
///
/// The higher-level public API is defined in pure python files under the
/// flight_fusion directory.
#[pymodule]
fn _internal(_py: Python, m: &PyModule) -> PyResult<()> {
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name("grpc-client")
        .install_simple()
        .unwrap();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()
        .unwrap();

    m.add_class::<FusionClient>()?;
    // m.add(
    //     "FlightFusionClientError",
    //     py.get_type::<FlightFusionClientError>(),
    // )?;
    Ok(())
}
