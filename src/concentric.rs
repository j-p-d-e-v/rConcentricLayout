use std::time::Instant;

use crate::{
    Edge, Node, NodeAngle, NodeConnections, NodeCoordinate, NormalizeNodeConnections, RingIndexes,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Timer {
    pub micros: Option<u128>,
    pub millis: Option<u128>,
    pub seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Concentric {
    pub timer: Timer,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub node_connections: NodeConnections,
    pub normalized_values: NormalizeNodeConnections,
    pub node_angles: Vec<NodeAngle>,
    pub node_coordinates: Vec<NodeCoordinate>,
    pub ring_indexes: RingIndexes,
    pub default_cx: Option<f32>,
    pub default_cy: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcetricData {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub coordinates: Vec<NodeCoordinate>,
}

impl Concentric {
    pub fn new(data: Concentric) -> Self {
        data
    }

    pub fn get(&mut self) -> anyhow::Result<ConcetricData> {
        let timer = Instant::now();
        self.count_node_connections()?;
        self.normalize_node_connections()?;
        self.calculate_rings_index()?;
        self.calculate_nodes_angle()?;
        self.calculate_nodes_coordinate()?;
        let elapsed = timer.elapsed();
        let data = self.node_coordinates.clone();
        self.timer = Timer {
            micros: Some(elapsed.as_micros()),
            millis: Some(elapsed.as_millis()),
            seconds: Some(elapsed.as_secs()),
        };
        Ok(ConcetricData {
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
        let result = NormalizeNodeConnections::get(&self.node_connections)?;
        self.normalized_values = result.clone();
        Ok(())
    }

    /// 3. Ring Indexs
    pub fn calculate_rings_index(&mut self) -> anyhow::Result<()> {
        self.ring_indexes = RingIndexes::get(&self.normalized_values)?;
        Ok(())
    }

    /// 4. Nodes Angle
    pub fn calculate_nodes_angle(&mut self) -> anyhow::Result<()> {
        self.node_angles = NodeAngle::get(&self.ring_indexes)?;
        Ok(())
    }

    /// 5. Nodes Coordinate
    pub fn calculate_nodes_coordinate(&mut self) -> anyhow::Result<()> {
        self.node_coordinates = NodeCoordinate::get(
            &self.node_angles,
            &self.ring_indexes,
            //           &self.rings_radius,
            self.default_cx,
            self.default_cy,
        )?;
        Ok(())
    }
}
