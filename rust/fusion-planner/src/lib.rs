use error::{FusionPlannerError, Result};
use flight_fusion_ipc::{
    signal_provider::Source as ProviderSource, table_reference::Table as TableRef, Signal,
    SignalFrame, SignalProvider,
};
use petgraph::{
    graph::NodeIndex,
    prelude::{DiGraph, Direction},
};
use std::collections::HashMap;

pub mod error;
pub mod frames;
pub mod query_tree;
pub mod test_utils;

impl TryInto<query_tree::FrameQueryPlanner> for SignalFrame {
    type Error = FusionPlannerError;

    fn try_into(self) -> std::result::Result<query_tree::FrameQueryPlanner, Self::Error> {
        // - write signal frame into graph
        // - generate context
        todo!()
    }
}

pub enum FrameGraphNode {
    Provider(SignalProvider),
    Signal(Signal),
}

pub struct FrameGraph {
    graph: DiGraph<FrameGraphNode, u32>,
    node_map: HashMap<String, NodeIndex>,
}

impl Default for FrameGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    pub fn register_signal_provider(&mut self, provider: &SignalProvider) -> Result<()> {
        if self.node_map.get(&provider.uid).is_some() {
            return Err(FusionPlannerError::PlanningError(
                "Provider has already been registered".to_string(),
            ));
        }

        let node_idx = self
            .graph
            .add_node(FrameGraphNode::Provider(provider.clone()));
        self.node_map.insert(provider.uid.clone(), node_idx);

        provider.signals.iter().for_each(|s| {
            let signal_idx = if let Some(idx) = self.node_map.get(&s.uid) {
                *idx
            } else {
                let idx = self.graph.add_node(FrameGraphNode::Signal(s.clone()));
                self.node_map.insert(s.uid.clone(), idx);
                idx
            };
            self.graph.add_edge(node_idx, signal_idx, 0);
        });

        provider.inputs.iter().for_each(|s| {
            let signal_idx = if let Some(idx) = self.node_map.get(&s.uid) {
                *idx
            } else {
                let idx = self.graph.add_node(FrameGraphNode::Signal(s.clone()));
                self.node_map.insert(s.uid.clone(), idx);
                idx
            };
            self.graph.add_edge(node_idx, signal_idx, 0);
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_catalog() {
        let frame = crate::test_utils::get_signal_frame();
        let mut fg = FrameGraph::new();

        frame
            .providers
            .iter()
            .for_each(|p| fg.register_signal_provider(p).unwrap());
    }
}
