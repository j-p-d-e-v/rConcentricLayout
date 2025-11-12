pub mod concentric;
pub mod edge;
pub mod node;
pub mod node_angle;
pub mod node_connections;
pub mod node_coordinate;
pub mod normalize;
pub mod radius;
pub mod ring;
pub use concentric::Concentric;
pub use edge::Edge;
pub use node::Node;
pub use node_angle::NodeAngle;
pub use node_connections::{NodeConnectionValue, NodeConnections};
pub use node_coordinate::NodeCoordinate;
pub use normalize::{NormalizeNodeConnections, NormalizedValue};
pub use radius::Radius;
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
        let sample_data_1_reader = std::fs::File::options()
            .read(true)
            .open("storage/sample-data/sample-data-cytoscape.json")
            .unwrap();
        let sample_data_1 = serde_json::from_reader::<_, SampleData>(sample_data_1_reader).unwrap();

        let mut layout = Concentric::new(Concentric {
            nodes: sample_data_1.nodes,
            edges: sample_data_1.edges,
            step_angle: Some(15.0),
            step_radius: Some(50),
            min_radius: Some(20),
            default_cx: Some(0.0),
            default_cy: Some(0.0),
            total_rings: Some(4),
            ..Default::default()
        });
        let result = layout.get();
        println!("Timer: {:#?}", layout.timer);
        assert!(result.is_ok(), "{:#?}", result.err());

        let writer = std::fs::File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(r"storage/concentric-calculation.json")
            .unwrap();
        serde_json::to_writer_pretty(writer, &layout).unwrap();
        let writer = std::fs::File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(r"storage/concentric.json")
            .unwrap();
        serde_json::to_writer_pretty(writer, &result.unwrap()).unwrap();
    }
}
