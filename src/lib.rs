pub mod concentric;
pub mod edge;
pub mod node;
pub mod node_connections;
pub mod normalize;
pub mod ring;
pub use concentric::Concentric;
pub use edge::Edge;
pub use node::Node;
pub use node_connections::{NodeConnectionValue, NodeConnections};
pub use normalize::{NormalizeNodeConnections, NormalizedValue};
pub use ring::{RingIndexValue, RingIndexes};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingIndex {
    nodes: Vec<String>,
    index: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreRadius {
    pub node: String,
    pub score: f32,
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RingRadius {
    pub ring: usize,
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAngle {
    pub node: String,
    pub angle_radian: f32,
    pub angle_degree: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCoordinate {
    pub cx: f32,
    pub cy: f32,
    pub radius: f32,
    pub x: f32,
    pub y: f32,
    pub node: String,
}

#[cfg(test)]
pub mod test_concetric_layout {
    use super::*;
    use std::{
        collections::{HashMap, HashSet},
        f32,
    };

    use super::*;

    #[test]
    fn test() {
        let nodes: Vec<Node> = vec![
            Node {
                id: "A".to_string(),
                label: "Node A".to_string(),
            },
            Node {
                id: "B".to_string(),
                label: "Node B".to_string(),
            },
            Node {
                id: "C".to_string(),
                label: "Node C".to_string(),
            },
            Node {
                id: "D".to_string(),
                label: "Node D".to_string(),
            },
            Node {
                id: "E".to_string(),
                label: "Node E".to_string(),
            },
            Node {
                id: "F".to_string(),
                label: "Node F".to_string(),
            },
        ];
        let edges: Vec<Edge> = vec![
            Edge {
                id: "AB".to_string(),
                source: "A".to_string(),
                target: "B".to_string(),
            },
            Edge {
                id: "AC".to_string(),
                source: "A".to_string(),
                target: "C".to_string(),
            },
            Edge {
                id: "AD".to_string(),
                source: "A".to_string(),
                target: "D".to_string(),
            },
            Edge {
                id: "AF".to_string(),
                source: "A".to_string(),
                target: "F".to_string(),
            },
            Edge {
                id: "AE".to_string(),
                source: "A".to_string(),
                target: "E".to_string(),
            },
            Edge {
                id: "CD".to_string(),
                source: "C".to_string(),
                target: "D".to_string(),
            },
            Edge {
                id: "CF".to_string(),
                source: "C".to_string(),
                target: "F".to_string(),
            },
            Edge {
                id: "DE".to_string(),
                source: "D".to_string(),
                target: "E".to_string(),
            },
            Edge {
                id: "CE".to_string(),
                source: "C".to_string(),
                target: "E".to_string(),
            },
            Edge {
                id: "DF".to_string(),
                source: "D".to_string(),
                target: "F".to_string(),
            },
            Edge {
                id: "EB".to_string(),
                source: "E".to_string(),
                target: "B".to_string(),
            },
            Edge {
                id: "FB".to_string(),
                source: "F".to_string(),
                target: "B".to_string(),
            },
        ];
        // Count the connections per node
        let mut node_connections: Vec<NodeConnection> = Vec::new();
        for n in &nodes {
            let total = edges
                .iter()
                .filter(|item| item.source == n.id || item.target == n.id)
                .count();
            node_connections.push(NodeConnection {
                node_id: n.id.clone(),
                total,
            });
        }
        node_connections.sort_by(|a, b| b.total.cmp(&a.total));
        for i in &node_connections {
            println!("Node: {}, Total: {}", i.node_id, i.total);
        }

        // Normalize
        // Formula: normalized_value = (degree - min_degree) / (max_degree - min_degree)
        // degree - is the number of edges per nodes. Refer to the connections per node count

        let mut normalized_values: Vec<NormalizedValue> = Vec::new();
        let min_degree = node_connections.clone().last().unwrap().total;
        let max_degree = node_connections.clone().first().unwrap().total;
        println!("{}", "=".repeat(10));
        println!("Normalization");
        println!("Min Degree: {}", min_degree);
        println!("Max Degree: {}", max_degree);

        for n in node_connections {
            let normalized_value = (n.total - min_degree) as f32 / (max_degree - min_degree) as f32;
            normalized_values.push(NormalizedValue {
                node_id: n.node_id.clone(),
                degree: n.total.clone(),
                min_degree: min_degree.clone(),
                max_degree: max_degree.clone(),
                normalized_value,
            });
        }
        println!("NodeId\tDegree\tMin\tMax\tNormalized");
        for n in &normalized_values {
            println!(
                "{}\t{}\t{}\t{}\t{}",
                n.node_id, n.degree, n.min_degree, n.max_degree, n.normalized_value
            );
        }

        // Ring Index
        // Decide how many rings. Now we choose 4
        // Formula: ring_index = floor((1 - s) x R)
        // R - total rings
        // 1 - highesr normalized value
        // s - the normalized value for reach nodes

        println!("{}", "=".repeat(10));
        let mut ring_indexes: HashMap<usize, Vec<String>> = HashMap::new();
        let highest_normalized_value = &normalized_values
            .iter()
            .max_by(|a, b| a.normalized_value.total_cmp(&b.normalized_value))
            .unwrap()
            .normalized_value;
        let total_rings = 4;
        for n in &normalized_values {
            let mut ring_index = ((*highest_normalized_value - n.normalized_value)
                * total_rings as f32)
                .floor() as usize;
            if ring_index > (total_rings - 1) as usize {
                ring_index = total_rings - 1;
            }
            ring_indexes
                .entry(ring_index)
                .and_modify(|item| item.push(n.node_id.clone()))
                .or_insert(vec![n.node_id.clone()]);
        }
        let mut ring_indexes = ring_indexes
            .iter()
            .map(|(index, values)| RingIndex {
                index: index.to_owned() as u64,
                nodes: values.to_owned(),
            })
            .collect::<Vec<RingIndex>>();

        ring_indexes.sort_by(|a, b| a.index.cmp(&b.index));
        println!("Ring\tNodes");
        for n in &ring_indexes {
            println!("{}\t{}", n.index, n.nodes.join(", "));
        }

        // Radius
        // Formula:
        println!("{}", "=".repeat(10));
        let r_max = 280.0; // Outer Most

        // Score Based Radius (Ignoring Rings)
        // radius = r_min ( highest_normalized_value - normalized_value) * (r_max - r_min)
        let mut scores_radius: Vec<ScoreRadius> = Vec::new();
        for n in &normalized_values {
            let r_min = if !scores_radius.is_empty() { 40.0 } else { 0.0 };
            let radius = r_min + (highest_normalized_value - n.normalized_value) * (r_max - r_min);
            scores_radius.push(ScoreRadius {
                node: n.node_id.clone(),
                score: n.normalized_value.clone(),
                radius,
            });
        }
        println!("Score Based Radius");
        println!("Node\tScore\tRadius");
        for i in &scores_radius {
            println!("{}\t{}\t{}", i.node, i.score, i.radius);
        }

        println!("{}", "-".repeat(10));
        // Ring Based Radius
        // Formula:
        // radius = r_min + ring_index + r_step
        // r_step = incremental spacing
        println!("Ring based Radius");
        let mut rings_radius: Vec<RingRadius> = Vec::new();
        let mut r_step = 0;
        for r in &ring_indexes {
            let r_min = if !rings_radius.is_empty() { 40.0 } else { 0.0 };
            let radius = r_min + r.index as f32 * r_step as f32;
            rings_radius.push(RingRadius {
                ring: r.index.clone() as usize,
                radius,
            });
            r_step += 20;
        }
        println!("Ring\tRadius");
        for i in &rings_radius {
            println!("{}\t{}", i.ring, i.radius);
        }

        println!("{}", "=".repeat(10));
        // Angle
        // angle = start_angle + (2 * PI) * (k / n)
        // k - node index
        // n - total nodes - 1

        let mut nodes_angle: Vec<NodeAngle> = Vec::new();
        for r in &ring_indexes {
            let nodes = &r.nodes;
            let mut k = 0u64;
            let n = nodes.len();

            let mut start_angle = 0_f32;
            for node in nodes {
                let new_start_angle = start_angle * (f32::consts::PI / 180_f32);
                let angle_radian =
                    new_start_angle + (2_f32 * f32::consts::PI) * (k as f32 / n as f32);
                let angle_degree = angle_radian * (180_f32 / f32::consts::PI);
                nodes_angle.push(NodeAngle {
                    node: node.clone(),
                    angle_radian,
                    angle_degree,
                });
                k += 1;

                start_angle += 30.0;
            }
        }
        println!("Node\tAngle(Rad)\tAngle(Deg)");
        for n in &nodes_angle {
            println!("{}\t{}\t{}", n.node, n.angle_radian, n.angle_degree);
        }

        // Computing Cx, and Cy
        // Formula:
        // x = cx + r * cos(theta or radian)
        // y = cy + r * sin(theta or radian)
        let cx = 0_f32;
        let cy = 0_f32;
        let mut nodes_coordinate: Vec<NodeCoordinate> = Vec::new();
        for n in &nodes_angle {
            let ring_index = ring_indexes
                .iter()
                .find(|item| item.nodes.contains(&n.node))
                .unwrap()
                .index;
            let ring_radius = rings_radius
                .iter()
                .find(|item| item.ring == ring_index as usize)
                .unwrap()
                .radius;
            let x = cx + ring_radius * n.angle_radian.cos();
            let y = cy + ring_radius * n.angle_radian.sin();
            nodes_coordinate.push(NodeCoordinate {
                cx: cx.clone(),
                cy: cy.clone(),
                x,
                y,
                radius: ring_radius,
                node: n.node.clone(),
            });
        }
        println!("{}", "-".repeat(10));
        println!("Node\tRadius\tCx\tCy\tX\tY");
        for n in &nodes_coordinate {
            println!(
                "{}\t{}\t{}\t{}\t{}\t{}",
                n.node, n.radius, n.cx, n.cy, n.x, n.y
            );
        }

        #[derive(Debug, Clone, Deserialize, Serialize)]
        pub struct Data {
            nodes: Vec<Node>,
            edges: Vec<Edge>,
            coordinates: Vec<NodeCoordinate>,
        }
        let data = Data {
            nodes,
            edges,
            coordinates: nodes_coordinate,
        };

        let mut writer = std::fs::File::options()
            .truncate(true)
            .write(true)
            .create(true)
            .open("storage/concentric.json")
            .unwrap();
        serde_json::to_writer_pretty(writer, &data).unwrap();
    }
}

