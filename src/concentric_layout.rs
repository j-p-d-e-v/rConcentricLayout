use crate::Timer;
use crate::cpu::CpuConcentric;
use crate::entities::{Edge, Node, NodePositionData};
use crate::gpu::GpuConcentric;
use rayon::ThreadPoolBuilder;

/// The kind of computing
/// Kinds:
/// - CPU: use cpu parallel computing. It accepts the number of threads as parameter.
/// - GPU: use gpu parallel computing
#[derive(Debug, Clone)]
pub enum ComputingConfig {
    Cpu(usize),
    Gpu,
}

#[derive(Debug)]
pub struct ConcentricLayout {
    pub config: ComputingConfig,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub cx: Option<f32>,
    pub cy: Option<f32>,
    pub timer: Option<Timer>,
}

impl ConcentricLayout {
    pub fn new(
        config: &ComputingConfig,
        nodes: &Vec<Node>,
        edges: &Vec<Edge>,
        cx: &Option<f32>,
        cy: &Option<f32>,
    ) -> Self {
        Self {
            config: config.clone(),
            nodes: nodes.to_owned(),
            edges: edges.to_owned(),
            cx: cx.to_owned(),
            cy: cy.to_owned(),
            timer: None,
        }
    }

    /// Run/Execute the calculation using CPU.
    /// Parameters:
    /// - num_threads: Total number of threads to spawn for parallel computing.
    async fn run_cpu(&mut self, num_threads: usize) -> anyhow::Result<Vec<NodePositionData>> {
        let builder = ThreadPoolBuilder::new().num_threads(num_threads).build()?;
        builder.install(|| -> anyhow::Result<Vec<NodePositionData>> {
            let mut layout = CpuConcentric::new(&self.nodes, &self.edges, &self.cx, &self.cy);
            let result = layout.get()?;
            self.timer = Some(layout.timer);
            Ok(result)
        })
    }

    /// Run/Execute the calculation using GPU. Number of threads is determine automatically based on the number of nodes
    /// multiple by default workgroup size 64. See gpu/wgsl to adjust the configured workgroup_size.
    async fn run_gpu(&mut self) -> anyhow::Result<Vec<NodePositionData>> {
        let mut layout = GpuConcentric::new(&self.nodes, &self.edges, &self.cx, &self.cy);
        let result = layout.get().await?;
        self.timer = Some(layout.timer);
        Ok(result)
    }

    /// The function to call to execute cpu or gpu computation.
    pub async fn execute(&mut self) -> anyhow::Result<Vec<NodePositionData>> {
        match self.config {
            ComputingConfig::Cpu(num_threads) => self.run_cpu(num_threads).await,
            ComputingConfig::Gpu => self.run_gpu().await,
        }
    }
}
