use crate::data_structures::custom_queue::CustomQueue;
use crate::graph_algorithms::graph_structures::edge::{Edge, UnweightedEdge};
use crate::graph_algorithms::graph_structures::graph::Graph;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct BFS<'a, V>
where
    V: Hash + Eq,
{
    explored: HashSet<&'a V>,
    frontier: CustomQueue<&'a V>,
    predecessors: HashMap<&'a V, (&'a V, usize)>,
}

impl<'a, V> BFS<'a, V>
where
    V: Hash + Eq + Clone,
{
    pub fn new() -> BFS<'a, V> {
        BFS::<'a, V> {
            explored: HashSet::new(),
            frontier: CustomQueue::new(),
            predecessors: HashMap::new(),
        }
    }

    pub fn bfs(&mut self, graph: &'a Graph<V, UnweightedEdge<V>>, start: &'a V, target: &'a V) -> Option<Vec<V>> {
        self.frontier.enqueue(start);
        self.explored.insert(start);
        self.predecessors.insert(start, (start, 0));

        while let Some(v) = self.frontier.dequeue() {
            if v == target {
                return Some(self.reconstruct_path(&start, &target));
            }

            if let Some(neighbors) = graph.get_adjacent(v) {
                for neighbor in neighbors {
                    let neighbor = neighbor.destination();
                    if self.explored.contains(neighbor) {continue;}
                    self.explored.insert(neighbor);
                    self.frontier.enqueue(neighbor);
                    self.predecessors.insert(neighbor, (v, self.predecessors[v].1 + 1));
                }
            }
        }
        None
    }

    fn reconstruct_path(&self, start: &'a V, target: &'a V) -> Vec<V> {
        let mut path: Vec<V> = Vec::new();
        let mut current = target;

        while current != start {
            path.push(current.clone());
            (current, _) = self.predecessors[&current].clone();
        }
        path.push(start.clone());

        path.reverse();
        path
    }
}
