use itertools::Itertools;

pub fn generate_permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    items.iter().cloned().permutations(items.len()).collect()
}
