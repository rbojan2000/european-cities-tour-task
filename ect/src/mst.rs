use std::collections::HashMap;
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
