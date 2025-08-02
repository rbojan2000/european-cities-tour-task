use crate::model::{City, CityGraph, DatasetEdge};
use crate::mst::UnionFind;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

impl CityGraph {
    pub fn new() -> Self {
        Self {
            adjacency_list: HashMap::new(),
            cities: HashMap::new(),
        }
    }

    /// Builds a CityGraph from a slice of DatasetEdge
    pub fn build_graph_from_edges(edges: &[DatasetEdge]) -> Self {
        let mut graph = CityGraph::new();

        for edge in edges {
            // Add from city if missing
            if !graph.cities.contains_key(&edge.from_city) {
                graph.add_city(City {
                    name: edge.from_city.clone(),
                    country: edge.from_country.clone(),
                });
            }
            // Add to city if missing
            if !graph.cities.contains_key(&edge.to_city) {
                graph.add_city(City {
                    name: edge.to_city.clone(),
                    country: edge.to_country.clone(),
                });
            }
            // Add edge
            graph.add_edge(&edge.from_city, &edge.to_city, edge.distance);
            graph.add_edge(&edge.to_city, &edge.from_city, edge.distance);
        }

        graph
    }

    // Add a city to the graph
    pub fn add_city(&mut self, city: City) {
        self.cities.insert(city.name.clone(), city);
    }

    // Add an edge between two cities by name
    pub fn add_edge(&mut self, from: &str, to: &str, distance: u32) {
        self.adjacency_list
            .entry(from.to_string())
            .or_insert_with(HashMap::new)
            .insert(to.to_string(), distance);
    }

    // Save graph JSON to a file
    pub fn save_graph_to_file(graph: &CityGraph, path: &str) -> std::io::Result<()> {
        let json = graph.to_json().expect("Failed to serialize graph");
        fs::write(path, json)?;
        Ok(())
    }

    // Load graph from a JSON file
    pub fn load_graph_from_file(path: &str) -> std::io::Result<CityGraph> {
        let data = fs::read_to_string(path)?;
        let graph = CityGraph::from_json(&data).expect("Failed to deserialize graph");
        Ok(graph)
    }

    // Serialize graph to JSON string
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    // Deserialize graph from JSON string
    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}
