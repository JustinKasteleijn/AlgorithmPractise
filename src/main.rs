use crate::graph_algorithms::DFS::DFS;
use crate::graph_algorithms::graph_structures::edge::{Edge, UnweightedEdge};
use crate::graph_algorithms::graph_structures::graph::Graph;

mod dynamic_programming;
mod pretty_print;
mod graph_algorithms;
mod data_structures;

fn main() {
    let mut graph: Graph<&str, UnweightedEdge<&str>> = Graph::with_capacity(7);
    graph.add_undirected_edge(UnweightedEdge::new("A", "C"));
    graph.add_undirected_edge(UnweightedEdge::new("A", "B"));
    graph.add_undirected_edge(UnweightedEdge::new("B", "C"));
    graph.add_undirected_edge(UnweightedEdge::new("B", "D"));
    graph.add_undirected_edge(UnweightedEdge::new("C", "E"));
    graph.add_undirected_edge(UnweightedEdge::new("C", "D"));
    graph.add_undirected_edge(UnweightedEdge::new("D", "E"));
    graph.add_undirected_edge(UnweightedEdge::new("E", "G"));
    graph.add_undirected_edge(UnweightedEdge::new("E", "F"));

    let mut dfs: DFS<&str> = DFS::new();
    println!("{:?}", dfs.dfs_with_destination(&graph, &"A",&"E"));
}

