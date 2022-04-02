use error::{FusionPlannerError, Result};
use flight_fusion_ipc::{Signal, SignalProvider};
use petgraph::{algo::toposort, graph::NodeIndex, prelude::DiGraph};
use query_tree::FrameQueryPlanner;
use std::collections::HashMap;

pub mod error;
pub mod frames;
pub mod query_tree;
pub mod test_utils;

#[derive(Debug)]
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
            self.graph.add_edge(signal_idx, node_idx, 0);
        });

        Ok(())
    }

    pub async fn into_frame_query_planner(self) -> Result<FrameQueryPlanner> {
        let mut planner = FrameQueryPlanner::new();

        match toposort(&self.graph, None) {
            Ok(order) => {
                // TODO figure out how to do this in a for_each iterator
                for idx in order {
                    if let Some(FrameGraphNode::Provider(provider)) = self.graph.node_weight(idx) {
                        planner.register_signal_provider(provider).await?;
                    }
                }
            }
            Err(err) => {
                if let Some(weight) = self.graph.node_weight(err.node_id()) {
                    println!("Error graph has cycle at node {:?}", weight);
                }
            }
        };

        Ok(planner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow_deps::arrow::util::pretty;
    use arrow_deps::datafusion::{physical_plan::collect, prelude::SessionContext};

    #[tokio::test]
    async fn create_catalog() {
        let frame = crate::test_utils::get_signal_frame();
        let mut fg = FrameGraph::new();

        frame
            .providers
            .iter()
            .for_each(|p| fg.register_signal_provider(p).unwrap());

        let planner = fg.into_frame_query_planner().await.unwrap();
        let plan = planner.create_physical_plan().await.unwrap();

        let session_ctx = SessionContext::new();
        let task_ctx = session_ctx.task_ctx();
        let results = collect(plan.clone(), task_ctx).await.unwrap();

        pretty::print_batches(&results).unwrap();
    }
}
