use crate::model::CityGraph;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

pub fn generate_permutations(items: Vec<usize>) -> Vec<Vec<usize>> {
    let mut results = Vec::new();
    let mut current = items.clone();
    permute(&mut current, 0, &mut results);
    results
}

fn permute(arr: &mut Vec<usize>, start: usize, results: &mut Vec<Vec<usize>>) {
    if start >= arr.len() {
        results.push(arr.clone());
        return;
    }

    for i in start..arr.len() {
        arr.swap(start, i);
        permute(arr, start + 1, results);
        arr.swap(start, i);
    }
}

fn bfs_distance(graph: &CityGraph, start: &str, goal: &str) -> Option<u32> {
    use std::collections::{HashMap, VecDeque};

    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    visited.insert(start.to_string(), 0);
    queue.push_back(start.to_string());

    while let Some(current) = queue.pop_front() {
        let current_dist = visited[&current];
        if let Some(neighbors) = graph.adjacency_list.get(&current) {
            for (neighbor, &weight) in neighbors {
                let next_dist = current_dist + weight;
                if !visited.contains_key(neighbor) || visited[neighbor] > next_dist {
                    visited.insert(neighbor.clone(), next_dist);
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    visited.get(goal).copied()
}

pub fn build_distance_matrix(
    graph: &CityGraph,
    cities: &Vec<String>,
) -> (Vec<Vec<u32>>, HashMap<String, usize>) {
    let mut index_map = HashMap::new();
    for (i, city) in cities.iter().enumerate() {
        index_map.insert(city.clone(), i);
    }

    let n = cities.len();
    let mut matrix = vec![vec![u32::MAX; n]; n];

    for (i, from_city) in cities.iter().enumerate() {
        for (j, to_city) in cities.iter().enumerate() {
            if from_city == to_city {
                matrix[i][j] = 0;
            } else if let Some(dist) = bfs_distance(graph, from_city, to_city) {
                matrix[i][j] = dist;
            }
        }
    }

    (matrix, index_map)
}

pub fn bf(
    permutations: Vec<Vec<String>>,
    dist_matrix: &Vec<Vec<u32>>,
    index_map: &HashMap<String, usize>,
) -> (Vec<String>, u32) {
    let mut best_score = u32::MAX;
    let mut best_path = Vec::new();

    for path in permutations {
        let score = score_path(&path, dist_matrix, index_map);
        if score < best_score {
            best_score = score;
            best_path = path;
        }
    }

    (best_path, best_score)
}

pub fn parallel_bf(
    permutations: Vec<Vec<String>>,
    dist_matrix: &Vec<Vec<u32>>,
    index_map: &HashMap<String, usize>,
) -> (Vec<String>, u32) {
    let dist_matrix = Arc::new(dist_matrix.clone());
    let index_map = Arc::new(index_map.clone());

    permutations
        .into_par_iter()
        .map(|path| {
            let score = score_path(&path, &dist_matrix, &index_map);
            (path, score)
        })
        .reduce(
            || (Vec::new(), u32::MAX),
            |best, current| if current.1 < best.1 { current } else { best },
        )
}

fn score_path(
    path: &Vec<String>,
    dist_matrix: &Vec<Vec<u32>>,
    index_map: &HashMap<String, usize>,
) -> u32 {
    let mut total = 0;
    for window in path.windows(2) {
        let from = index_map[&window[0]];
        let to = index_map[&window[1]];
        let dist = dist_matrix[from][to];
        if dist == u32::MAX {
            return u32::MAX;
        }
        total += dist;
    }
    total
}

pub fn index_perms_to_city_perms(
    index_perms: Vec<Vec<usize>>,
    cities: &Vec<String>,
) -> Vec<Vec<String>> {
    index_perms
        .into_iter()
        .map(|perm| perm.into_iter().map(|i| cities[i].clone()).collect())
        .collect()
}
