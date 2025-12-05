pub mod cpu;
pub mod entities;
pub mod gpu;
pub mod timer;
pub use timer::Timer;
pub mod concentric_layout;
pub use concentric_layout::{ComputingConfig, ConcentricLayout};

#[cfg(test)]
pub mod test_concentric_layout {
    use super::*;
    use crate::entities::NodePositionData;
    use chrono::Local;
    use entities::{Edge, Node};
    use serde::{Deserialize, Serialize};
    use std::{fs::create_dir_all, io::Write, path::Path};
    use tabular::{Row, Table};

    fn get_sample_data_files() -> Vec<String> {
        [
            "nodes_10_full_mesh.json",
            "nodes_100_full_mesh.json",
            "nodes_1000_random.json",
            "nodes_2000_random.json",
            "nodes_5000_random.json",
            "nodes_10000_random.json",
            "nodes_50000_random.json",
            "nodes_100000_random.json",
            "telco_sample.json",
        ]
        .iter()
        .map(|value| value.to_string())
        .collect()
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct SampleData {
        nodes: Vec<Node>,
        edges: Vec<Edge>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct TestResult {
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        positions: Vec<NodePositionData>,
        timer: Timer,
    }
    fn get_sample_datasets(file_path: &str) -> SampleData {
        let reader = std::fs::File::options()
            .read(true)
            .open(format!("storage/sample-data/{}", file_path))
            .unwrap();
        let sample_data = serde_json::from_reader::<_, SampleData>(reader).unwrap();
        sample_data
    }
    fn write_benchmark(computing_kind: &str, benchmark: String) {
        create_dir_all(&format!("storage/benchmark")).unwrap();
        let file_path = format!(
            "storage/benchmark/{}-{}.txt",
            computing_kind,
            Local::now().timestamp()
        );
        let file_path = Path::new(&file_path);
        let mut file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)
            .unwrap();
        file.write_all(benchmark.as_bytes()).unwrap();
    }
    fn write_result(
        computing_kind: &str,
        file_name: &str,
        nodes: &Vec<Node>,
        edges: &Vec<Edge>,
        timer: &Timer,
        data: &Vec<NodePositionData>,
    ) {
        create_dir_all(format!("storage/output/{}", computing_kind)).unwrap();
        let writer = std::fs::File::options()
            .truncate(true)
            .create(true)
            .write(true)
            .open(format!("storage/output/{}/{}", computing_kind, file_name))
            .unwrap();
        serde_json::to_writer_pretty(
            writer,
            &TestResult {
                nodes: nodes.to_owned(),
                edges: edges.to_owned(),
                positions: data.to_owned(),
                timer: timer.to_owned(),
            },
        )
        .unwrap();
    }
    #[tokio::test]
    async fn test_cpu_based() {
        let mut table = Table::new(
            "| {:^} threads | {:<} | {:^} nodes | {:^} edges | {:^}s | {:^}ms | {:^}us |",
        );
        for num_threads in [
            //2, 4, 8, 16, 32
            16,
        ] {
            for sample_file in get_sample_data_files().iter() {
                let sample_data = get_sample_datasets(sample_file);
                let mut layout = ConcentricLayout::new(
                    &ComputingConfig::Cpu(num_threads as usize),
                    &sample_data.nodes,
                    &sample_data.edges,
                    &Some(0.0),
                    &Some(0.0),
                );
                let result = layout.execute().await;
                assert!(result.is_ok(), "{:#?}", result.err());
                assert!(layout.timer.is_some(), "Timer not found");
                let data = result.unwrap();
                let timer = layout.timer.unwrap();
                assert_eq!(sample_data.nodes.len(), data.len());
                write_result(
                    "cpu",
                    sample_file,
                    &sample_data.nodes,
                    &sample_data.edges,
                    &timer,
                    &data,
                );
                table = table.with_row(
                    Row::new()
                        .with_cell(num_threads)
                        .with_cell(sample_file)
                        .with_cell(layout.nodes.len())
                        .with_cell(layout.edges.len())
                        .with_cell(timer.seconds.unwrap().to_string())
                        .with_cell(timer.millis.unwrap().to_string())
                        .with_cell(timer.micros.unwrap().to_string()),
                );
            }
        }
        write_benchmark("cpu", table.to_string());
    }

    #[tokio::test]
    async fn test_gpu_based() {
        let mut table = Table::new("| {:<} | {:^} nodes | {:^} edges | {:^}s | {:^}ms | {:^}us |");
        for sample_file in get_sample_data_files().iter() {
            let sample_data = get_sample_datasets(sample_file);
            let mut layout = ConcentricLayout::new(
                &ComputingConfig::Gpu,
                &sample_data.nodes,
                &sample_data.edges,
                &Some(0.0),
                &Some(0.0),
            );
            let result = layout.execute().await;
            assert!(result.is_ok(), "{:#?}", result.err());
            assert!(layout.timer.is_some(), "Timer not found");
            let data = result.unwrap();
            let timer = layout.timer.unwrap();
            assert_eq!(sample_data.nodes.len(), data.len());
            write_result(
                "gpu",
                sample_file,
                &sample_data.nodes,
                &sample_data.edges,
                &timer,
                &data,
            );
            table = table.with_row(
                Row::new()
                    .with_cell(sample_file)
                    .with_cell(layout.nodes.len())
                    .with_cell(layout.edges.len())
                    .with_cell(timer.seconds.unwrap().to_string())
                    .with_cell(timer.millis.unwrap().to_string())
                    .with_cell(timer.micros.unwrap().to_string()),
            );
        }
        write_benchmark("gpu", table.to_string());
    }
}
