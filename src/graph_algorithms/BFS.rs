use crate::data_structures::custom_queue::CustomQueue;
use crate::graph_algorithms::graph_structures::edge::{Edge, UnweightedEdge};
use crate::graph_algorithms::graph_structures::graph::Graph;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct BFS<V>
where
    V: Hash + Eq,
{
    explored: HashSet<V>,
    frontier: CustomQueue<V>,
    predecessors: HashMap<V, V>,
    distances: HashMap<V, usize>,
}

impl<V> BFS<V>
where
    V: Hash + Eq + Clone,
{
    pub fn new() -> BFS<V> {
        BFS::<V> {
            explored: HashSet::new(),
            frontier: CustomQueue::new(),
            predecessors: HashMap::new(),
            distances: HashMap::new(),
        }
    }

    pub fn bfs(&mut self, graph: &Graph<V, UnweightedEdge<V>>, start: V, target: V) -> HashMap<V, V> {
        self.predecessors.insert(start.clone(), start.clone());
        self.explored.insert(start.clone());
        self.frontier.enqueue(start.clone());
        self.distances.insert(start.clone(), 0);

        while let Some(v) = self.frontier.dequeue() {
            if v == target {
                return self.predecessors.clone();
            }

            for neighbor in graph.get_adjacent(&v).unwrap_or(&Vec::new()) {
                let neighbor = neighbor.destination();
                if self.explored.contains(neighbor) { continue; }
                self.frontier.enqueue(neighbor.clone());
                self.explored.insert(neighbor.clone());
                self.distances.insert(neighbor.clone(), self.distances[&v] + 1);
                self.predecessors.insert(neighbor.clone(), v.clone());
            }
        }

        self.predecessors.clone()
    }

    pub fn reconstruct_path(&self, start: &V, target: &V) -> Vec<V> {
        let mut path: Vec<V> = Vec::new();
        let mut current = target.clone();

        while current != *start {
            path.push(current.clone());
            current = self.predecessors[&current].clone();
        }
        path.push(start.clone());

        path.reverse();
        path
    }
}
