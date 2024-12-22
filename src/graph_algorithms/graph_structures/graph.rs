use crate::graph_algorithms::graph_structures::edge::Edge;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Graph<V, E>
where
    V: Eq + Hash,
    E: Edge<Value=V>,
{
    adj_list: HashMap<V, Vec<E>>,
}

impl<V, E> Graph<V, E>
where
    V: Eq + Hash + Clone,
    E: Edge<Value=V> + Clone,
{
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Graph {
            adj_list: HashMap::with_capacity(capacity),
        }
    }

    pub fn add_directed_edge(&mut self, edge: E) {
        self.adj_list.entry(edge.source().clone()).or_insert_with(Vec::new).push(edge);
    }

    pub fn add_undirected_edge(&mut self, edge: E) {
        let reversed = E::new(edge.destination().clone(), edge.source().clone());
        self.add_directed_edge(edge.clone());
        self.add_directed_edge(reversed);
    }

    pub fn get_adjacent(&self, vertex: &V) -> Option<&Vec<E>> {
        self.adj_list.get(vertex)
    }
}
