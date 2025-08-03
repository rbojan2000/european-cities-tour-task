mod graph;
mod model;
mod mst;
mod tsp;
mod utils;

use crate::model::{Algorithm, Args, CityGraph};
use crate::mst::build_mst;
use crate::tsp::{
    bf, build_distance_matrix, generate_permutations, index_perms_to_city_perms, parallel_bf,
};
use crate::utils::{print_distance_matrix, read_dataset_file};
use clap::Parser;
use std::collections::HashMap;
use std::time::Instant;

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

fn prepare_dist_matrix(
    graph: &CityGraph,
    cities: Vec<String>,
) -> (Vec<Vec<String>>, Vec<Vec<u32>>, HashMap<String, usize>) {
    let index_perms = generate_permutations((0..cities.len()).collect());
    let city_perms = index_perms_to_city_perms(index_perms, &cities);
    let (dist_matrix, index_map) = build_distance_matrix(&graph, &cities);
    println!("Cities len: {:?}", cities.len());
    println!("Cities: {:?}", cities);
    println!("Generated {} city permutations", city_perms.len());
    print_distance_matrix(&dist_matrix, &cities);

    // print_permutations(index_perms.clone(), cities.clone());

    (city_perms, dist_matrix, index_map)
}

fn common_cases(graph: &CityGraph, num_threads: usize, alg: Algorithm) {
    let n_8_cities: Vec<String> = vec![
        "Barcelona".to_string(),
        "Paris".to_string(),
        "Madrid".to_string(),
        "London".to_string(),
        "Prague".to_string(),
        "Frankfurt".to_string(),
        "Zurich".to_string(),
        "Lyon".to_string(),
    ];

    let n_10_cities: Vec<String> = vec![
        "Barcelona".to_string(),
        "Paris".to_string(),
        "Madrid".to_string(),
        "London".to_string(),
        "Prague".to_string(),
        "Frankfurt".to_string(),
        "Zurich".to_string(),
        "Lyon".to_string(),
        "Vienna".to_string(),
        "Amsterdam".to_string(),
    ];

    match alg {
        Algorithm::Serial => {
            println!("\n[Serial] Common case 1");
            let start = Instant::now();

            let (city_perms, dist_matrix, index_map) =
                prepare_dist_matrix(graph, n_8_cities.clone());
            let (best_path, best_score) = bf(city_perms, &dist_matrix, &index_map);

            println!("BF Path: {:?} with score: {}", best_path, best_score);
            println!("Time for 8 cities: {:.2?}", start.elapsed());

            println!("\n[Serial] Common case 2");
            let start = Instant::now();

            let (city_perms, dist_matrix, index_map) =
                prepare_dist_matrix(graph, n_10_cities.clone());
            let (best_path, best_score) = bf(city_perms, &dist_matrix, &index_map);
            println!("Path: {:?} with score: {}", best_path, best_score);
            println!("Time for 10 cities: {:.2?}", start.elapsed());
        }
        Algorithm::Parallel => {
            // Podešavanje broja niti u rayon thread pool-u (opciono)
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build_global()
                .unwrap();

            println!("\n[Parallel] Common case 1");
            let start = Instant::now();
            let (city_perms, dist_matrix, index_map) =
                prepare_dist_matrix(graph, n_8_cities.clone());

            let (best_path, best_score) = parallel_bf(city_perms, &dist_matrix, &index_map);

            println!("BF Path: {:?} with score: {}", best_path, best_score);
            println!("Time for 8 cities: {:.2?}", start.elapsed());

            println!("\n[Parallel] Common case 2");
            let start = Instant::now();
            let (city_perms, dist_matrix, index_map) =
                prepare_dist_matrix(graph, n_10_cities.clone());

            let (best_path, best_score) = parallel_bf(city_perms, &dist_matrix, &index_map);

            println!(
                "BF Path: {:?}, num threads: {},  score: {}",
                best_path, best_score, num_threads
            );
            println!("Time for 10 cities: {:.2?}", start.elapsed());
        }
    }
}

fn main() {
    let input_dataset_path = format!("{}input.txt", DATASET_PATH);
    let graph_path = format!("{}graph.json", DATASET_PATH);
    let subgraph_path = format!("{}mst_graph.json", DATASET_PATH);

    let args = Args::parse();

    let algorithm = args.algorithm.unwrap_or(model::Algorithm::Serial);
    let strategy = args.strategy.unwrap_or(model::Strategy::Bf);

    let num_threads = args.num_threads.unwrap_or(1);

    println!("\n=== CLI Parameters ===");
    println!("Task           : {:?}", args.task);
    println!("Execution Mode : {:?}", algorithm);
    println!("Strategy       : {:?}", strategy);
    println!("Threads        : {}", num_threads);
    println!("======================\n");

    match args.task {
        model::Task::BuildGraph => {
            build_graph(&input_dataset_path, &graph_path);
        }
        model::Task::BuildMst => {
            build_mst_subgraph(&graph_path, &subgraph_path);
        }
        model::Task::FindBestPath => {
            let graph =
                CityGraph::load_graph_from_file(&subgraph_path).expect("Failed to load graph");

            common_cases(&graph, num_threads, algorithm);
        }
    }
}
