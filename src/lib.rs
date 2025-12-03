pub mod cpu;
pub mod entities;
pub mod gpu;
pub mod timer;
pub use timer::Timer;

#[cfg(test)]
pub mod test_concentric_layout {
    use super::*;
    use crate::gpu::GpuConcentric;
    use chrono::Local;
    use cpu::CpuConcentric;
    use entities::{Edge, Node};
    use rayon::ThreadPoolBuilder;
    use serde::{Deserialize, Serialize};
    use std::{io::Write, path::Path};
    use tabular::{Row, Table};

    #[test]
    fn test_cpu_based() {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        pub struct SampleData {
            nodes: Vec<Node>,
            edges: Vec<Edge>,
        }
        let samples = [
            "nodes_10_full_mesh.json",
            "nodes_100_full_mesh.json",
            "nodes_1000_random.json",
            "nodes_2000_random.json",
            "nodes_5000_random.json",
            "nodes_10000_random.json",
            "nodes_50000_random.json",
            "nodes_100000_random.json",
            "telco_sample.json",
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
                        .open(format!("storage/output-cpu-{}", sample_file))
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

    #[tokio::test]
    async fn test_gpu_based() {
        #[derive(Serialize, Deserialize, Clone, Debug)]
        pub struct SampleData {
            nodes: Vec<Node>,
            edges: Vec<Edge>,
        }
        let samples = [
            "nodes_10_full_mesh.json",
            "nodes_100_full_mesh.json",
            "nodes_1000_random.json",
            "nodes_2000_random.json",
            "nodes_5000_random.json",
            "nodes_10000_random.json",
            "nodes_50000_random.json",
            "nodes_100000_random.json",
            "telco_sample.json",
        ];
        let mut benchmark: Vec<String> = Vec::new();
        for total_threads in [16] {
            let thread_pool = ThreadPoolBuilder::new()
                .num_threads(total_threads)
                .build()
                .unwrap();
            thread_pool.install(async || {
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
                    let mut layout = GpuConcentric::new(GpuConcentric {
                        nodes: sample_data_1.nodes,
                        edges: sample_data_1.edges,
                        default_cx: Some(0.0),
                        default_cy: Some(0.0),
                        ..Default::default()
                    });
                    let data_prepare_timer = tokio::time::Instant::now();
                    let result = layout.get().await;
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
                    let writer = std::fs::File::options()
                        .truncate(true)
                        .create(true)
                        .write(true)
                        .open(format!("storage/calculation-gpu-{}", sample_file))
                        .unwrap();
                   serde_json::to_writer_pretty(writer, &layout).unwrap();
                    let writer = std::fs::File::options()
                        .truncate(true)
                        .create(true)
                        .write(true)
                        .open(format!("storage/output-gpu-{}", sample_file))
                        .unwrap();
                    serde_json::to_writer_pretty(writer, &data).unwrap();
                    println!("Data Prepare {}: {}s, {}ms",sample_file,data_prepare_timer.elapsed().as_secs(),data_prepare_timer.elapsed().as_millis());
                }

                benchmark.push(table.to_string());
            }).await;
        }
        let file_path = format!("storage/gpu-benchmark-{}", Local::now().timestamp());
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
