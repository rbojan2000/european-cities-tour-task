use std::collections::{VecDeque, HashSet};

fn find_all_paths(&self, start: &str, targets: &[String]) -> Vec<(Vec<String>, u32)> {
    let target_set: HashSet<_> = targets.iter().cloned().collect();
    let mut results = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back((vec![start.to_string()], 0u32));

    while let Some((path, dist)) = queue.pop_front() {
        let current = path.last().unwrap();

        // Check if all targets visited
        let visited_targets: HashSet<_> = path.iter().cloned().collect();
        if target_set.is_subset(&visited_targets) {
            results.push((path.clone(), dist));
            continue;
        }

        if let Some(neighbors) = self.adjacency_list.get(current) {
            for (neighbor, &d) in neighbors {
                if !path.contains(neighbor) { // avoid cycles
                    let mut new_path = path.clone();
                    new_path.push(neighbor.clone());
                    queue.push_back((new_path, dist + d));
                }
            }
        }
    }

    results
}
