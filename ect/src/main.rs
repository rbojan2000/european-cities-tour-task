mod graph;
mod model;
mod mst;
mod tsp;
mod utils;

use crate::model::{Args, CityGraph};
use crate::mst::build_mst;
use crate::utils::read_dataset_file;
use clap::Parser;
use std::time::Instant;
use tsp::generate_permutations;

const DATASET_PATH: &str = "../dataset/";

fn build_graph(input_dataset_path: &str, graph_path: &str) {
    let edges = read_dataset_file(input_dataset_path).expect("Failed to read dataset");
    println!("Loaded {} edges from dataset", edges.len());
    let graph = CityGraph::build_graph_from_edges(&edges);
    CityGraph::save_graph_to_file(&graph, graph_path).expect("Failed to save graph");
    println!("Saved graph to path: {}", graph_path);
}

fn build_mst_subgraph(graph_path: &str, subgraph_path: &str) {
    let loaded_graph = CityGraph::load_graph_from_file(graph_path).expect("Failed to load graph");
    let mst_graph = build_mst(&loaded_graph);
    CityGraph::save_graph_to_file(&mst_graph, subgraph_path).unwrap();
}

// fn extract_best_path(graph: &CityGraph, cities: Vec<String>) -> Vec<String> {
//     let permutations = generate_permutations(cities);
// }

// fn run_default_cases() {
//     let start = Instant::now();

//     let path = extract_path(&graph, &cities);

//     let duration = start.elapsed();
// }

fn main() {
    let input_dataset_path = format!("{}input.txt", DATASET_PATH);
    let graph_path = format!("{}graph.json", DATASET_PATH);
    let subgraph_path = format!("{}mst_graph.json", DATASET_PATH);

    let args = Args::parse();

    println!("Task: {:?}", args.task);
    println!("Algorithm: {:?}", args.algorithm);
    if let Some(cities) = &args.cities {
        println!("Cities: {:?}", cities);
    } else {
        println!("No cities specified");
    }

    match args.task {
        model::Task::BuildGraph => {
            build_graph(&input_dataset_path, &graph_path);
        }
        model::Task::BuildMst => {
            build_mst_subgraph(&graph_path, &subgraph_path);
        }
        model::Task::MeasureTime => {
            println!("Measuring time... (not implemented)");
            // Implement your timing logic here
        }
    }
}
