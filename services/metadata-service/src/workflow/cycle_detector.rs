use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use shared::CoreResult;

/// Detect cycles in workflow graph using DFS
pub struct CycleDetector {
    graph: HashMap<Uuid, Vec<Uuid>>,
}

impl CycleDetector {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, from: Uuid, to: Uuid) {
        self.graph.entry(from).or_insert_with(Vec::new).push(to);
    }

    /// Detect if there are any cycles in the graph
    /// Returns true if cycle detected, false otherwise
    pub fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for &node in self.graph.keys() {
            if !visited.contains(&node) {
                if self.dfs_cycle(node, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }

        false
    }

    /// DFS to detect cycle
    fn dfs_cycle(
        &self,
        node: Uuid,
        visited: &mut HashSet<Uuid>,
        rec_stack: &mut HashSet<Uuid>,
    ) -> bool {
        visited.insert(node);
        rec_stack.insert(node);

        if let Some(neighbors) = self.graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    if self.dfs_cycle(neighbor, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&neighbor) {
                    // Found cycle
                    return true;
                }
            }
        }

        rec_stack.remove(&node);
        false
    }

    /// Find all cycles (for detailed reporting)
    pub fn find_cycles(&self) -> Vec<Vec<Uuid>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = Vec::new();

        for &node in self.graph.keys() {
            if !visited.contains(&node) {
                self.dfs_find_cycles(node, &mut visited, &mut rec_stack, &mut cycles);
            }
        }

        cycles
    }

    fn dfs_find_cycles(
        &self,
        node: Uuid,
        visited: &mut HashSet<Uuid>,
        rec_stack: &mut Vec<Uuid>,
        cycles: &mut Vec<Vec<Uuid>>,
    ) {
        visited.insert(node);
        rec_stack.push(node);

        if let Some(neighbors) = self.graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    self.dfs_find_cycles(neighbor, visited, rec_stack, cycles);
                } else if let Some(pos) = rec_stack.iter().position(|&n| n == neighbor) {
                    // Found cycle - extract it
                    let cycle = rec_stack[pos..].to_vec();
                    cycles.push(cycle);
                }
            }
        }

        rec_stack.pop();
    }

    /// Check if graph is acyclic (DAG)
    pub fn is_dag(&self) -> bool {
        !self.has_cycle()
    }

    /// Topological sort (only works for DAG)
    pub fn topological_sort(&self) -> Option<Vec<Uuid>> {
        if self.has_cycle() {
            return None;
        }

        let mut in_degree: HashMap<Uuid, usize> = HashMap::new();
        let mut result = Vec::new();
        let mut queue = Vec::new();

        // Calculate in-degrees
        for node in self.graph.keys() {
            in_degree.entry(*node).or_insert(0);
        }

        for neighbors in self.graph.values() {
            for &neighbor in neighbors {
                *in_degree.entry(neighbor).or_insert(0) += 1;
            }
        }

        // Find nodes with in-degree 0
        for (&node, &degree) in &in_degree {
            if degree == 0 {
                queue.push(node);
            }
        }

        // Process queue
        while let Some(node) = queue.pop() {
            result.push(node);

            if let Some(neighbors) = self.graph.get(&node) {
                for &neighbor in neighbors {
                    if let Some(degree) = in_degree.get_mut(&neighbor) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(neighbor);
                        }
                    }
                }
            }
        }

        if result.len() == in_degree.len() {
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_cycle() {
        let mut detector = CycleDetector::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        // Linear: n1 -> n2 -> n3
        detector.add_edge(n1, n2);
        detector.add_edge(n2, n3);

        assert!(!detector.has_cycle());
        assert!(detector.is_dag());
    }

    #[test]
    fn test_simple_cycle() {
        let mut detector = CycleDetector::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();

        // Cycle: n1 -> n2 -> n1
        detector.add_edge(n1, n2);
        detector.add_edge(n2, n1);

        assert!(detector.has_cycle());
        assert!(!detector.is_dag());
    }

    #[test]
    fn test_self_loop() {
        let mut detector = CycleDetector::new();
        let n1 = Uuid::new_v4();

        // Self loop: n1 -> n1
        detector.add_edge(n1, n1);

        assert!(detector.has_cycle());
    }

    #[test]
    fn test_complex_cycle() {
        let mut detector = CycleDetector::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();
        let n4 = Uuid::new_v4();

        // n1 -> n2 -> n3 -> n4 -> n2 (cycle)
        detector.add_edge(n1, n2);
        detector.add_edge(n2, n3);
        detector.add_edge(n3, n4);
        detector.add_edge(n4, n2);

        assert!(detector.has_cycle());
    }

    #[test]
    fn test_topological_sort() {
        let mut detector = CycleDetector::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        detector.add_edge(n1, n2);
        detector.add_edge(n2, n3);

        let sorted = detector.topological_sort();
        assert!(sorted.is_some());
    }
}
