use crate::graph_algorithms::BFS::BFS;
use crate::graph_algorithms::graph_structures::edge::{Edge, UnweightedEdge};
use crate::graph_algorithms::graph_structures::graph::Graph;

mod dynamic_programming;
mod pretty_print;
mod graph_algorithms;
mod data_structures;

fn main() {
    let mut graph: Graph<usize, UnweightedEdge<usize>> = Graph::with_capacity(5);
    graph.add_undirected_edge(UnweightedEdge::new(1, 2));
    graph.add_undirected_edge(UnweightedEdge::new(1, 4));
    graph.add_undirected_edge(UnweightedEdge::new(4, 5));
    graph.add_undirected_edge(UnweightedEdge::new(4, 3));
    graph.add_undirected_edge(UnweightedEdge::new(2, 4));
    graph.add_undirected_edge(UnweightedEdge::new(3, 5));

    let mut bfs: BFS<usize> = BFS::new();
    bfs.bfs(&graph, 1, 5);
    println!("{:?}", bfs.reconstruct_path(&1, &3));
}

