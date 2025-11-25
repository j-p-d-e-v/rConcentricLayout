pub mod cpu;
pub mod edge;
pub mod entities;
pub mod gpu;
pub mod node;
pub mod timer;
pub use edge::Edge;
pub use node::Node;
pub use timer::Timer;

#[cfg(test)]
pub mod test_concetric_layout {
    use std::{io::Write, path::Path};

    use super::*;
    use chrono::Local;
    use cpu::CpuConcentric;
    use rayon::ThreadPoolBuilder;
    use serde::{Deserialize, Serialize};
    use tabular::{Row, Table};

    #[test]
    fn test_cpu_based() {
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
            "sample_graph_1000.json",                                 //6
            "sample_tree_1000.json",                                  //7
            "sample_scalefree_1000.json",                             //8
            "graph_10000.json",                                       //9,
            "graph_20000.json",                                       //10,
            "graph_50000.json",                                       //11,
            "graph_100000.json",                                      //12
            "telco_realistic_1000_nodes.json",                        //13
        ];
        let mut benchmark: Vec<String> = Vec::new();
        for total_threads in [2, 4, 8, 16, 32] {
            let thread_pool = ThreadPoolBuilder::new()
                .num_threads(total_threads)
                .build()
                .unwrap();
            thread_pool.install(|| {
                let mut table = Table::new(
                    "| {:^} threads | {:<} | {:^} nodes | {:^} edges | {:^}s | {:^}ms | {:^}us |",
                );

                for (_sample_index, sample_file) in samples.iter().enumerate() {
                    let sample_data_reader = std::fs::File::options()
                        .read(true)
                        .open(format!("storage/sample-data/{}", sample_file))
                        .unwrap();
                    let sample_data_1 =
                        serde_json::from_reader::<_, SampleData>(sample_data_reader).unwrap();

                    let mut layout = CpuConcentric::new(CpuConcentric {
                        nodes: sample_data_1.nodes,
                        edges: sample_data_1.edges,
                        default_cx: Some(0.0),
                        default_cy: Some(0.0),
                        ..Default::default()
                    });
                    let result = layout.get();
                    let timer = layout.timer.clone();
                    let data = result.unwrap();
                    assert_eq!(data.nodes.len(), data.coordinates.len());
                    table = table.with_row(
                        Row::new()
                            .with_cell(thread_pool.current_num_threads())
                            .with_cell(sample_file)
                            .with_cell(layout.nodes.len())
                            .with_cell(layout.edges.len())
                            .with_cell(timer.clone().seconds.unwrap().to_string())
                            .with_cell(timer.clone().millis.unwrap().to_string())
                            .with_cell(timer.clone().micros.unwrap().to_string()),
                    );
                    // let writer = std::fs::File::options()
                    //     .truncate(true)
                    //     .create(true)
                    //     .write(true)
                    //     .open(format!("storage/calculation-{}", sample_file))
                    //     .unwrap();
                    // serde_json::to_writer_pretty(writer, &layout).unwrap();
                    let writer = std::fs::File::options()
                        .truncate(true)
                        .create(true)
                        .write(true)
                        .open(format!("storage/output-{}", sample_file))
                        .unwrap();
                    serde_json::to_writer_pretty(writer, &data).unwrap();
                }

                benchmark.push(table.to_string());
            });
        }
        let file_path = format!("storage/cpu-benchmark-{}", Local::now().timestamp());
        let file_path = Path::new(&file_path);
        let data = benchmark.join("\n\n");
        let mut file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)
            .unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
}
