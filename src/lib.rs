pub mod concentric;
pub mod edge;
pub mod node;
pub mod node_angle;
pub mod node_connections;
pub mod node_coordinate;
pub mod normalize;
pub mod ring;
pub use concentric::Concentric;
pub use edge::Edge;
pub use node::Node;
pub use node_angle::NodeAngle;
pub use node_connections::{NodeConnectionValue, NodeConnections};
pub use node_coordinate::NodeCoordinate;
pub use normalize::{NormalizeNodeConnections, NormalizedValue};
pub use ring::{RingIndexValue, RingIndexes};

#[cfg(test)]
pub mod test_concetric_layout {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[test]
    fn test() {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        pub struct SampleData {
            nodes: Vec<Node>,
            edges: Vec<Edge>,
        }
        let samples = [
            "concentric_nonmesh_star_100.json",                       //0
            "sample-data-100-nodes-full-mesh-15-rings-neighbor.json", //1
            "sample-data-100-nodes-full-mesh-15-rings.json",          //2
            "sample-data-100-nodes-full-mesh.json",                   //3
            "sample-data-cytoscape.json",                             //4
            "sample-data.json",                                       //5
        ];
        for (_sample_index, sample_file) in samples.iter().enumerate() {
            let sample_data_reader = std::fs::File::options()
                .read(true)
                .open(format!("storage/sample-data/{}", sample_file))
                .unwrap();
            let sample_data_1 =
                serde_json::from_reader::<_, SampleData>(sample_data_reader).unwrap();

            let mut layout = Concentric::new(Concentric {
                nodes: sample_data_1.nodes,
                edges: sample_data_1.edges,
                default_cx: Some(0.0),
                default_cy: Some(0.0),
                ..Default::default()
            });
            let result = layout.get();
            println!("Timer: {:#?}", layout.timer);
            assert!(result.is_ok(), "{:#?}", result.err());

            let writer = std::fs::File::options()
                .truncate(true)
                .create(true)
                .write(true)
                .open(format!("storage/calculation-{}", sample_file))
                .unwrap();
            serde_json::to_writer_pretty(writer, &layout).unwrap();
            let writer = std::fs::File::options()
                .truncate(true)
                .create(true)
                .write(true)
                .open(format!("storage/output-{}", sample_file))
                .unwrap();
            serde_json::to_writer_pretty(writer, &result.unwrap()).unwrap();
        }
    }
}
