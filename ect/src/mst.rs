use crate::model::CityGraph;
use std::collections::{HashMap, HashSet};

pub struct UnionFind {
    parent: HashMap<String, String>,
}

impl UnionFind {
    pub fn new(elements: &[String]) -> Self {
        let mut parent = HashMap::new();
        for e in elements {
            parent.insert(e.clone(), e.clone());
        }
        Self { parent }
    }

    pub fn find(&mut self, x: &str) -> String {
        let parent_x = self.parent.get(x).unwrap().clone();
        if &parent_x != x {
            let root = self.find(&parent_x);
            self.parent.insert(x.to_string(), root.clone());
            root
        } else {
            parent_x
        }
    }

    pub fn union(&mut self, a: &str, b: &str) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a == root_b {
            false
        } else {
            self.parent.insert(root_a, root_b);
            true
        }
    }
}

pub fn build_mst(graph: &CityGraph) -> CityGraph {
    let mut edges = Vec::new();
    let mut seen = HashSet::new();

    for (from, neighbors) in &graph.adjacency_list {
        for (to, &distance) in neighbors {
            let key = if from < to {
                (from.clone(), to.clone())
            } else {
                (to.clone(), from.clone())
            };
            if !seen.contains(&key) {
                seen.insert(key.clone());
                edges.push((distance, key.0, key.1));
            }
        }
    }

    edges.sort_by_key(|&(distance, _, _)| distance);

    let city_names: Vec<String> = graph.cities.keys().cloned().collect();
    let mut uf = UnionFind::new(&city_names);
    let mut mst = CityGraph::new();
    for city in graph.cities.values() {
        mst.add_city(city.clone());
    }

    for (distance, from, to) in edges {
        if uf.union(&from, &to) {
            mst.add_edge(&from, &to, distance);
            mst.add_edge(&to, &from, distance);
        }
    }

    mst
}
