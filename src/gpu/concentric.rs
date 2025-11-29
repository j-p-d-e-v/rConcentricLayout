use crate::Timer;
use crate::entities::NodePositionData;
use crate::gpu::node_positions::{NodePositions, NodePositionsResult};
use crate::gpu::normalize::{Normalize, NormalizeResult};
use crate::gpu::{GpuData, NodeConnections, NodeConnectionsResult};
use crate::{Edge, Node};
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GpuConcentric {
    pub timer: Timer,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub gpu_data: GpuData,
    pub node_connections: NodeConnectionsResult,
    pub normalized_values: NormalizeResult,
    pub node_positions: NodePositionsResult,
    pub default_cx: Option<f32>,
    pub default_cy: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConcetricData {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub positions: Vec<NodePositionData>,
}

impl GpuConcentric {
    pub fn new(data: GpuConcentric) -> Self {
        data
    }

    pub async fn get(&mut self) -> anyhow::Result<GpuConcetricData> {
        let timer = Instant::now();
        self.count_node_connections().await?;
        self.normalize_node_connections().await?;
        self.calculate_node_positions().await?;
        let elapsed = timer.elapsed();
        let data = self.node_positions.data.clone();
        self.timer = Timer {
            micros: Some(elapsed.as_micros()),
            millis: Some(elapsed.as_millis()),
            seconds: Some(elapsed.as_secs()),
        };
        Ok(GpuConcetricData {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            positions: data,
        })
    }

    /// 1. Count the number of edges/paths per node
    pub async fn count_node_connections(&mut self) -> anyhow::Result<()> {
        let node_connections = NodeConnections::new(&self.gpu_data).await?;
        self.node_connections = node_connections.execute().await?;
        Ok(())
    }

    /// 2. Normalize Node Connections
    pub async fn normalize_node_connections(&mut self) -> anyhow::Result<()> {
        let normalize = Normalize::new(&self.gpu_data, &self.node_connections).await?;
        self.normalized_values = normalize.execute().await?;
        Ok(())
    }

    /// 3. Calculate Node Positions (Ring, Angle, and Coordinates)
    pub async fn calculate_node_positions(&mut self) -> anyhow::Result<()> {
        let node_positions = NodePositions::new(
            &self.gpu_data,
            self.normalized_values.clone(),
            self.default_cx,
            self.default_cy,
        )
        .await?;
        self.node_positions = node_positions.execute().await?;
        Ok(())
    }
}
