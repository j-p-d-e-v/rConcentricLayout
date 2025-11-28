pub mod ring_index_assignment;
pub use ring_index_assignment::RingIndexAssignment;

#[cfg(test)]
pub mod test_gpu_ring {
    use serde::Deserialize;

    use crate::{
        Edge, Node,
        gpu::{GpuData, normalize::NormalizeResult, ring::RingIndexAssignment},
    };

    #[tokio::test]
    async fn test_ring() {
        #[derive(Debug, Clone, Deserialize)]
        struct SampleData {
            nodes: Vec<Node>,
            edges: Vec<Edge>,
        }
        let node_connections_reader = std::fs::File::options()
            .read(true)
            .open("storage/gpu-normalize.json")
            .unwrap();
        let sample_data_reader = std::fs::File::options()
            .read(true)
            .open("storage/sample-data/sample-data.json")
            .unwrap();
        let normalize_data =
            serde_json::from_reader::<_, NormalizeResult>(node_connections_reader).unwrap();
        let sample_data = serde_json::from_reader::<_, SampleData>(sample_data_reader).unwrap();
        let gpu_data = GpuData::new(&sample_data.nodes, &sample_data.edges);
        let gpu_data = gpu_data.unwrap();
        let ring_index_assignment = RingIndexAssignment::new(&gpu_data, normalize_data).await;
        assert!(
            ring_index_assignment.is_ok(),
            "{:?}",
            ring_index_assignment.err()
        );
        let ring_index_assignment = ring_index_assignment.unwrap();
        let result = ring_index_assignment.execute().await;
        assert!(result.is_ok(), "{:?}", result.err());
        let result = result.unwrap();
        // let mut writer = std::fs::File::options()
        //     .create(true)
        //     .truncate(true)
        //     .write(true)
        //     .open("storage/gpu-normalize.json")
        //     .unwrap();
        // serde_json::to_writer_pretty(&mut writer, &result).unwrap();
    }
}
