use crate::Timer;
use crate::cpu::{NodeAngle, NodeConnections, NodeCoordinate, NormalizeNodeConnections, Ring};
use crate::entities::NodeConnectionsData;
use crate::{Edge, Node};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CpuConcentric {
    pub timer: Timer,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub node_connections: NodeConnectionsData,
    pub normalized_values: NormalizeNodeConnections,
    pub node_angles: Vec<NodeAngle>,
    pub node_coordinates: Vec<NodeCoordinate>,
    pub rings: Vec<Ring>,
    pub default_cx: Option<f32>,
    pub default_cy: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuConcetricData {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub coordinates: Vec<NodeCoordinate>,
}

impl CpuConcentric {
    pub fn new(data: CpuConcentric) -> Self {
        data
    }

    pub fn get(&mut self) -> anyhow::Result<CpuConcetricData> {
        let timer = Instant::now();
        self.count_node_connections()?;
        self.normalize_node_connections()?;
        self.calculate_rings()?;
        self.calculate_nodes_angle()?;
        self.calculate_nodes_coordinate()?;
        let elapsed = timer.elapsed();
        let data = self.node_coordinates.clone();
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
        let result = NormalizeNodeConnections::get(&self.node_connections)?;
        self.normalized_values = result.clone();
        Ok(())
    }

    /// 3. Ring Indexs
    pub fn calculate_rings(&mut self) -> anyhow::Result<()> {
        self.rings = Ring::get(&self.normalized_values)?;
        Ok(())
    }

    /// 4. Nodes Angle
    pub fn calculate_nodes_angle(&mut self) -> anyhow::Result<()> {
        self.node_angles = NodeAngle::get(&self.rings)?;
        Ok(())
    }

    /// 5. Nodes Coordinate
    pub fn calculate_nodes_coordinate(&mut self) -> anyhow::Result<()> {
        self.node_coordinates = NodeCoordinate::get(
            &self.node_angles,
            &self.rings,
            self.default_cx,
            self.default_cy,
        )?;
        Ok(())
    }
}
