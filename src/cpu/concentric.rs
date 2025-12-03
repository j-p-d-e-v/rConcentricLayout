use crate::Timer;
use crate::cpu::{NodeConnections, NodePositions, Normalize};
use crate::entities::{Edge, Node, NodeConnectionsData, NodePositionData, NormalizeData};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuConcentric {
    pub timer: Timer,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub node_connections: NodeConnectionsData,
    pub normalized_values: NormalizeData,
    pub node_positions: Vec<NodePositionData>,
    pub default_cx: Option<f32>,
    pub default_cy: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuConcetricData {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub coordinates: Vec<NodePositionData>,
}

impl CpuConcentric {
    pub fn new(data: CpuConcentric) -> Self {
        data
    }

    pub fn get(&mut self) -> anyhow::Result<CpuConcetricData> {
        let timer = Instant::now();
        self.count_node_connections()?;
        self.normalize_node_connections()?;
        self.calculate_node_positions()?;
        let elapsed = timer.elapsed();
        let data = self.node_positions.clone();
        self.timer = Timer {
            micros: Some(elapsed.as_micros()),
            millis: Some(elapsed.as_millis()),
            seconds: Some(elapsed.as_secs()),
        };
        Ok(CpuConcetricData {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            coordinates: data,
        })
    }

    /// 1. Count the number of edges/paths per node
    pub fn count_node_connections(&mut self) -> anyhow::Result<()> {
        let result = NodeConnections::get(&self.nodes, &self.edges)?;
        self.node_connections = result.clone();
        Ok(())
    }

    /// 2. Normalize Node Connections
    pub fn normalize_node_connections(&mut self) -> anyhow::Result<()> {
        let result = Normalize::get(&self.node_connections)?;
        self.normalized_values = result.clone();
        Ok(())
    }

    /// 3. Node Posititons
    pub fn calculate_node_positions(&mut self) -> anyhow::Result<()> {
        self.node_positions =
            NodePositions::get(&self.normalized_values, self.default_cx, self.default_cy);
        Ok(())
    }
}
