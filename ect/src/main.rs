mod graph;
mod model;
mod mst;
mod tsp;
mod utils;

use crate::model::{Args, CityGraph};
use crate::mst::build_mst;
use crate::tsp::{
    build_distance_matrix, find_best_path, generate_permutations, index_perms_to_city_perms,
};
use crate::utils::{print_distance_matrix, read_dataset_file, print_permutations};
use clap::Parser;

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

fn best_path(graph: &CityGraph, cities: Vec<String>) -> Vec<String> {
    let index_perms = generate_permutations((0..cities.len()).collect());
    // print_permutations(index_perms.clone(), cities.clone());
    let city_perms = index_perms_to_city_perms(index_perms, &cities);
    println!("Cities: {:?}", cities);    
    println!("Generated {} city permutations", city_perms.len());
    let (dist_matrix, index_map) = build_distance_matrix(&graph, &cities);
    print_distance_matrix(&dist_matrix, &cities);

    let (best_path, best_score) = find_best_path(city_perms, &dist_matrix, &index_map);
    println!("Best path: {:?} with score: {}", best_path, best_score);

    best_path
}

fn common_cases(graph: &CityGraph) {
    //-> Vec<String>
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

    let n_12_cities: Vec<String> = vec![
        "Barcelona".to_string(),
        "Paris".to_string(),
        "Madrid".to_string(),
        "London".to_string(),
        "Prague".to_string(),
        "Frankfurt".to_string(),
        "Zurich".to_string(),
        "Lyon".to_string(),
        "Rome".to_string(),
        "Berlin".to_string(),
        "Vienna".to_string(),
        "Amsterdam".to_string(),
    ];

    let n_16_cities: Vec<String> = vec![
        "Barcelona".to_string(),
        "Paris".to_string(),
        "Madrid".to_string(),
        "London".to_string(),
        "Prague".to_string(),
        "Frankfurt".to_string(),
        "Zurich".to_string(),
        "Lyon".to_string(),
        "Rome".to_string(),
        "Berlin".to_string(),
        "Vienna".to_string(),
        "Amsterdam".to_string(),
        "Budapest".to_string(),
        "Munich".to_string(),
        "Geneva".to_string(),
        "Brussels".to_string(),
    ];

    let n_20_cities: Vec<String> = vec![
        "Barcelona".to_string(),
        "Paris".to_string(),
        "Madrid".to_string(),
        "London".to_string(),
        "Prague".to_string(),
        "Frankfurt".to_string(),
        "Zurich".to_string(),
        "Lyon".to_string(),
        "Rome".to_string(),
        "Berlin".to_string(),
        "Vienna".to_string(),
        "Amsterdam".to_string(),
        "Budapest".to_string(),
        "Munich".to_string(),
        "Geneva".to_string(),
        "Brussels".to_string(),
        "Zagreb".to_string(),
        "Florence".to_string(),
        "Venice".to_string(),
        "Milan".to_string(),
    ];

    // best_path(graph, n_8_cities.clone());
    best_path(graph, n_12_cities.clone());
    // best_path(graph, n_16_cities.clone());
    // best_path(graph, n_20_cities.clone());
}

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
        model::Task::FindBestPath => {
            let graph =
                CityGraph::load_graph_from_file(&subgraph_path).expect("Failed to load graph");

            common_cases(&graph);

            println!("Measuring time... (not implemented)");
            // Implement your timing logic here
        }
    }
}
