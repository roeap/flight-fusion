use arrow_flight::{Action, ActionType, FlightData, PutResult, Ticket};
use futures::Stream;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use tonic::{Status, Streaming};

use async_trait::async_trait;

pub mod fusion;
pub mod actions;

pub type BoxedFlightStream<T> =
    Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + Sync + 'static>>;

#[async_trait]
pub trait DoGetHandler: Sync + Send + Debug {
    async fn do_get(&self, ticket: Ticket) -> Result<BoxedFlightStream<FlightData>, Status>;

    fn can_handle_ticket(&self, ticket: Ticket) -> Result<bool, Status>;
}

#[async_trait]
pub trait DoPutHandler: Sync + Send + Debug {
    async fn do_put(
        &self,
        flight_data: FlightData,
        mut stream: Streaming<FlightData>,
    ) -> Result<BoxedFlightStream<PutResult>, Status>;

    fn can_handle_descriptor(&self, ticket: Ticket) -> Result<bool, Status>;
}

/// A Registry holds all the flight handlers at runtime.
pub struct FlightHandlerRegistry {
    pub do_get_handlers: RwLock<HashMap<String, Arc<dyn DoGetHandler>>>,
    pub do_put_handlers: RwLock<HashMap<String, Arc<dyn DoPutHandler>>>,
}

impl fmt::Debug for FlightHandlerRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlightHandlerRegistry")
            .field(
                "do_get",
                &self
                    .do_get_handlers
                    .read()
                    .unwrap()
                    .keys()
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl FlightHandlerRegistry {
    /// Create the registry that flight handlers can registered into.
    pub fn new() -> Self {
        let do_get_map: HashMap<String, Arc<dyn DoGetHandler>> = HashMap::new();
        let do_put_map: HashMap<String, Arc<dyn DoPutHandler>> = HashMap::new();

        Self {
            do_get_handlers: RwLock::new(do_get_map),
            do_put_handlers: RwLock::new(do_put_map),
        }
    }

    /// Adds a new flight handler to this registry.
    /// If a store of the same prefix existed before, it is replaced in the registry and returned.
    pub fn register_do_get_handler(
        &self,
        scheme: String,
        handler: Arc<dyn DoGetHandler>,
    ) -> Option<Arc<dyn DoGetHandler>> {
        let mut stores = self.do_get_handlers.write().unwrap();
        stores.insert(scheme, handler)
    }

    pub fn register_do_put_handler(
        &self,
        scheme: String,
        handler: Arc<dyn DoPutHandler>,
    ) -> Option<Arc<dyn DoPutHandler>> {
        let mut stores = self.do_put_handlers.write().unwrap();
        stores.insert(scheme, handler)
    }

    /// Get the do_get handler registered for scheme
    pub fn get_do_get(&self, scheme: &str) -> Option<Arc<dyn DoGetHandler>> {
        let stores = self.do_get_handlers.read().unwrap();
        stores.get(scheme).cloned()
    }

    pub fn get_do_put(&self, scheme: &str) -> Option<Arc<dyn DoPutHandler>> {
        let stores = self.do_put_handlers.read().unwrap();
        stores.get(scheme).cloned()
    }

}
